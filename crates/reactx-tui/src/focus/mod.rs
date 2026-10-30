//! Keyboard focus ring over the terminal DOM.

use reactx_core::{NodeId, NodeTree, TuiNode};

/// The result of a focus transition, used to emit `blur`/`focus` events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FocusChange {
    pub previous: Option<NodeId>,
    pub current: Option<NodeId>,
}

impl FocusChange {
    pub fn is_change(&self) -> bool {
        self.previous != self.current
    }
}

/// Tracks the document-order list of focusable nodes and which one is active.
/// While a node with `focus-trap` is visible, the ring is restricted to its
/// subtree; focus returns to the previous target when the trap disappears.
#[derive(Debug, Clone, Default)]
pub struct FocusManager {
    order: Vec<NodeId>,
    focused: Option<NodeId>,
    initialized: bool,
    trap: Option<NodeId>,
    saved_focus: Option<NodeId>,
}

impl FocusManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebuild the focus ring from the current tree. Focus is kept if the node
    /// still exists; the first focusable node is selected on the first frame.
    pub fn rebuild(&mut self, tree: &NodeTree) {
        let trap = tree.topmost_focus_trap();
        let previous_trap = self.trap;

        if trap != previous_trap {
            match (previous_trap, trap) {
                (None, Some(_)) => self.saved_focus = self.focused,
                (Some(_), None) => {
                    if let Some(saved) = self.saved_focus.take() {
                        self.focused = Some(saved);
                    }
                }
                _ => {}
            }
            self.trap = trap;
        }

        let focusables = focusable_nodes(tree);
        self.order = match trap {
            Some(trap) => focusables
                .into_iter()
                .filter(|id| tree.is_within(*id, trap))
                .collect(),
            None => focusables,
        };

        if let Some(current) = self.focused {
            if !self.order.contains(&current) {
                self.focused = self.preferred_target(tree);
            }
        } else if !self.initialized || trap != previous_trap {
            self.focused = self.preferred_target(tree);
        }

        if self.focused.is_none() {
            self.focused = self.preferred_target(tree);
        }

        self.initialized = true;
    }

    /// The active trap, if any.
    pub fn trap(&self) -> Option<NodeId> {
        self.trap
    }

    fn preferred_target(&self, tree: &NodeTree) -> Option<NodeId> {
        self.order
            .iter()
            .copied()
            .find(|id| {
                tree.get(*id)
                    .map(|node| node.attrs.autofocus)
                    .unwrap_or(false)
            })
            .or_else(|| self.order.first().copied())
    }

    pub fn order(&self) -> &[NodeId] {
        &self.order
    }

    pub fn focused_id(&self) -> Option<NodeId> {
        self.focused
    }

    pub fn is_focused(&self, id: NodeId) -> bool {
        self.focused == Some(id)
    }

    pub fn set_focus(&mut self, id: Option<NodeId>) -> FocusChange {
        let previous = self.focused;
        self.focused = id.filter(|id| self.order.contains(id));
        self.initialized = true;
        FocusChange {
            previous,
            current: self.focused,
        }
    }

    pub fn clear(&mut self) -> FocusChange {
        self.set_focus(None)
    }

    pub fn focus_first(&mut self) -> FocusChange {
        self.set_focus(self.order.first().copied())
    }

    pub fn focus_last(&mut self) -> FocusChange {
        self.set_focus(self.order.last().copied())
    }

    pub fn focus_next(&mut self) -> FocusChange {
        if self.order.is_empty() {
            return self.set_focus(None);
        }
        let next = match self.focused {
            Some(current) => self
                .order
                .iter()
                .position(|id| *id == current)
                .map(|index| self.order[(index + 1) % self.order.len()])
                .or_else(|| self.order.first().copied()),
            None => self.order.first().copied(),
        };
        self.set_focus(next)
    }

    pub fn focus_prev(&mut self) -> FocusChange {
        if self.order.is_empty() {
            return self.set_focus(None);
        }
        let prev = match self.focused {
            Some(current) => self
                .order
                .iter()
                .position(|id| *id == current)
                .map(|index| {
                    let len = self.order.len();
                    self.order[(index + len - 1) % len]
                })
                .or_else(|| self.order.last().copied()),
            None => self.order.last().copied(),
        };
        self.set_focus(prev)
    }
}

#[cfg(test)]
mod tests;

/// Focusable nodes in document order. Elements registered through Ink-style
/// `use_focus` join the ring even without a `focusable` attribute.
fn focusable_nodes(tree: &NodeTree) -> Vec<NodeId> {
    tree.nodes_in_order()
        .into_iter()
        .filter(|id| *id != tree.root_id())
        .filter(|id| {
            !tree.ancestors(*id).iter().any(|ancestor| {
                tree.get(*ancestor)
                    .is_some_and(|node| node.is_hidden() || node.is_placeholder())
            })
        })
        .filter(|id| {
            tree.get(*id)
                .is_some_and(|node| !node.is_hidden() && (node.is_focusable() || registered(node)))
        })
        .collect()
}

fn registered(node: &TuiNode) -> bool {
    node.attrs
        .custom
        .get("id")
        .is_some_and(|id| crate::runtime::focus::is_registered(id))
}
