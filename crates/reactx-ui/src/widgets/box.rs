//! Flexbox layout container.

use dioxus::prelude::*;

use crate::widgets::{assistive_body, style_flag, style_push};

/// Props for [`Box`].
#[derive(Props, Clone, PartialEq)]
pub struct BoxProps {
    #[props(default)]
    pub direction: String,
    #[props(default)]
    pub justify: String,
    #[props(default)]
    pub align: String,
    #[props(default)]
    pub gap: Option<u16>,
    #[props(default)]
    pub padding: Option<String>,
    #[props(default)]
    pub border: String,
    #[props(default)]
    pub border_color: String,
    /// Override the border colour of one edge.
    #[props(default)]
    pub border_top_color: Option<String>,
    #[props(default)]
    pub border_right_color: Option<String>,
    #[props(default)]
    pub border_bottom_color: Option<String>,
    #[props(default)]
    pub border_left_color: Option<String>,
    /// Render the border dimmed.
    #[props(default)]
    pub border_dim: bool,
    #[props(default)]
    pub bg: Option<String>,
    #[props(default)]
    pub color: Option<String>,
    #[props(default)]
    pub width: Option<String>,
    #[props(default)]
    pub height: Option<String>,
    /// Width-to-height ratio enforced by the layout engine.
    #[props(default)]
    pub aspect_ratio: Option<f32>,
    #[props(default)]
    pub flex_grow: Option<f32>,
    #[props(default)]
    pub title: Option<String>,
    /// Text announced instead of the children by screen readers.
    #[props(default)]
    pub aria_label: Option<String>,
    /// Hide this box from screen readers.
    #[props(default)]
    pub aria_hidden: bool,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    pub class: String,
    pub children: Element,
}

/// A layout container exposing the common flexbox properties.
///
/// Written by hand because a `#[component]` named `Box` would shadow
/// `std::boxed::Box` in the type namespace used by the `jsx!` expansion.
#[allow(non_snake_case)]
pub fn Box(props: BoxProps) -> Element {
    let mut style = String::new();
    style_push(&mut style, "direction", &props.direction);
    style_push(&mut style, "justify", &props.justify);
    style_push(&mut style, "align", &props.align);
    if let Some(gap) = props.gap {
        style_push(&mut style, "gap", &gap.to_string());
    }
    if let Some(padding) = props.padding.as_deref() {
        style_push(&mut style, "padding", padding);
    }
    style_push(&mut style, "border", &props.border);
    style_push(&mut style, "border-color", &props.border_color);
    for (edge, color) in [
        ("border-top-color", &props.border_top_color),
        ("border-right-color", &props.border_right_color),
        ("border-bottom-color", &props.border_bottom_color),
        ("border-left-color", &props.border_left_color),
    ] {
        if let Some(color) = color.as_deref() {
            style_push(&mut style, edge, color);
        }
    }
    style_flag(&mut style, "border-dim", props.border_dim);
    if let Some(bg) = props.bg.as_deref() {
        style_push(&mut style, "bg", bg);
    }
    if let Some(color) = props.color.as_deref() {
        style_push(&mut style, "color", color);
    }
    if let Some(width) = props.width.as_deref() {
        style_push(&mut style, "width", width);
    }
    if let Some(height) = props.height.as_deref() {
        style_push(&mut style, "height", height);
    }
    if let Some(ratio) = props.aspect_ratio {
        style_push(&mut style, "aspect-ratio", &ratio.to_string());
    }
    if let Some(flex_grow) = props.flex_grow {
        style_push(&mut style, "flex-grow", &flex_grow.to_string());
    }
    if let Some(title) = props.title.as_deref() {
        style_push(&mut style, "title", title);
    }

    let body = assistive_body(
        props.aria_hidden,
        props.aria_label.as_deref(),
        props.children,
    );
    rsx! {
        div { class: props.class, style: style, {body} }
    }
}
