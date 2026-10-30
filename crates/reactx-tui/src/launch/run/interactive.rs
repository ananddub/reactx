//! The interactive terminal loop: raw input, painted frames and effects.

use std::io;
use std::time::{Duration, Instant};

use crossterm::event::EventStream;
use crossterm::execute;
use crossterm::terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate};
use dioxus_core::{Element, VirtualDom};
use futures_util::StreamExt;
use ratatui::backend::CrosstermBackend;
use ratatui::{Terminal, TerminalOptions, Viewport};

use super::flush_static;
use crate::config::{RenderMetrics, RuntimeConfig};
use crate::launch::{handle_batch, Mode, EFFECT_FRAME};
use crate::TuiRenderer;

pub(super) async fn run(
    app: fn() -> Element,
    mode: Mode,
    config: &RuntimeConfig,
) -> io::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let viewport = match mode {
        Mode::Fullscreen => Viewport::Fullscreen,
        Mode::Inline(height) => Viewport::Inline(height),
    };
    let mut terminal = Terminal::with_options(backend, TerminalOptions { viewport })?;
    if matches!(mode, Mode::Fullscreen) {
        terminal.clear()?;
    }

    let mut dom = VirtualDom::new(app);
    let mut renderer = TuiRenderer::new();
    dom.rebuild(&mut renderer);

    let size = terminal.size()?;
    let mut reported = match mode {
        Mode::Fullscreen => (size.width, size.height),
        Mode::Inline(height) => (size.width, height),
    };
    crate::terminal::set_terminal_size(&dom, reported.0, reported.1);
    dom.render_immediate(&mut renderer);

    let mut stream = EventStream::new();
    let interval = config.frame_interval();
    let mut last_frame: Option<Instant> = None;

    loop {
        if crate::app::take_exit() {
            break;
        }
        pace(interval, last_frame).await;
        let frame_start = Instant::now();
        // DEC 2026: terminals apply the frame atomically, avoiding tearing
        // when the scrollback and the live view change together.
        execute!(io::stdout(), BeginSynchronizedUpdate)?;
        flush_static(&mut terminal, mode)?;
        let paint_start = Instant::now();
        renderer.paint(&mut terminal)?;
        let render_time = paint_start.elapsed();
        let _ = execute!(io::stdout(), EndSynchronizedUpdate);
        if crate::app::take_clear() {
            terminal.clear()?;
        }
        let painted = (renderer.area.width, renderer.area.height);
        if painted != reported {
            reported = painted;
            crate::terminal::set_terminal_size(&dom, painted.0, painted.1);
        }
        after_frame(
            &dom,
            &renderer,
            config,
            last_frame,
            frame_start,
            render_time,
        );
        last_frame = Some(frame_start);

        tokio::select! {
            biased;
            maybe_event = stream.next() => {
                let Some(Ok(event)) = maybe_event else {
                    break;
                };
                if !handle_batch(event, &mut stream, &dom, &mut renderer).await {
                    break;
                }
                crate::runtime::focus::drain(&dom, &mut renderer);
                dom.render_immediate(&mut renderer);
            }
            _ = dom.wait_for_work() => {
                dom.render_immediate(&mut renderer);
            }
            // Keep frames coming while an effect is running; screen effects
            // advance in `paint`, not in the VirtualDom's task queue.
            _ = tokio::time::sleep(EFFECT_FRAME), if renderer.effects_active() => {}
        }
    }

    Ok(())
}

/// Sleep until the frame budget allows the next paint.
async fn pace(interval: Option<Duration>, last_frame: Option<Instant>) {
    let (Some(interval), Some(last)) = (interval, last_frame) else {
        return;
    };
    let next = last + interval;
    let now = Instant::now();
    if next > now {
        tokio::time::sleep(next - now).await;
    }
}

fn after_frame(
    dom: &VirtualDom,
    renderer: &TuiRenderer,
    config: &RuntimeConfig,
    last_frame: Option<Instant>,
    frame_start: Instant,
    render_time: Duration,
) {
    crate::runtime::focus::publish(dom, renderer);
    crate::runtime::metrics::publish(dom, renderer);
    crate::app::flush_render_waiters();
    if let Some(callback) = &config.on_render {
        let frame_time = last_frame
            .map(|last| frame_start.duration_since(last))
            .unwrap_or(render_time);
        callback(RenderMetrics {
            render_time,
            frame_time,
        });
    }
}
