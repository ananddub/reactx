//! Event dispatch helpers shared by the keyboard and mouse handlers.

use std::any::Any;
use std::rc::Rc;

use dioxus_core::{ElementId, Event, VirtualDom};
use dioxus_html::PlatformEventData;
use reactx_core::{KeyModifiers, MouseButtonKind, NodeId, NodeTree, TuiKeyEvent};

use super::convert::{map_key, map_modifiers};
use super::data::{TuiFocusEvent, TuiFormEvent, TuiKeyboardEvent, TuiMouseEvent, TuiWheelEvent};

/// Dispatch an event and report whether its default action is still enabled,
/// so callers can honor `event.prevent_default()` called by a listener.
pub(super) fn dispatch(
    dom: &VirtualDom,
    element: ElementId,
    name: &str,
    data: Rc<dyn Any>,
    bubbles: bool,
) -> bool {
    let event = Event::new(data, bubbles);
    let probe = event.clone();
    dom.runtime().handle_event(name, event, element);
    probe.default_action_enabled()
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
    dispatch_chain_default(dom, tree, start, name, data);
}

/// Like [`dispatch_chain`], but also reports whether the default action is
/// still enabled after the listeners ran (`true` when nothing listened).
pub(super) fn dispatch_chain_default(
    dom: &VirtualDom,
    tree: &NodeTree,
    start: NodeId,
    name: &str,
    data: Rc<dyn Any>,
) -> bool {
    let mut current = Some(start);
    while let Some(id) = current {
        let Some(node) = tree.get(id) else {
            return true;
        };
        if node.events.contains(name) {
            let Some(element) = tree.element_for_node(id) else {
                return true;
            };
            return dispatch(dom, element, name, data, true);
        }
        current = node.parent;
    }
    true
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

/// Pointer state carried by every mouse event.
#[derive(Clone, Copy)]
pub(super) struct Pointer {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) button: MouseButtonKind,
    pub(super) modifiers: KeyModifiers,
}

impl Pointer {
    pub(super) fn at(x: u16, y: u16, button: MouseButtonKind, modifiers: KeyModifiers) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
            button,
            modifiers,
        }
    }

    fn data(self) -> Rc<dyn Any> {
        Rc::new(PlatformEventData::new(Box::new(TuiMouseEvent {
            x: self.x,
            y: self.y,
            modifiers: map_modifiers(self.modifiers),
            button: self.button,
        })))
    }
}

pub(super) fn dispatch_mouse(
    dom: &VirtualDom,
    tree: &NodeTree,
    node: NodeId,
    name: &str,
    pointer: Pointer,
) {
    dispatch_chain(dom, tree, node, name, pointer.data());
}

/// Dispatch a mouse event to one element only, without bubbling
/// (`mouseenter`/`mouseleave` follow browser semantics here).
pub(super) fn dispatch_mouse_self(
    dom: &VirtualDom,
    tree: &NodeTree,
    node: NodeId,
    name: &str,
    pointer: Pointer,
) {
    if let Some(element) = tree.element_for_node(node) {
        dispatch(dom, element, name, pointer.data(), false);
    }
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

#[derive(Clone, Copy)]
pub(super) struct WheelDelta {
    pub(super) x: i32,
    pub(super) y: i32,
}

pub(super) fn dispatch_wheel(
    dom: &VirtualDom,
    tree: &NodeTree,
    node: NodeId,
    x: u16,
    y: u16,
    delta: WheelDelta,
    modifiers: KeyModifiers,
) {
    let data = PlatformEventData::new(Box::new(TuiWheelEvent {
        x: x as f64,
        y: y as f64,
        delta_x: delta.x as f64,
        delta_y: delta.y as f64,
        modifiers: map_modifiers(modifiers),
    }));
    dispatch_chain(dom, tree, node, "wheel", Rc::new(data));
}

pub(super) fn dispatch_focus(dom: &VirtualDom, element: ElementId, name: &str) {
    let data = PlatformEventData::new(Box::new(TuiFocusEvent));
    dispatch(dom, element, name, Rc::new(data), false);
}
