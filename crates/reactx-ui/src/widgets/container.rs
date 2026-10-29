//! Flexbox container.

use dioxus::prelude::*;

use crate::widgets::style_push;

/// A layout container exposing the common flexbox properties.
#[component]
pub fn Container(
    #[props(default)] direction: String,
    #[props(default)] justify: String,
    #[props(default)] align: String,
    #[props(default)] gap: Option<u16>,
    #[props(default)] padding: Option<String>,
    #[props(default)] border: String,
    #[props(default)] border_color: String,
    #[props(default)] bg: Option<String>,
    #[props(default)] color: Option<String>,
    #[props(default)] width: Option<String>,
    #[props(default)] height: Option<String>,
    #[props(default)] flex_grow: Option<f32>,
    #[props(default)] title: Option<String>,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    let mut style = String::new();
    style_push(&mut style, "direction", &direction);
    style_push(&mut style, "justify", &justify);
    style_push(&mut style, "align", &align);
    if let Some(gap) = gap {
        style_push(&mut style, "gap", &gap.to_string());
    }
    if let Some(padding) = padding.as_deref() {
        style_push(&mut style, "padding", padding);
    }
    style_push(&mut style, "border", &border);
    style_push(&mut style, "border-color", &border_color);
    if let Some(bg) = bg.as_deref() {
        style_push(&mut style, "bg", bg);
    }
    if let Some(color) = color.as_deref() {
        style_push(&mut style, "color", color);
    }
    if let Some(width) = width.as_deref() {
        style_push(&mut style, "width", width);
    }
    if let Some(height) = height.as_deref() {
        style_push(&mut style, "height", height);
    }
    if let Some(flex_grow) = flex_grow {
        style_push(&mut style, "flex-grow", &flex_grow.to_string());
    }
    if let Some(title) = title.as_deref() {
        style_push(&mut style, "title", title);
    }

    rsx! {
        div { class: class, style: style, {children} }
    }
}
