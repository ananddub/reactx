//! Ink-style `useStdin`, `useStdout` and `useStderr` hooks.

use std::cell::{Cell, RefCell};
use std::io;

use reactx_core::Line;

thread_local! {
    static RAW_MODE: Cell<bool> = const { Cell::new(false) };
    static STDERR: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// Access to the app's input stream.
#[derive(Clone, Copy, Debug, Default)]
pub struct StdinHandle;

impl StdinHandle {
    /// Whether raw mode can be toggled (only when stdout is a terminal).
    pub fn is_raw_mode_supported(&self) -> bool {
        crate::config::with(|config| config.interactive)
    }

    /// Whether stdin is currently in raw mode.
    pub fn is_raw_mode(&self) -> bool {
        RAW_MODE.with(Cell::get)
    }

    /// Enter or leave raw mode. Outside an interactive run this is a no-op.
    pub fn set_raw_mode(&self, enabled: bool) -> io::Result<()> {
        if !self.is_raw_mode_supported() {
            return Ok(());
        }
        if enabled {
            crossterm::terminal::enable_raw_mode()?;
        } else {
            crossterm::terminal::disable_raw_mode()?;
        }
        RAW_MODE.with(|slot| slot.set(enabled));
        Ok(())
    }
}

/// Access to the app's output stream.
#[derive(Clone, Copy, Debug, Default)]
pub struct StdoutHandle;

impl StdoutHandle {
    /// Queue `text` above the live view. Lines stay in the terminal's
    /// scrollback, like the `<Static>` widget.
    pub fn write(&self, text: impl AsRef<str>) {
        let lines: Vec<Line<'static>> = text
            .as_ref()
            .split('\n')
            .map(|line| Line::from(line.to_string()))
            .collect();
        reactx_core::static_out::append_lines(lines);
    }
}

/// Access to the app's error stream.
#[derive(Clone, Copy, Debug, Default)]
pub struct StderrHandle;

impl StderrHandle {
    /// Queue `text` for stderr. It is written once the terminal is restored, so
    /// it never corrupts the live view.
    pub fn write(&self, text: impl AsRef<str>) {
        let text = text.as_ref().to_string();
        STDERR.with(|lines| lines.borrow_mut().push(text));
    }
}

/// Access the app's input stream.
pub fn use_stdin() -> StdinHandle {
    StdinHandle
}

/// Access the app's output stream.
pub fn use_stdout() -> StdoutHandle {
    StdoutHandle
}

/// Access the app's error stream.
pub fn use_stderr() -> StderrHandle {
    StderrHandle
}

/// Record the raw-mode state the launcher established.
pub(crate) fn set_raw_mode(enabled: bool) {
    RAW_MODE.with(|slot| slot.set(enabled));
}

/// Write everything queued through [`StderrHandle::write`] to stderr.
pub(crate) fn flush() {
    STDERR.with(|lines| {
        for text in lines.borrow_mut().drain(..) {
            eprintln!("{text}");
        }
    });
}

pub(crate) fn clear() {
    STDERR.with(|lines| lines.borrow_mut().clear());
    RAW_MODE.with(|slot| slot.set(false));
}
