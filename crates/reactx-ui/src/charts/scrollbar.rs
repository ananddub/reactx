//! Scroll position indicator.

use super::css;
use crate::theme::use_theme;
use crate::widgets::style_push;
use dioxus::prelude::*;

/// A scroll position indicator for long content.
#[component]
pub fn Scrollbar(
    position: usize,
    content_length: usize,
    #[props(default)] orientation: Option<String>,
    #[props(default)] thumb_color: String,
    #[props(default)] track_color: String,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
) -> Element {
    let theme = use_theme();
    let thumb = if thumb_color.is_empty() {
        css(theme.accent)
    } else {
        thumb_color
    };
    let track = if track_color.is_empty() {
        css(theme.border)
    } else {
        track_color
    };

    let mut style = String::new();
    style_push(&mut style, "variant", "scrollbar");
    style_push(&mut style, "scrollbar-position", &position.to_string());
    style_push(
        &mut style,
        "scrollbar-content-length",
        &content_length.to_string(),
    );
    if let Some(orientation) = orientation {
        style_push(&mut style, "scrollbar-orientation", &orientation);
    }
    style_push(&mut style, "scrollbar-thumb", &thumb);
    style_push(&mut style, "scrollbar-track", &track);

    rsx! {
        div { class: class, style: style }
    }
}
