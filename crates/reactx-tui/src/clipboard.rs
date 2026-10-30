//! Clipboard support through OSC 52.
//!
//! Copying writes the text to the terminal emulator's clipboard, which works
//! over SSH as well. Terminals that ignore OSC 52 simply do nothing. tmux
//! users need `set-clipboard on`.

use std::io::{IsTerminal, Write};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

/// Copy `text` to the terminal clipboard. Returns whether a terminal received
/// the sequence (false when stdout is not a terminal, e.g. in tests).
pub fn copy(text: &str) -> bool {
    if text.is_empty() || !std::io::stdout().is_terminal() {
        return false;
    }
    let encoded = STANDARD.encode(text.as_bytes());
    let mut out = std::io::stdout().lock();
    write!(out, "\x1b]52;c;{encoded}\x07").is_ok() && out.flush().is_ok()
}
