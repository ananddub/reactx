//! Non-interactive fallback: render without a TTY and print the final frame.

use std::io;
use std::time::Instant;

use dioxus_core::{Element, VirtualDom};
use ratatui::backend::TestBackend;
use ratatui::Terminal;

use crate::config::{RenderMetrics, RuntimeConfig};
use crate::launch::Mode;
use crate::TuiRenderer;

pub(super) async fn run(
    app: fn() -> Element,
    _mode: Mode,
    config: &RuntimeConfig,
) -> io::Result<()> {
    let (width, height) = size();
    let mut terminal = Terminal::new(TestBackend::new(width, height))?;
    let mut dom = VirtualDom::new(app);
    let mut renderer = TuiRenderer::new();
    dom.rebuild(&mut renderer);
    crate::terminal::set_terminal_size(&dom, width, height);
    dom.render_immediate(&mut renderer);

    let mut last_frame: Option<Instant> = None;
    loop {
        let frame_start = Instant::now();
        renderer.paint(&mut terminal)?;
        crate::runtime::focus::publish(&dom, &renderer);
        crate::runtime::metrics::publish(&dom, &renderer);
        crate::app::flush_render_waiters();
        if let Some(callback) = &config.on_render {
            let frame_time = last_frame
                .map(|last| frame_start.duration_since(last))
                .unwrap_or_default();
            callback(RenderMetrics {
                render_time: frame_start.elapsed(),
                frame_time,
            });
        }
        last_frame = Some(frame_start);
        if crate::app::take_exit() {
            break;
        }
        dom.wait_for_work().await;
        dom.render_immediate(&mut renderer);
    }

    println!(
        "{}",
        crate::string::buffer_text(terminal.backend().buffer())
    );
    Ok(())
}

/// Dimensions for the headless viewport, honoring `$COLUMNS` / `$LINES`.
fn size() -> (u16, u16) {
    let read = |key: &str, fallback: u16| {
        std::env::var(key)
            .ok()
            .and_then(|value| value.parse().ok())
            .filter(|value| *value > 0)
            .unwrap_or(fallback)
    };
    (read("COLUMNS", 80), read("LINES", 24))
}
