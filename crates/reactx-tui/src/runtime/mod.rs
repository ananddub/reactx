//! Ink-compatible runtime hooks: raw input, paste, focus, cursor, metrics and
//! the standard streams.

pub mod cursor;
pub mod focus;
pub mod input;
pub mod metrics;
pub mod screen;
pub mod streams;

pub use cursor::{use_cursor, CursorHandle};
pub use focus::{use_focus, use_focus_manager, FocusManagerHandle};
pub use input::{use_input, use_input_when, use_paste, use_paste_when};
pub use metrics::{measure_element, use_box_metrics, BoxMetrics};
pub use screen::use_is_screen_reader_enabled;
pub use streams::{use_stderr, use_stdin, use_stdout, StderrHandle, StdinHandle, StdoutHandle};

/// Reset every hook registry. Called when a run starts or ends.
pub(crate) fn clear() {
    input::clear();
    focus::clear();
    cursor::clear();
    streams::clear();
}
