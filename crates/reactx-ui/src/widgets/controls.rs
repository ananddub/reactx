//! Button and checkbox controls.

use dioxus::prelude::*;

use crate::theme::use_theme;
use crate::widgets::{style_flag, style_push};

/// A focusable, themed button. `variant` is `primary` (default), `success`,
/// `danger` or `ghost`.
#[component]
pub fn Button(
    on_click: EventHandler<MouseEvent>,
    #[props(default)] variant: String,
    #[props(default)] disabled: bool,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    let theme = use_theme();
    let accent = match variant.as_str() {
        "danger" => theme.red,
        "success" => theme.green,
        "ghost" => theme.overlay,
        _ => theme.accent,
    };

    let mut style = String::new();
    style_push(
        &mut style,
        "border",
        if variant == "ghost" {
            "none"
        } else {
            "rounded"
        },
    );
    style_push(&mut style, "border-color", &accent.to_css());
    style_push(&mut style, "bg", &theme.surface.to_css());
    style_push(&mut style, "color", &theme.text.to_css());
    style_push(&mut style, "padding", "0 1");
    style_flag(&mut style, "bold", true);
    style_flag(&mut style, "disabled", disabled);
    style_flag(&mut style, "dim", disabled);

    rsx! {
        button {
            class: class,
            style: style,
            onclick: move |event| {
                if !disabled {
                    on_click.call(event);
                }
            },
            {children}
        }
    }
}

/// A checkbox row toggled by click or Enter.
#[component]
pub fn Checkbox(
    on_change: EventHandler<bool>,
    #[props(default)] checked: bool,
    #[props(default)] label: String,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
) -> Element {
    let theme = use_theme();
    let mut style = String::new();
    style_push(&mut style, "variant", "checkbox");
    style_flag(&mut style, "checked", checked);
    style_flag(&mut style, "focusable", true);
    style_push(&mut style, "color", &theme.text.to_css());

    rsx! {
        span {
            class: class,
            style: style,
            onclick: move |_| on_change.call(!checked),
            "{label}"
        }
    }
}
