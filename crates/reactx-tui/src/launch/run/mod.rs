//! The frame loop shared by fullscreen and inline launch.

use std::io;

use dioxus_core::Element;
use ratatui::backend::Backend;
use ratatui::widgets::{Paragraph, Widget, Wrap};

use super::guard::TerminalGuard;
use super::Mode;
use crate::config::{self, LaunchOptions, RuntimeConfig};

mod headless;
mod interactive;

pub(super) async fn run(
    app: fn() -> Element,
    mode: Mode,
    options: LaunchOptions,
) -> io::Result<()> {
    let runtime = RuntimeConfig::resolve(&options);
    begin(&runtime);
    let result = if runtime.interactive {
        run_interactive(app, mode, &runtime).await
    } else {
        headless::run(app, mode, &runtime).await
    };
    finish();
    result
}

async fn run_interactive(
    app: fn() -> Element,
    mode: Mode,
    runtime: &RuntimeConfig,
) -> io::Result<()> {
    super::install_panic_hook();
    let guard = TerminalGuard::enter(mode, runtime.kitty_keyboard)?;
    let result = interactive::run(app, mode, runtime).await;
    drop(guard);
    result
}

fn begin(runtime: &RuntimeConfig) {
    config::install(runtime.clone());
    reactx_core::screen::set_enabled(Some(runtime.screen_reader));
    reactx_core::transform::clear();
    crate::runtime::clear();
    crate::runtime::streams::set_raw_mode(runtime.interactive);
    crate::app::reset();
}

fn finish() {
    crate::runtime::streams::flush();
    crate::app::flush_exit_waiters();
    crate::runtime::clear();
    reactx_core::screen::set_enabled(None);
    config::clear();
}

/// Write lines queued by `<Static>` above the live view. Fullscreen terminals
/// have no scrollback region to target, so their queue is just discarded to
/// keep it from growing.
pub(super) fn flush_static<T: Backend>(
    terminal: &mut ratatui::Terminal<T>,
    mode: Mode,
) -> io::Result<()> {
    let lines = reactx_core::static_out::take_lines();
    if lines.is_empty() || !matches!(mode, Mode::Inline(_)) {
        return Ok(());
    }
    let width = terminal.size()?.width.max(1);
    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    let height = u16::try_from(paragraph.line_count(width)).unwrap_or(u16::MAX);
    terminal.insert_before(height, |buf| paragraph.render(buf.area, buf))
}
