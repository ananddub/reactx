//! The in-memory terminal node tree.

use dioxus_core::ElementId;
use rustc_hash::FxHashMap;
use std::collections::HashSet;

use crate::style::{ClassState, Display, StyleProps};

mod mutations;
mod transform;
mod traversal;

#[cfg(test)]
mod tests;

/// Stable, tree-local identifier for a terminal node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NodeId(pub u64);

impl NodeId {
    pub const ROOT: Self = Self(0);
}

/// A mounted terminal node.
#[derive(Debug, Clone, Default)]
pub struct TuiNode {
    pub id: NodeId,
    /// The Dioxus id this node is addressed by, when it has one.
    pub dioxus_id: Option<ElementId>,
    /// Normalized tag name (`box`, `text`, `button`, `input`, `#text`, ...).
    pub tag: String,
    pub text: Option<String>,
    /// Effective style props after base attributes, classes and inline style.
    pub attrs: StyleProps,
    /// Attributes that do not come from `class`/`style`; kept so [`Self::attrs`]
    /// can be rebuilt when variant state or the terminal size changes.
    pub base_attrs: StyleProps,
    /// Text produced by a surrounding `<Transform>`; overrides the collected
    /// child text when present.
    pub text_override: Option<String>,
    /// Raw `class="..."` list, if any.
    pub class: Option<String>,
    /// Raw `style="..."` declarations, if any.
    pub style: Option<String>,
    pub children: Vec<NodeId>,
    pub parent: Option<NodeId>,
    /// Event listener names, without the `on` prefix.
    pub events: HashSet<String>,
}

impl TuiNode {
    pub fn is_text(&self) -> bool {
        self.tag == "#text"
    }

    pub fn is_placeholder(&self) -> bool {
        self.tag == "#placeholder"
    }

    pub fn is_leaf(&self) -> bool {
        matches!(
            self.tag.as_str(),
            "#text"
                | "#placeholder"
                | "text"
                | "p"
                | "span"
                | "button"
                | "input"
                | "checkbox"
                | "spacer"
        )
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self.attrs.display, Some(Display::None))
    }

    /// Recompute [`Self::attrs`] in CSS precedence order: base attributes,
    /// then classes (plain utilities before matching variants), then inline
    /// `style`.
    pub fn rebuild_styles(&mut self, state: &ClassState) {
        self.attrs = self.base_attrs.clone();
        if let Some(classes) = self.class.as_deref() {
            self.attrs.apply_classes(classes, state);
        }
        if let Some(style) = self.style.as_deref() {
            self.attrs.apply_style_string(style);
        }
    }

    /// Whether the node participates in keyboard focus navigation.
    pub fn is_focusable(&self) -> bool {
        if self.is_text() || self.is_placeholder() || self.attrs.disabled || self.is_hidden() {
            return false;
        }
        if self.attrs.focusable == Some(false) || self.attrs.focus_trap {
            return false;
        }
        matches!(self.tag.as_str(), "button" | "input" | "checkbox")
            || self.attrs.focusable == Some(true)
            || self.events.contains("click")
            || self.events.contains("input")
            || self.events.contains("keydown")
    }
}

/// The in-memory terminal VDOM.
#[derive(Debug)]
pub struct NodeTree {
    nodes: FxHashMap<NodeId, TuiNode>,
    root: NodeId,
    next_id: u64,
    element_map: FxHashMap<ElementId, NodeId>,
}

impl Default for NodeTree {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeTree {
    pub fn new() -> Self {
        let root = NodeId::ROOT;
        let mut nodes = FxHashMap::default();
        nodes.insert(
            root,
            TuiNode {
                id: root,
                tag: "#root".to_string(),
                ..Default::default()
            },
        );
        Self {
            nodes,
            root,
            next_id: 1,
            element_map: FxHashMap::default(),
        }
    }

    pub fn root_id(&self) -> NodeId {
        self.root
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn contains(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn get(&self, id: NodeId) -> Option<&TuiNode> {
        self.nodes.get(&id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut TuiNode> {
        self.nodes.get_mut(&id)
    }
}
