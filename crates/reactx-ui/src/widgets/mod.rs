//! Themed terminal widgets.
//!
//! Widgets render plain elements with inline `style` declarations. The
//! terminal renderer turns those declarations into Ratatui cells, so a widget
//! tree stays a normal Dioxus element tree.

mod animate;
mod r#box;
mod controls;
mod flow;
mod input;
mod list;
mod modal;
mod static_out;
mod tabs;
mod text;
mod transform;

pub use animate::Animate;
pub use controls::{Button, Checkbox};
pub use flow::{Newline, Spacer};
pub use input::{Input, TextArea};
pub use list::{List, ListItem};
pub use modal::Modal;
pub use r#box::Box;
pub use static_out::Static;
pub use tabs::Tabs;
pub use text::Text;
pub use transform::Transform;

use dioxus::prelude::*;

/// Pick what a screen reader should hear, or the visual children otherwise.
pub(crate) fn assistive_body(aria_hidden: bool, label: Option<&str>, children: Element) -> Element {
    if !reactx_core::screen::is_enabled() {
        return rsx! { {children} };
    }
    if aria_hidden {
        return rsx! {};
    }
    match label {
        Some(label) => rsx! { "{label}" },
        None => rsx! { {children} },
    }
}

pub(crate) fn style_push(style: &mut String, key: &str, value: &str) {
    let value = value.trim();
    if value.is_empty() {
        return;
    }
    if !style.is_empty() {
        style.push(';');
    }
    style.push_str(key);
    style.push(':');
    style.push_str(value);
}

pub(crate) fn style_flag(style: &mut String, key: &str, value: bool) {
    if value {
        style_push(style, key, "true");
    }
}
