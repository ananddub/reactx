//! Selectable tab strip.

use dioxus::prelude::*;

use crate::theme::use_theme;
use crate::widgets::{style_flag, style_push};

/// A strip of selectable tabs.
#[component]
pub fn Tabs(
    tabs: Vec<String>,
    on_select: EventHandler<usize>,
    #[props(default)] selected: usize,
) -> Element {
    let theme = use_theme();
    let mut row_style = String::new();
    style_push(&mut row_style, "direction", "row");
    style_push(&mut row_style, "gap", "1");

    let items: Vec<Element> = tabs
        .iter()
        .enumerate()
        .map(|(index, tab)| {
            let active = index == selected;
            let mut style = String::new();
            style_push(
                &mut style,
                "border",
                if active { "rounded" } else { "none" },
            );
            style_push(
                &mut style,
                "color",
                &if active { theme.accent } else { theme.subtext }.to_css(),
            );
            style_push(&mut style, "padding", "0 1");
            style_flag(&mut style, "bold", active);
            rsx! {
                button {
                    key: "{index}",
                    style: style,
                    onclick: move |_| on_select.call(index),
                    "{tab}"
                }
            }
        })
        .collect();

    rsx! {
        div { style: row_style, {items.into_iter()} }
    }
}
