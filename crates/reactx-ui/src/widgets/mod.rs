//! Themed terminal widgets.
//!
//! Widgets render plain elements with inline `style` declarations. The
//! terminal renderer turns those declarations into Ratatui cells, so a widget
//! tree stays a normal Dioxus element tree.

mod container;
mod controls;
mod input;
mod list;
mod modal;
mod tabs;
mod text;

pub use container::Container;
pub use controls::{Button, Checkbox};
pub use input::{Input, TextArea};
pub use list::{List, ListItem};
pub use modal::Modal;
pub use tabs::Tabs;
pub use text::Text;

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
