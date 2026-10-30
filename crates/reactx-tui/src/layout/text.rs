//! Widget kinds, labels and text wrapping.

use reactx_core::{NodeTree, TuiNode};
use textwrap::Options;
use unicode_width::UnicodeWidthStr;

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
    let Some(width) = max_width else {
        return text.split('\n').map(str::to_string).collect();
    };
    let width = width.max(1);

    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }
        let options = Options::new(width).break_words(true);
        lines.extend(
            textwrap::wrap(paragraph, options)
                .into_iter()
                .map(|line| line.into_owned()),
        );
    }

    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
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

#[cfg(test)]
mod tests {
    use super::wrap;

    #[test]
    fn wraps_words_and_long_tokens() {
        assert_eq!(wrap("hello world", Some(5)), vec!["hello", "world"]);
        assert_eq!(wrap("abcdefgh", Some(3)), vec!["abc", "def", "gh"]);
    }

    #[test]
    fn wraps_wide_text() {
        assert_eq!(wrap("你好世界", Some(4)), vec!["你好", "世界"]);
    }

    #[test]
    fn none_disables_wrapping() {
        assert_eq!(wrap("a b c", None), vec!["a b c"]);
        assert_eq!(wrap("a\n\nb", None), vec!["a", "", "b"]);
    }
}
