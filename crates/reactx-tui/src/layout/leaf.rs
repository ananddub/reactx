//! Measured text leaves and text-input classification.

use reactx_core::{NodeTree, TuiNode};

use super::text::{checkbox_text, widget_kind};

/// A node treated as a single measured leaf instead of a flex container.
#[derive(Debug, Clone)]
pub(crate) struct Leaf {
    pub text: String,
    /// Soft-wrap long lines to the available width. Text inputs scroll
    /// horizontally instead.
    pub wrap: bool,
}

pub(crate) fn leaf_for(tree: &NodeTree, node: &TuiNode) -> Option<Leaf> {
    let kind = widget_kind(node);
    let text = match kind {
        "text" => tree.collect_text(node.id),
        "#text" => node.text.clone().unwrap_or_default(),
        "button" => tree.collect_text(node.id),
        "checkbox" => checkbox_text(node, &tree.collect_text(node.id)),
        "input" | "textarea" => node
            .attrs
            .value
            .clone()
            .or_else(|| node.attrs.placeholder.clone())
            .unwrap_or_default(),
        _ => return None,
    };
    // Inputs and text areas keep their value on one logical line; painting
    // scrolls instead of wrapping. `textarea` still honours hard newlines.
    Some(Leaf {
        text,
        wrap: !matches!(kind, "input" | "textarea"),
    })
}

/// Whether a node edits text with the shared caret/selection behaviour.
pub(crate) fn is_text_input(node: &TuiNode) -> bool {
    matches!(widget_kind(node), "input" | "textarea")
}

/// Whether a text input edits multiple lines.
pub(crate) fn is_textarea(node: &TuiNode) -> bool {
    widget_kind(node) == "textarea"
}
