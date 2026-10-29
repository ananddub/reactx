//! Categorical bar chart.

use dioxus::prelude::*;
use reactx_core::wire;

use super::{border_style, css, title_style};
use crate::theme::use_theme;
use crate::widgets::style_push;

/// A categorical bar chart. Labels come from `data`; `horizontal` swaps the
/// axes.
#[component]
pub fn BarChart(
    data: Vec<(String, u64)>,
    #[props(default)] bar_width: Option<u16>,
    #[props(default)] bar_gap: Option<u16>,
    #[props(default)] horizontal: bool,
    #[props(default)] max: Option<u64>,
    #[props(default)] color: String,
    #[props(default)] title: String,
    #[props(default)] border: String,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
) -> Element {
    let theme = use_theme();
    let rows: Vec<Vec<String>> = data
        .iter()
        .map(|(label, value)| vec![label.clone(), value.to_string()])
        .collect();
    let fill = if color.is_empty() {
        css(theme.accent)
    } else {
        color
    };

    let mut style = String::new();
    style_push(&mut style, "variant", "barchart");
    style_push(&mut style, "barchart-data", &wire::join_rows(&rows));
    if let Some(bar_width) = bar_width {
        style_push(&mut style, "barchart-bar-width", &bar_width.to_string());
    }
    if let Some(bar_gap) = bar_gap {
        style_push(&mut style, "barchart-bar-gap", &bar_gap.to_string());
    }
    if horizontal {
        style_push(&mut style, "barchart-horizontal", "true");
    }
    if let Some(max) = max {
        style_push(&mut style, "barchart-max", &max.to_string());
    }
    style_push(&mut style, "barchart-color", &fill);
    title_style(&mut style, &title);
    border_style(&mut style, &border, &theme);

    rsx! {
        div { class: class, style: style }
    }
}
