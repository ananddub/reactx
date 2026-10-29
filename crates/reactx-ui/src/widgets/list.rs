//! Vertical list rows.

use dioxus::prelude::*;

use crate::theme::use_theme;
use crate::widgets::{style_flag, style_push};

/// A vertical list of [`ListItem`]s.
#[component]
pub fn List(
    #[props(default)] title: Option<String>,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    let theme = use_theme();
    let mut style = String::new();
    style_push(&mut style, "direction", "column");
    style_push(&mut style, "border", "single");
    style_push(&mut style, "border-color", &theme.border.to_css());
    style_push(&mut style, "padding", "0 1");
    if let Some(title) = title.as_deref() {
        style_push(&mut style, "title", title);
    }

    rsx! {
        div { class: class, style: style, {children} }
    }
}

/// One row of a [`List`]. Rows with an `on_click` handler are focusable.
#[component]
pub fn ListItem(
    #[props(default)] selected: bool,
    #[props(default)] on_click: Option<EventHandler<()>>,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    let theme = use_theme();
    let mut style = String::new();
    style_push(&mut style, "padding", "0 1");
    if selected {
        style_push(&mut style, "bg", &theme.selection.to_css());
        style_push(&mut style, "color", &theme.accent.to_css());
        style_flag(&mut style, "bold", true);
    } else {
        style_push(&mut style, "color", &theme.text.to_css());
    }

    if let Some(on_click) = on_click {
        style_flag(&mut style, "focusable", true);
        rsx! {
            span {
                class: class,
                style: style,
                onclick: move |_| on_click.call(()),
                {children}
            }
        }
    } else {
        rsx! {
            span { class: class, style: style, {children} }
        }
    }
}
