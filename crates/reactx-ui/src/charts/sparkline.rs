//! Unicode trendline.

use dioxus::prelude::*;
use reactx_core::wire;

use super::{border_style, css, title_style};
use crate::theme::use_theme;
use crate::widgets::style_push;

/// A compact unicode trendline (`▁▂▃▄▅▆▇█`).
#[component]
pub fn Sparkline(
    data: Vec<u64>,
    #[props(default)] max: Option<u64>,
    #[props(default)] color: String,
    #[props(default)] title: String,
    #[props(default)] border: String,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
) -> Element {
    let theme = use_theme();
    let fill = if color.is_empty() {
        css(theme.green)
    } else {
        color
    };

    let mut style = String::new();
    style_push(&mut style, "variant", "sparkline");
    style_push(&mut style, "sparkline-data", &wire::join_nums(&data));
    if let Some(max) = max {
        style_push(&mut style, "sparkline-max", &max.to_string());
    }
    style_push(&mut style, "sparkline-color", &fill);
    title_style(&mut style, &title);
    border_style(&mut style, &border, &theme);

    rsx! {
        div { class: class, style: style }
    }
}
