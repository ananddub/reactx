//! Queue of styled lines destined for the terminal scrollback.
//!
//! The `<Static>` widget appends here while rendering; the inline launcher
//! drains the queue before each frame.

use ratatui::text::Line;
use std::sync::Mutex;

static PENDING: Mutex<Vec<Line<'static>>> = Mutex::new(Vec::new());

/// Queue lines to be written above the live view on the next frame.
pub fn append_lines(lines: impl IntoIterator<Item = Line<'static>>) {
    if let Ok(mut pending) = PENDING.lock() {
        pending.extend(lines);
    }
}

/// Remove and return everything queued since the last call.
pub fn take_lines() -> Vec<Line<'static>> {
    PENDING
        .lock()
        .map(|mut pending| std::mem::take(&mut *pending))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_queue_and_drain() {
        let _ = take_lines();
        append_lines([Line::from("one"), Line::from("two")]);
        assert_eq!(take_lines().len(), 2);
        assert!(take_lines().is_empty());
    }
}
