//! Ink-compatible wrap and truncate modes.

use reactx_core::TextWrap;
use unicode_segmentation::UnicodeSegmentation;
use unicode_truncate::UnicodeTruncateStr;
use unicode_width::UnicodeWidthStr;

use super::text::wrap;

const ELLIPSIS: char = '…';

/// Split text into display lines according to an optional [`TextWrap`] mode.
pub fn wrap_mode(text: &str, max_width: Option<usize>, mode: Option<TextWrap>) -> Vec<String> {
    let Some(width) = max_width.map(|width| width.max(1)) else {
        return wrap(text, None);
    };
    match mode.unwrap_or_default() {
        TextWrap::Wrap => wrap(text, Some(width)),
        TextWrap::Hard => wrap_hard(text, width),
        // Ink treats `truncate` as an alias for `truncate-end`.
        TextWrap::Truncate | TextWrap::TruncateEnd => text
            .split('\n')
            .map(|line| truncate_end(line, width))
            .collect(),
        TextWrap::TruncateStart => text
            .split('\n')
            .map(|line| truncate_start(line, width))
            .collect(),
        TextWrap::TruncateMiddle => text
            .split('\n')
            .map(|line| truncate_middle(line, width))
            .collect(),
    }
}

/// Fill every row to `width` cells, breaking words at the column boundary.
fn wrap_hard(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        let mut used = 0;
        for grapheme in paragraph.graphemes(true) {
            let grapheme_width = UnicodeWidthStr::width(grapheme);
            if used + grapheme_width > width && !line.is_empty() {
                lines.push(std::mem::take(&mut line));
                used = 0;
            }
            line.push_str(grapheme);
            used += grapheme_width;
        }
        lines.push(line);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn truncate_end(line: &str, width: usize) -> String {
    if UnicodeWidthStr::width(line) <= width {
        return line.to_string();
    }
    let kept = line.unicode_truncate(width.saturating_sub(1)).0;
    format!("{kept}{ELLIPSIS}")
}

fn truncate_start(line: &str, width: usize) -> String {
    if UnicodeWidthStr::width(line) <= width {
        return line.to_string();
    }
    let kept = line.unicode_truncate_start(width.saturating_sub(1)).0;
    format!("{ELLIPSIS}{kept}")
}

fn truncate_middle(line: &str, width: usize) -> String {
    if UnicodeWidthStr::width(line) <= width {
        return line.to_string();
    }
    let budget = width.saturating_sub(1);
    let head = budget / 2;
    let tail = budget - head;
    let start = line.unicode_truncate(head).0;
    let end = line.unicode_truncate_start(tail).0;
    format!("{start}{ELLIPSIS}{end}")
}

#[cfg(test)]
mod tests {
    use super::wrap_mode;
    use reactx_core::TextWrap;

    #[test]
    fn truncate_modes_fit_the_width() {
        let width = Some(6);
        assert_eq!(
            wrap_mode("hello world", width, Some(TextWrap::Truncate)),
            vec!["hello…"]
        );
        assert_eq!(
            wrap_mode("hello world", width, Some(TextWrap::TruncateEnd)),
            vec!["hello…"]
        );
        assert_eq!(
            wrap_mode("hello world", width, Some(TextWrap::TruncateStart)),
            vec!["…world"]
        );
        assert_eq!(
            wrap_mode("hello world", width, Some(TextWrap::TruncateMiddle)),
            vec!["he…rld"]
        );
        assert_eq!(
            wrap_mode("short", Some(10), Some(TextWrap::TruncateEnd)),
            vec!["short"]
        );
    }

    #[test]
    fn hard_wrap_fills_the_width() {
        assert_eq!(
            wrap_mode("Hello World", Some(7), Some(TextWrap::Hard)),
            vec!["Hello W", "orld"]
        );
        assert_eq!(
            wrap_mode("a b c", Some(3), None),
            vec!["a b", "c"],
            "soft wrap keeps words intact"
        );
    }
}
