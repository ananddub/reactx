//! Terminal lifecycle, the async event loop, and the launchers.

use std::io;
use std::time::Duration;

use crossterm::event::Event as CrosstermEvent;
use dioxus_core::{Element, VirtualDom};
use futures_util::future::poll_immediate;
use futures_util::{Stream, StreamExt};
use reactx_core::TuiEvent;
use tokio::task::LocalSet;

mod guard;
mod run;
mod translate;

pub use crate::config::LaunchOptions;
pub use guard::{install_panic_hook, restore_terminal};

use crate::events::{self, Flow};
use crate::TuiRenderer;

/// How a launcher claims the screen.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Mode {
    /// Alternate screen, covering the whole terminal.
    Fullscreen,
    /// A live region of `height` rows at the bottom of the screen, leaving the
    /// lines above it in the scrollback.
    Inline(u16),
}

/// Run a ReactX application fullscreen until the user quits.
///
/// The VirtualDom is `!Send`, so the terminal loop runs on its own thread with
/// a current-thread runtime. This keeps `launch` usable from `#[tokio::main]`.
pub async fn launch(app: fn() -> Element) {
    launch_with(app, LaunchOptions::new()).await;
}

/// Run a ReactX application fullscreen with custom [`LaunchOptions`].
pub async fn launch_with(app: fn() -> Element, options: LaunchOptions) {
    launch_mode(app, Mode::Fullscreen, options).await;
}

/// Run a ReactX application in an inline viewport of `height` rows.
///
/// Lines emitted by the `<Static>` widget are written above the live view and
/// stay in the terminal's scrollback after the app exits.
pub async fn launch_inline(app: fn() -> Element, height: u16) {
    launch_inline_with(app, height, LaunchOptions::new()).await;
}

/// Run a ReactX application inline with custom [`LaunchOptions`].
pub async fn launch_inline_with(app: fn() -> Element, height: u16, options: LaunchOptions) {
    launch_mode(app, Mode::Inline(height.max(1)), options).await;
}

async fn launch_mode(app: fn() -> Element, mode: Mode, options: LaunchOptions) {
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
                    local.block_on(&runtime, async move { run::run(app, mode, options).await })
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
