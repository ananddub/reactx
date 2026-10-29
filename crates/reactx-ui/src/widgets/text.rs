//! Themed inline text.

use dioxus::prelude::*;

use crate::theme::use_theme;
use crate::widgets::{style_flag, style_push};

/// Themed inline text.
#[component]
pub fn Text(
    #[props(default)] color: String,
    #[props(default)] align: String,
    #[props(default)] bold: bool,
    #[props(default)] italic: bool,
    #[props(default)] underline: bool,
    #[props(default)] strikethrough: bool,
    #[props(default)] dim: bool,
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
    style_push(&mut style, "text-align", &align);
    style_flag(&mut style, "bold", bold);
    style_flag(&mut style, "italic", italic);
    style_flag(&mut style, "underline", underline);
    style_flag(&mut style, "strikethrough", strikethrough);
    style_flag(&mut style, "dim", dim);

    rsx! {
        span { style: style, {children} }
    }
}
