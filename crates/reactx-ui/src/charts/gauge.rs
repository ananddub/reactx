//! Filled and line gauges.

use super::{border_style, css, title_style};
use crate::theme::use_theme;
use crate::widgets::style_push;
use dioxus::prelude::*;

/// A filled progress bar. Values in `0.0..=1.0` are ratios; larger values are
/// treated as percentages (`82.0` means 82%).
#[component]
pub fn Gauge(
    value: f64,
    #[props(default)] label: String,
    #[props(default)] use_line: bool,
    #[props(default)] color: String,
    #[props(default)] bg: String,
    #[props(default)] title: String,
    #[props(default)] border: String,
) -> Element {
    let theme = use_theme();
    let ratio = if value > 1.0 { value / 100.0 } else { value };
    let fill = if color.is_empty() {
        css(theme.accent)
    } else {
        color
    };
    let track = if bg.is_empty() {
        css(theme.surface)
    } else {
        bg
    };

    let mut style = String::new();
    style_push(
        &mut style,
        "variant",
        if use_line { "line-gauge" } else { "gauge" },
    );
    style_push(
        &mut style,
        "gauge-ratio",
        &ratio.clamp(0.0, 1.0).to_string(),
    );
    style_push(&mut style, "gauge-label", &label);
    style_push(&mut style, "color", &fill);
    style_push(&mut style, "bg", &track);
    title_style(&mut style, &title);
    border_style(&mut style, &border, &theme);

    rsx! {
        div { style: style }
    }
}
