//! Hover tracking and the enter/leave/over/out event family.

use dioxus_core::VirtualDom;
use reactx_core::{KeyModifiers, MouseButtonKind, NodeId, NodeTree};

use super::pointer_target;
use crate::events::dispatch::{dispatch_mouse, dispatch_mouse_self, Pointer};
use crate::TuiRenderer;

pub(crate) fn handle_mouse_move(
    dom: &VirtualDom,
    renderer: &mut TuiRenderer,
    x: u16,
    y: u16,
    modifiers: KeyModifiers,
) {
    let target = pointer_target(renderer, x, y);
    let pointer = Pointer::at(x, y, MouseButtonKind::Left, modifiers);
    if renderer.hover != target {
        let previous = renderer.hover;
        let previous_chain = previous
            .map(|id| ancestors(&renderer.tree, id))
            .unwrap_or_default();
        let current_chain = target
            .map(|id| ancestors(&renderer.tree, id))
            .unwrap_or_default();
        if let Some(previous) = previous {
            dispatch_mouse(dom, &renderer.tree, previous, "mouseout", pointer);
            for id in previous_chain
                .iter()
                .filter(|id| !current_chain.contains(id))
            {
                dispatch_self(dom, &renderer.tree, *id, "mouseleave", pointer);
            }
        }
        if let Some(current) = target {
            dispatch_mouse(dom, &renderer.tree, current, "mouseover", pointer);
            for id in current_chain
                .iter()
                .filter(|id| !previous_chain.contains(id))
            {
                dispatch_self(dom, &renderer.tree, *id, "mouseenter", pointer);
            }
        }
        renderer.hover = target;
    }
    if let Some(current) = target {
        dispatch_mouse(dom, &renderer.tree, current, "mousemove", pointer);
    }
}

/// Ids from `start` up to the root, deepest first.
fn ancestors(tree: &NodeTree, start: NodeId) -> Vec<NodeId> {
    let mut out = Vec::new();
    let mut current = Some(start);
    while let Some(id) = current {
        let Some(node) = tree.get(id) else {
            break;
        };
        out.push(id);
        current = node.parent;
    }
    out
}

/// Deliver a non-bubbling event to one node when it listens for it.
fn dispatch_self(dom: &VirtualDom, tree: &NodeTree, id: NodeId, name: &str, pointer: Pointer) {
    if tree.get(id).is_some_and(|node| node.events.contains(name)) {
        dispatch_mouse_self(dom, tree, id, name, pointer);
    }
}
