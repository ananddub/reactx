//! Terminal lifecycle, the async event loop, and `launch`.

use std::io;
use std::time::Duration;

use crossterm::cursor::Show;
use crossterm::event::{
    DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
    EnableFocusChange, EnableMouseCapture, Event as CrosstermEvent, EventStream,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use dioxus_core::{Element, VirtualDom};
use futures_util::future::poll_immediate;
use futures_util::{Stream, StreamExt};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use reactx_core::TuiEvent;
use tokio::task::LocalSet;

mod translate;

use crate::events::{self, Flow};
use crate::TuiRenderer;

/// Run a ReactX application until the user quits.
///
/// The VirtualDom is `!Send`, so the terminal loop runs on its own thread with
/// a current-thread runtime. This keeps `launch` usable from `#[tokio::main]`.
pub async fn launch(app: fn() -> Element) {
    let (tx, rx) = tokio::sync::oneshot::channel::<io::Result<()>>();

    let spawned = std::thread::Builder::new()
        .name("reactx-tui".to_string())
        .spawn(move || {
            let result = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => {
                    let local = LocalSet::new();
                    local.block_on(&runtime, async move { run(app).await })
                }
                Err(err) => Err(err),
            };
            let _ = tx.send(result);
        });

    if let Err(err) = spawned {
        eprintln!("reactx: failed to spawn terminal thread: {err}");
        return;
    }

    match rx.await {
        Ok(Ok(())) => {}
        Ok(Err(err)) => eprintln!("reactx: {err}"),
        Err(_) => {}
    }
}

/// Restore the terminal to a usable state. Safe to call at any time.
pub fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = execute!(
        io::stdout(),
        DisableMouseCapture,
        DisableBracketedPaste,
        DisableFocusChange,
        LeaveAlternateScreen,
        Show
    );
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            EnableMouseCapture,
            EnableBracketedPaste,
            EnableFocusChange,
            Show
        )?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        restore_terminal();
    }
}

async fn run(app: fn() -> Element) -> io::Result<()> {
    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut dom = VirtualDom::new(app);
    let mut renderer = TuiRenderer::new();
    dom.rebuild(&mut renderer);
    let size = terminal.size()?;
    crate::terminal::set_terminal_size(&dom, size.width, size.height);
    dom.render_immediate(&mut renderer);

    let mut stream = EventStream::new();

    loop {
        renderer.paint(&mut terminal)?;

        tokio::select! {
            biased;
            maybe_event = stream.next() => {
                let Some(Ok(event)) = maybe_event else {
                    break;
                };
                if !handle_batch(event, &mut stream, &dom, &mut renderer).await {
                    break;
                }
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

/// Upper bound on events drained before the next render, so a continuous
/// flood still yields frames.
const MAX_BATCH: usize = 256;

/// Frame interval used while a screen effect is running (~60 fps).
const EFFECT_FRAME: Duration = Duration::from_millis(16);

/// Handle the first event plus everything already queued before rendering.
///
/// Rapid input (mouse motion in particular) can queue hundreds of events;
/// handling the whole backlog before rendering keeps click latency stable,
/// and superseded pointer samples are dropped. Returns false on quit.
///
/// The drain polls [`poll_immediate`], never `now_or_never`: crossterm's
/// `EventStream` keeps a single waker slot, so polling it with a noop waker
/// would swallow the registration and leave the loop permanently asleep.
async fn handle_batch<S>(
    first: CrosstermEvent,
    stream: &mut S,
    dom: &VirtualDom,
    renderer: &mut TuiRenderer,
) -> bool
where
    S: Stream<Item = io::Result<CrosstermEvent>> + Unpin,
{
    let mut pending: Vec<TuiEvent> = Vec::new();
    if let Some(event) = translate::translate(first) {
        pending.push(event);
    }
    while pending.len() < MAX_BATCH {
        let Some(Some(Ok(raw))) = poll_immediate(stream.next()).await else {
            break;
        };
        if let Some(event) = translate::translate(raw) {
            pending.push(event);
        }
    }
    while !pending.is_empty() {
        let event = pending.remove(0);
        if matches!(event, TuiEvent::MouseMove { .. })
            && matches!(pending.first(), Some(TuiEvent::MouseMove { .. }))
        {
            continue;
        }
        if events::handle_event(dom, renderer, event) == Flow::Quit {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests;
