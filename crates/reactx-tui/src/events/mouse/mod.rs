//! Mouse hit testing shared by clicks, hover and scrolling.

use reactx_core::NodeId;

use crate::TuiRenderer;

mod buttons;
mod click;
mod hover;

pub(super) use buttons::{handle_mouse_button, handle_scroll};
pub(super) use click::{handle_click, handle_mouse_down};
pub(super) use hover::handle_mouse_move;

/// The topmost node under the pointer that is not outside an active focus trap.
pub(super) fn pointer_target(renderer: &TuiRenderer, x: u16, y: u16) -> Option<NodeId> {
    let node_id = renderer.node_at(x, y)?;
    if let Some(trap) = renderer.focus.trap() {
        if !renderer.tree.is_within(node_id, trap) {
            return None;
        }
    }
    Some(node_id)
}

fn focusable_target(renderer: &TuiRenderer, node_id: NodeId) -> Option<NodeId> {
    let mut current = Some(node_id);
    while let Some(id) = current {
        let node = renderer.tree.get(id)?;
        if node.is_focusable() {
            return Some(id);
        }
        current = node.parent;
    }
    None
}
