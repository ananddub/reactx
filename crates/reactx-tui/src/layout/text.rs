//! Widget kinds, labels and text wrapping.

use reactx_core::{NodeTree, TuiNode};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// The widget a node behaves as. `variant` lets a plain element opt into a
/// widget rendering (for example `span` with `variant:checkbox` or a `div`
/// with `variant:table`).
pub fn widget_kind(node: &TuiNode) -> &str {
    node.attrs.variant.as_deref().unwrap_or(node.tag.as_str())
}

pub(crate) fn checkbox_text(node: &TuiNode, fallback: &str) -> String {
    let label = node.attrs.label.as_deref().unwrap_or(fallback);
    let mark = if node.attrs.checked.unwrap_or(false) {
        "x"
    } else {
        " "
    };
    if label.is_empty() {
        format!("[{mark}]")
    } else {
        format!("[{mark}] {label}")
    }
}

/// Split text into display lines that fit `max_width` cells. `None` disables wrapping.
pub fn wrap(text: &str, max_width: Option<usize>) -> Vec<String> {
    let width = match max_width {
        Some(width) => width.max(1),
        None => usize::MAX,
    };

    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }

        let mut line = String::new();
        let mut line_width = 0usize;

        for word in paragraph.split(' ').filter(|word| !word.is_empty()) {
            let word_width = UnicodeWidthStr::width(word);
            if line_width > 0 && line_width + 1 + word_width <= width {
                line.push(' ');
                line.push_str(word);
                line_width += 1 + word_width;
                continue;
            }
            if line_width > 0 {
                lines.push(std::mem::take(&mut line));
                line_width = 0;
            }
            push_long_word(&mut lines, &mut line, &mut line_width, word, width);
        }

        lines.push(line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn push_long_word(
    lines: &mut Vec<String>,
    line: &mut String,
    line_width: &mut usize,
    word: &str,
    width: usize,
) {
    if UnicodeWidthStr::width(word) <= width {
        line.push_str(word);
        *line_width = UnicodeWidthStr::width(word);
        return;
    }

    for ch in word.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if *line_width > 0 && *line_width + ch_width > width {
            lines.push(std::mem::take(line));
            *line_width = 0;
        }
        line.push(ch);
        *line_width += ch_width;
    }
}

/// Display width of a string in terminal cells.
pub fn str_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

/// The text a node contributes to rendering, without border/padding.
pub fn node_label(tree: &NodeTree, node: &TuiNode) -> String {
    match node.tag.as_str() {
        "text" | "button" => tree.collect_text(node.id),
        "#text" => node.text.clone().unwrap_or_default(),
        "checkbox" => checkbox_text(node, &tree.collect_text(node.id)),
        "input" | "textarea" => node
            .attrs
            .value
            .clone()
            .unwrap_or_else(|| node.attrs.placeholder.clone().unwrap_or_default()),
        _ => node.text.clone().unwrap_or_default(),
    }
}
