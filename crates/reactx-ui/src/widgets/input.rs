//! Text input and textarea fields.

use dioxus::prelude::*;

use crate::theme::use_theme;
use crate::widgets::{style_flag, style_push};

/// A single-line text input. The value is controlled: feed `value` back from
/// `on_input`.
#[component]
pub fn Input(
    on_input: EventHandler<String>,
    #[props(default)] on_submit: Option<EventHandler<String>>,
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] label: String,
    #[props(default)] disabled: bool,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
) -> Element {
    let theme = use_theme();
    let mut style = String::new();
    style_push(&mut style, "border", "single");
    style_push(&mut style, "border-color", &theme.border.to_css());
    style_push(&mut style, "color", &theme.text.to_css());
    style_push(&mut style, "padding", "0 1");
    style_flag(&mut style, "disabled", disabled);

    let submitted = value.clone();
    let field = rsx! {
        input {
            class: class,
            style: style,
            value: value,
            placeholder: placeholder,
            oninput: move |event: FormEvent| on_input.call(event.value()),
            onkeydown: move |event: KeyboardEvent| {
                if event.key() == Key::Enter {
                    if let Some(on_submit) = on_submit {
                        on_submit.call(submitted.clone());
                    }
                }
            },
        }
    };

    if label.is_empty() {
        field
    } else {
        let label_style = format!("color:{}", theme.subtext.to_css());
        rsx! {
            div { style: "direction:column",
                span { style: label_style, "{label}" }
                {field}
            }
        }
    }
}

/// A multiline text area. Enter inserts a newline; `Ctrl+Enter` submits
/// through `on_submit`. The value is controlled: feed `value` back from
/// `on_input`.
#[component]
pub fn TextArea(
    on_input: EventHandler<String>,
    #[props(default)] on_submit: Option<EventHandler<String>>,
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] label: String,
    #[props(default)] height: Option<u16>,
    #[props(default)] disabled: bool,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
) -> Element {
    let theme = use_theme();
    let mut style = String::new();
    style_push(&mut style, "variant", "textarea");
    style_push(&mut style, "border", "single");
    style_push(&mut style, "border-color", &theme.border.to_css());
    style_push(&mut style, "color", &theme.text.to_css());
    style_push(&mut style, "padding", "0 1");
    style_flag(&mut style, "focusable", true);
    style_flag(&mut style, "disabled", disabled);
    if let Some(height) = height {
        style_push(&mut style, "height", &height.to_string());
    }

    let submitted = value.clone();
    let field = rsx! {
        textarea {
            class: class,
            style: style,
            value: value,
            placeholder: placeholder,
            oninput: move |event: FormEvent| on_input.call(event.value()),
            onkeydown: move |event: KeyboardEvent| {
                if event.key() == Key::Enter && event.modifiers().contains(Modifiers::CONTROL) {
                    if let Some(on_submit) = on_submit {
                        on_submit.call(submitted.clone());
                    }
                }
            },
        }
    };

    if label.is_empty() {
        field
    } else {
        let label_style = format!("color:{}", theme.subtext.to_css());
        rsx! {
            div { style: "direction:column",
                span { style: label_style, "{label}" }
                {field}
            }
        }
    }
}
