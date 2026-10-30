//! Rewrite the text rendered by a subtree, like Ink's `Transform`.

use dioxus::prelude::*;

/// Props for [`Transform`].
#[derive(Props, Clone, PartialEq)]
pub struct TransformProps {
    /// Maps the text rendered by the children to replacement text.
    pub transform: Callback<String, String>,
    pub children: Element,
}

/// Rewrites the text its children render.
///
/// Only the text changes: styles, padding and layout are untouched, and the
/// callback receives plain text, not the ANSI sequences used to paint styles.
#[allow(non_snake_case)]
pub fn Transform(props: TransformProps) -> Element {
    let id = use_hook(reactx_core::transform::next_id);
    {
        let id = id.clone();
        use_drop(move || reactx_core::transform::remove(&id));
    }
    reactx_core::transform::register(&id, props.transform);
    rsx! {
        span { "data-transform": id, {props.children} }
    }
}
