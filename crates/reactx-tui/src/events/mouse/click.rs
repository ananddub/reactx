//! Click, double-click and context-menu dispatch.

use std::time::{Duration, Instant};

use dioxus_core::VirtualDom;
use reactx_core::{KeyModifiers, MouseButtonKind, NodeId};

use super::{focusable_target, pointer_target};
use crate::events::dispatch::{dispatch_mouse, Pointer};
use crate::events::focus::apply_focus;
use crate::TuiRenderer;

const DOUBLE_CLICK: Duration = Duration::from_millis(400);

pub(crate) fn handle_click(
    dom: &VirtualDom,
    renderer: &mut TuiRenderer,
    x: u16,
    y: u16,
    modifiers: KeyModifiers,
) {
    let Some(node_id) = pointer_target(renderer, x, y) else {
        return;
    };
    focus_at(dom, renderer, node_id);
    let pointer = Pointer::at(x, y, MouseButtonKind::Left, modifiers);
    dispatch_mouse(dom, &renderer.tree, node_id, "click", pointer);
}

pub(crate) fn handle_mouse_down(
    dom: &VirtualDom,
    renderer: &mut TuiRenderer,
    x: u16,
    y: u16,
    button: MouseButtonKind,
    modifiers: KeyModifiers,
) {
    let Some(node_id) = pointer_target(renderer, x, y) else {
        return;
    };
    if button == MouseButtonKind::Left {
        focus_at(dom, renderer, node_id);
    }
    let pointer = Pointer::at(x, y, button, modifiers);
    dispatch_mouse(dom, &renderer.tree, node_id, "mousedown", pointer);
    match button {
        MouseButtonKind::Left => {
            dispatch_mouse(dom, &renderer.tree, node_id, "click", pointer);
            if register_click(renderer, x, y) {
                dispatch_mouse(dom, &renderer.tree, node_id, "dblclick", pointer);
            }
        }
        MouseButtonKind::Right => {
            dispatch_mouse(dom, &renderer.tree, node_id, "contextmenu", pointer);
        }
        MouseButtonKind::Middle => {}
    }
}

fn focus_at(dom: &VirtualDom, renderer: &mut TuiRenderer, node_id: NodeId) {
    if let Some(target) = focusable_target(renderer, node_id) {
        let change = renderer.focus.set_focus(Some(target));
        apply_focus(dom, renderer, change);
    }
}

/// Track the previous left click and report whether this one doubles it.
fn register_click(renderer: &mut TuiRenderer, x: u16, y: u16) -> bool {
    let now = Instant::now();
    let repeated = renderer.last_click.is_some_and(|(then, last_x, last_y)| {
        now.duration_since(then) <= DOUBLE_CLICK
            && x.abs_diff(last_x) <= 1
            && y.abs_diff(last_y) <= 1
    });
    renderer.last_click = if repeated { None } else { Some((now, x, y)) };
    repeated
}
