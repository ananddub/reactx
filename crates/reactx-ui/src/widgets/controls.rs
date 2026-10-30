//! Button and checkbox controls.

use dioxus::prelude::*;

use crate::theme::use_theme;
use crate::widgets::{style_flag, style_push};

/// A focusable button. `variant` is `primary` (default), `success`, `danger`
/// or `ghost`. Variant styling is emitted as Tailwind classes before your own
/// `class`, so any utility you pass wins; no background is painted unless you
/// ask for one.
#[component]
pub fn Button(
    on_click: EventHandler<MouseEvent>,
    #[props(default)] variant: String,
    #[props(default)] disabled: bool,
    /// Tailwind-style utility classes, applied after the widget defaults.
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    let theme = use_theme();
    let accent = match variant.as_str() {
        "danger" => theme.red,
        "success" => theme.green,
        _ => theme.accent,
    };
    let border = if variant == "ghost" {
        "border-none".to_string()
    } else {
        format!("border rounded border-[{}]", accent.to_css())
    };
    let defaults = format!("text-[{}] px-1 bold {border}", theme.text.to_css());

    let mut style = String::new();
    style_flag(&mut style, "disabled", disabled);
    style_flag(&mut style, "dim", disabled);

    rsx! {
        button {
            class: format!("{defaults} {class}"),
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
