//! Flow helpers: explicit line breaks and grow-to-fill space.

use dioxus::prelude::*;

/// Inserts `count` line breaks into the surrounding text.
///
/// Like Ink's `Newline`, this is meant to be nested inside a text element:
///
/// ```ignore
/// jsx! { <Text>{"first" <Newline/> "second"}</Text> }
/// ```
#[component]
pub fn Newline(#[props(default = 1)] count: u16) -> Element {
    let text = "\n".repeat(count as usize);
    rsx! {
        span { {text} }
    }
}

/// Expands to fill the available space along the parent's main axis.
#[component]
pub fn Spacer() -> Element {
    rsx! {
        div { style: "flex-grow:1" }
    }
}
