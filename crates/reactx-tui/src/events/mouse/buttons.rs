//! Button release, drag and wheel dispatch.

use dioxus_core::VirtualDom;
use reactx_core::{KeyModifiers, MouseButtonKind};

use super::pointer_target;
use crate::events::dispatch::{dispatch_mouse, dispatch_wheel, Pointer, WheelDelta};
use crate::TuiRenderer;

pub(crate) fn handle_mouse_button(
    dom: &VirtualDom,
    renderer: &mut TuiRenderer,
    x: u16,
    y: u16,
    button: MouseButtonKind,
    modifiers: KeyModifiers,
    name: &str,
) {
    let Some(node_id) = pointer_target(renderer, x, y) else {
        return;
    };
    let pointer = Pointer::at(x, y, button, modifiers);
    dispatch_mouse(dom, &renderer.tree, node_id, name, pointer);
}

pub(crate) fn handle_scroll(
    dom: &VirtualDom,
    renderer: &mut TuiRenderer,
    x: u16,
    y: u16,
    delta_x: i32,
    delta_y: i32,
    modifiers: KeyModifiers,
) {
    let Some(node_id) = pointer_target(renderer, x, y) else {
        return;
    };
    let delta = WheelDelta {
        x: delta_x,
        y: delta_y,
    };
    dispatch_wheel(dom, &renderer.tree, node_id, x, y, delta, modifiers);
}
