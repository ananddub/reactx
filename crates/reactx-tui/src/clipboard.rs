//! Clipboard support through OSC 52.
//!
//! Copying writes the text to the terminal emulator's clipboard, which works
//! over SSH as well. Terminals that ignore OSC 52 simply do nothing. tmux
//! users need `set-clipboard on`.

use std::io::{IsTerminal, Write};

/// Copy `text` to the terminal clipboard. Returns whether a terminal received
/// the sequence (false when stdout is not a terminal, e.g. in tests).
pub fn copy(text: &str) -> bool {
    if text.is_empty() || !std::io::stdout().is_terminal() {
        return false;
    }
    let encoded = base64(text.as_bytes());
    let mut out = std::io::stdout().lock();
    write!(out, "\x1b]52;c;{encoded}\x07").is_ok() && out.flush().is_ok()
}

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(ALPHABET[(triple >> 18) as usize & 0x3f] as char);
        out.push(ALPHABET[(triple >> 12) as usize & 0x3f] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[(triple >> 6) as usize & 0x3f] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[triple as usize & 0x3f] as char
        } else {
            '='
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_matches_rfc4648() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"hello world"), "aGVsbG8gd29ybGQ=");
        assert_eq!(base64("héllo".as_bytes()), "aMOpbGxv");
    }
}
