//! Top-level key routing: shortcuts, focus cycling and activation.

use dioxus_core::VirtualDom;
use reactx_core::{MouseButtonKind, TuiKey, TuiKeyEvent};

use super::dispatch::{dispatch_keyboard, dispatch_mouse, Pointer};
use super::focus::apply_focus;
use super::input::handle_input_key;
use super::text::{copy_selection, cut_selection};
use super::Flow;
use crate::TuiRenderer;

pub(super) fn handle_key(dom: &VirtualDom, renderer: &mut TuiRenderer, key: TuiKeyEvent) -> Flow {
    if key.modifiers.ctrl && matches!(key.key, TuiKey::Char('c') | TuiKey::Char('C')) {
        if copy_selection(dom, renderer) {
            return Flow::Continue;
        }
        return Flow::Quit;
    }
    if key.modifiers.ctrl && matches!(key.key, TuiKey::Char('x') | TuiKey::Char('X')) {
        cut_selection(dom, renderer);
        return Flow::Continue;
    }

    match key.key {
        TuiKey::Tab if key.modifiers.shift => {
            let change = renderer.focus.focus_prev();
            apply_focus(dom, renderer, change);
            return Flow::Continue;
        }
        TuiKey::BackTab => {
            let change = renderer.focus.focus_prev();
            apply_focus(dom, renderer, change);
            return Flow::Continue;
        }
        TuiKey::Tab => {
            let change = renderer.focus.focus_next();
            apply_focus(dom, renderer, change);
            return Flow::Continue;
        }
        _ => {}
    }

    let Some(node_id) = renderer
        .focus
        .focused_id()
        .or_else(|| renderer.focus.trap())
    else {
        dispatch_global(dom, renderer, "keydown", &key);
        return Flow::Continue;
    };
    let Some(node) = renderer.tree.get(node_id) else {
        return Flow::Continue;
    };

    let kind = crate::layout::widget_kind(node).to_string();
    let activates = matches!(kind.as_str(), "button" | "checkbox") || node.events.contains("click");
    dispatch_keyboard(dom, &renderer.tree, node_id, "keydown", &key);

    match kind.as_str() {
        "input" | "textarea" => handle_input_key(dom, renderer, node_id, key),
        _ => {
            if activates && matches!(key.key, TuiKey::Enter | TuiKey::Char(' ')) {
                let pointer = Pointer::at(0, 0, MouseButtonKind::Left, key.modifiers);
                dispatch_mouse(dom, &renderer.tree, node_id, "click", pointer);
            }
        }
    }

    Flow::Continue
}

/// Raise `keyup` on the focused element, or globally when nothing is focusable.
pub(super) fn handle_key_up(dom: &VirtualDom, renderer: &TuiRenderer, key: TuiKeyEvent) {
    match renderer
        .focus
        .focused_id()
        .or_else(|| renderer.focus.trap())
    {
        Some(node_id) => dispatch_keyboard(dom, &renderer.tree, node_id, "keyup", &key),
        None => dispatch_global(dom, renderer, "keyup", &key),
    }
}

/// Route a key event to the app's top-level elements when nothing is
/// focusable, so a handler on the root behaves like a document-level one.
fn dispatch_global(dom: &VirtualDom, renderer: &TuiRenderer, name: &str, key: &TuiKeyEvent) {
    let root = renderer.tree.root_id();
    let children = renderer
        .tree
        .get(root)
        .map(|node| node.children.clone())
        .unwrap_or_default();
    for child in children {
        dispatch_keyboard(dom, &renderer.tree, child, name, key);
    }
}
