//! Terminal lifecycle, the async event loop, and `launch`.

use std::io;

use crossterm::cursor::Show;
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
    EventStream,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use dioxus_core::{Element, VirtualDom};
use futures_util::StreamExt;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
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
                if let Some(event) = translate::translate(event) {
                    if events::handle_event(&dom, &mut renderer, event) == Flow::Quit {
                        break;
                    }
                    dom.render_immediate(&mut renderer);
                }
            }
            _ = dom.wait_for_work() => {
                dom.render_immediate(&mut renderer);
            }
        }
    }

    Ok(())
}
