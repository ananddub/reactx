//! Tabular grid.

use dioxus::prelude::*;
use reactx_core::wire;

use super::{border_style, css, title_style};
use crate::theme::use_theme;
use crate::widgets::style_push;

/// A tabular grid with headers and an optional highlighted row.
#[component]
pub fn Table(
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    #[props(default)] widths: Option<Vec<u16>>,
    #[props(default)] selected_row: Option<usize>,
    #[props(default)] title: String,
    #[props(default)] border: String,
    #[props(default)] highlight_color: String,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
) -> Element {
    let theme = use_theme();
    let border = if border.is_empty() {
        "single".to_string()
    } else {
        border
    };
    let highlight = if highlight_color.is_empty() {
        css(theme.selection)
    } else {
        highlight_color
    };

    let mut style = String::new();
    style_push(&mut style, "variant", "table");
    style_push(&mut style, "table-headers", &wire::join_fields(&headers));
    style_push(&mut style, "table-rows", &wire::join_rows(&rows));
    if let Some(widths) = widths {
        let widths: Vec<u64> = widths.into_iter().map(u64::from).collect();
        style_push(&mut style, "table-widths", &wire::join_nums(&widths));
    }
    if let Some(selected) = selected_row {
        style_push(&mut style, "table-selected", &selected.to_string());
    }
    style_push(&mut style, "table-highlight", &highlight);
    style_push(&mut style, "color", &css(theme.text));
    title_style(&mut style, &title);
    border_style(&mut style, &border, &theme);

    rsx! {
        div { class: class, style: style }
    }
}
