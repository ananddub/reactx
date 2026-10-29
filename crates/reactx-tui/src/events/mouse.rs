//! Mouse hit testing, hovering and scrolling.

use dioxus_core::VirtualDom;
use reactx_core::{MouseButtonKind, NodeId};

use super::dispatch::{dispatch_mouse, dispatch_wheel};
use super::focus::apply_focus;
use crate::TuiRenderer;

pub(super) fn handle_click(dom: &VirtualDom, renderer: &mut TuiRenderer, x: u16, y: u16) {
    let Some(node_id) = pointer_target(renderer, x, y) else {
        return;
    };
    if let Some(target) = focusable_target(renderer, node_id) {
        let change = renderer.focus.set_focus(Some(target));
        apply_focus(dom, renderer, change);
    }
    dispatch_mouse(
        dom,
        &renderer.tree,
        node_id,
        "click",
        x as f64,
        y as f64,
        MouseButtonKind::Left,
    );
}

pub(super) fn handle_mouse_down(
    dom: &VirtualDom,
    renderer: &mut TuiRenderer,
    x: u16,
    y: u16,
    button: MouseButtonKind,
) {
    let Some(node_id) = pointer_target(renderer, x, y) else {
        return;
    };
    if button == MouseButtonKind::Left {
        if let Some(target) = focusable_target(renderer, node_id) {
            let change = renderer.focus.set_focus(Some(target));
            apply_focus(dom, renderer, change);
        }
    }
    dispatch_mouse(
        dom,
        &renderer.tree,
        node_id,
        "mousedown",
        x as f64,
        y as f64,
        button,
    );
    if button == MouseButtonKind::Left {
        dispatch_mouse(
            dom,
            &renderer.tree,
            node_id,
            "click",
            x as f64,
            y as f64,
            button,
        );
    }
}

pub(super) fn handle_mouse_button(
    dom: &VirtualDom,
    renderer: &mut TuiRenderer,
    x: u16,
    y: u16,
    button: MouseButtonKind,
    name: &str,
) {
    let Some(node_id) = pointer_target(renderer, x, y) else {
        return;
    };
    dispatch_mouse(
        dom,
        &renderer.tree,
        node_id,
        name,
        x as f64,
        y as f64,
        button,
    );
}

pub(super) fn handle_mouse_move(dom: &VirtualDom, renderer: &mut TuiRenderer, x: u16, y: u16) {
    let target = pointer_target(renderer, x, y);
    if renderer.hover != target {
        if let Some(previous) = renderer.hover {
            dispatch_mouse(
                dom,
                &renderer.tree,
                previous,
                "mouseout",
                x as f64,
                y as f64,
                MouseButtonKind::Left,
            );
        }
        if let Some(current) = target {
            dispatch_mouse(
                dom,
                &renderer.tree,
                current,
                "mouseover",
                x as f64,
                y as f64,
                MouseButtonKind::Left,
            );
        }
        renderer.hover = target;
    }
    if let Some(current) = target {
        dispatch_mouse(
            dom,
            &renderer.tree,
            current,
            "mousemove",
            x as f64,
            y as f64,
            MouseButtonKind::Left,
        );
    }
}

/// The topmost node under the pointer that is not outside an active focus trap.
fn pointer_target(renderer: &TuiRenderer, x: u16, y: u16) -> Option<NodeId> {
    let node_id = renderer.node_at(x, y)?;
    if let Some(trap) = renderer.focus.trap() {
        if !renderer.tree.is_within(node_id, trap) {
            return None;
        }
    }
    Some(node_id)
}

fn focusable_target(
    renderer: &TuiRenderer,
    node_id: reactx_core::NodeId,
) -> Option<reactx_core::NodeId> {
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

pub(super) fn handle_scroll(
    dom: &VirtualDom,
    renderer: &mut TuiRenderer,
    x: u16,
    y: u16,
    delta_x: i32,
    delta_y: i32,
) {
    let Some(node_id) = renderer.node_at(x, y) else {
        return;
    };
    if let Some(trap) = renderer.focus.trap() {
        if !renderer.tree.is_within(node_id, trap) {
            return;
        }
    }
    dispatch_wheel(dom, &renderer.tree, node_id, x, y, delta_x, delta_y);
}
