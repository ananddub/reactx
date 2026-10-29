//! Centered modal overlay.

use dioxus::prelude::*;

use crate::theme::use_theme;
use crate::widgets::Button;
use crate::widgets::{style_flag, style_push};

/// A centred overlay. Render it as the last child of the app root so it paints
/// above the rest of the tree.
#[component]
pub fn Modal(
    open: bool,
    #[props(default)] title: String,
    #[props(default)] on_close: Option<EventHandler<()>>,
    /// Tailwind-style utility classes, applied to the card beneath its style.
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }
    let theme = use_theme();

    let mut overlay = String::new();
    style_push(&mut overlay, "position", "absolute");
    style_push(&mut overlay, "left", "0");
    style_push(&mut overlay, "top", "0");
    style_push(&mut overlay, "width", "100%");
    style_push(&mut overlay, "height", "100%");
    style_push(&mut overlay, "direction", "column");
    style_push(&mut overlay, "justify", "center");
    style_push(&mut overlay, "align", "center");
    style_push(&mut overlay, "bg", &theme.crust.to_css());
    style_push(&mut overlay, "layer", "100");
    style_flag(&mut overlay, "focus-trap", true);

    let mut card = String::new();
    style_push(&mut card, "direction", "column");
    style_push(&mut card, "gap", "1");
    style_push(&mut card, "border", "rounded");
    style_push(&mut card, "border-color", &theme.accent.to_css());
    style_push(&mut card, "bg", &theme.base.to_css());
    style_push(&mut card, "color", &theme.text.to_css());
    style_push(&mut card, "padding", "1 2");
    style_push(&mut card, "focusable", "false");
    if !title.is_empty() {
        style_push(&mut card, "title", &title);
    }

    let close = on_close;
    rsx! {
        div {
            style: overlay,
            onclick: move |_| {
                if let Some(close) = close {
                    close.call(());
                }
            },
            onkeydown: move |event: Event<KeyboardData>| {
                if event.key() == Key::Escape {
                    if let Some(close) = close {
                        close.call(());
                    }
                }
            },
            div {
                class: class,
                style: card,
                onclick: move |event: Event<MouseData>| event.stop_propagation(),
                {children}
                if let Some(on_close) = on_close {
                    div { style: "direction:row;justify:end",
                        Button { on_click: move |_| on_close.call(()), "Close" }
                    }
                }
            }
        }
    }
}
