//! Event dispatch helpers shared by the keyboard and mouse handlers.

use std::any::Any;
use std::rc::Rc;

use dioxus_core::{ElementId, Event, VirtualDom};
use dioxus_html::input_data::keyboard_types::Modifiers;
use dioxus_html::PlatformEventData;
use reactx_core::{MouseButtonKind, NodeId, NodeTree, TuiKeyEvent};

use super::convert::{map_key, map_modifiers};
use super::data::{TuiFocusEvent, TuiFormEvent, TuiKeyboardEvent, TuiMouseEvent, TuiWheelEvent};

pub(super) fn dispatch(
    dom: &VirtualDom,
    element: ElementId,
    name: &str,
    data: Rc<dyn Any>,
    bubbles: bool,
) {
    dom.runtime()
        .handle_event(name, Event::new(data, bubbles), element);
}

/// Dispatch an event to the nearest node at or above `start` that listens for
/// `name`, then let Dioxus bubble from that live element.
///
/// The backend's mount tree does not always line up with the node tree (text
/// leaves inside templates in particular), so starting the dispatch at the
/// hit element can drop the event entirely. Starting at the element that owns
/// the listener keeps bubbling and `stop_propagation()` working.
pub(super) fn dispatch_chain(
    dom: &VirtualDom,
    tree: &NodeTree,
    start: NodeId,
    name: &str,
    data: Rc<dyn Any>,
) {
    let mut current = Some(start);
    while let Some(id) = current {
        let Some(node) = tree.get(id) else {
            return;
        };
        if node.events.contains(name) {
            if let Some(element) = tree.element_for_node(id) {
                dispatch(dom, element, name, data, true);
            }
            return;
        }
        current = node.parent;
    }
}

pub(super) fn dispatch_form(
    dom: &VirtualDom,
    tree: &NodeTree,
    node: NodeId,
    name: &str,
    value: String,
) {
    let data = PlatformEventData::new(Box::new(TuiFormEvent { value }));
    dispatch_chain(dom, tree, node, name, Rc::new(data));
}

pub(super) fn dispatch_mouse(
    dom: &VirtualDom,
    tree: &NodeTree,
    node: NodeId,
    name: &str,
    x: f64,
    y: f64,
    button: MouseButtonKind,
) {
    let data = PlatformEventData::new(Box::new(TuiMouseEvent {
        x,
        y,
        modifiers: Modifiers::empty(),
        button,
    }));
    dispatch_chain(dom, tree, node, name, Rc::new(data));
}

pub(super) fn dispatch_keyboard(
    dom: &VirtualDom,
    tree: &NodeTree,
    node: NodeId,
    name: &str,
    event: &TuiKeyEvent,
) {
    let (key, code) = map_key(&event.key);
    let data = PlatformEventData::new(Box::new(TuiKeyboardEvent {
        key,
        code,
        modifiers: map_modifiers(event.modifiers),
    }));
    dispatch_chain(dom, tree, node, name, Rc::new(data));
}

pub(super) fn dispatch_wheel(
    dom: &VirtualDom,
    tree: &NodeTree,
    node: NodeId,
    x: u16,
    y: u16,
    delta_x: i32,
    delta_y: i32,
) {
    let data = PlatformEventData::new(Box::new(TuiWheelEvent {
        x: x as f64,
        y: y as f64,
        delta_x: delta_x as f64,
        delta_y: delta_y as f64,
    }));
    dispatch_chain(dom, tree, node, "wheel", Rc::new(data));
}

pub(super) fn dispatch_focus(dom: &VirtualDom, element: ElementId, name: &str) {
    let data = PlatformEventData::new(Box::new(TuiFocusEvent));
    dispatch(dom, element, name, Rc::new(data), false);
}
