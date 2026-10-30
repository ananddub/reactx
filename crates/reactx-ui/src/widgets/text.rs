//! Themed inline text.

use dioxus::prelude::*;

use crate::theme::use_theme;
use crate::widgets::{style_flag, style_push};

/// Themed inline text.
#[component]
pub fn Text(
    #[props(default)] color: String,
    /// Background colour (CSS colour name or `#rrggbb`).
    #[props(default)]
    bg: String,
    #[props(default)] align: String,
    #[props(default)] bold: bool,
    #[props(default)] italic: bool,
    #[props(default)] underline: bool,
    #[props(default)] strikethrough: bool,
    #[props(default)] dim: bool,
    /// Swap foreground and background colours.
    #[props(default)]
    inverse: bool,
    /// How overflowing text wraps or truncates: `wrap`, `hard`, `truncate`,
    /// `truncate-start`, `truncate-middle` or `truncate-end`.
    #[props(default)]
    wrap: String,
    /// Text announced instead of the children by screen readers.
    #[props(default)]
    aria_label: Option<String>,
    /// Hide this text from screen readers.
    #[props(default)]
    aria_hidden: bool,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    let theme = use_theme();
    let color = if color.is_empty() {
        theme.text.to_css()
    } else {
        color
    };

    let mut style = String::new();
    style_push(&mut style, "color", &color);
    style_push(&mut style, "bg", &bg);
    style_push(&mut style, "text-align", &align);
    style_push(&mut style, "text-wrap", &wrap);
    style_flag(&mut style, "bold", bold);
    style_flag(&mut style, "italic", italic);
    style_flag(&mut style, "underline", underline);
    style_flag(&mut style, "strikethrough", strikethrough);
    style_flag(&mut style, "dim", dim);
    style_flag(&mut style, "inverse", inverse);

    let body = crate::widgets::assistive_body(aria_hidden, aria_label.as_deref(), children);
    rsx! {
        span { class: class, style: style, {body} }
    }
}
