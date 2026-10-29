//! Selection, clipboard events and paste handling for text fields.

use std::any::Any;
use std::rc::Rc;

use dioxus_core::VirtualDom;
use dioxus_html::{HasClipboardData, PlatformEventData};
use reactx_core::{NodeId, NodeTree};

use super::dispatch::{dispatch_chain_default, dispatch_form};
use crate::TuiRenderer;

/// Payload of `oncopy`, `oncut` and `onpaste`.
///
/// Read the clipboard text from a handler with
/// `event.data().downcast::<TuiClipboardEvent>()`.
#[derive(Clone)]
pub struct TuiClipboardEvent {
    pub text: Option<String>,
}

impl HasClipboardData for TuiClipboardEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Raise a clipboard event on the node and report whether handlers left the
/// default clipboard action enabled.
fn dispatch_clipboard(
    dom: &VirtualDom,
    tree: &NodeTree,
    node: NodeId,
    name: &str,
    text: Option<String>,
) -> bool {
    let data = PlatformEventData::new(Box::new(TuiClipboardEvent { text }));
    dispatch_chain_default(dom, tree, node, name, Rc::new(data))
}

fn focused_text_node(renderer: &TuiRenderer) -> Option<NodeId> {
    let id = renderer.focus.focused_id()?;
    let node = renderer.tree.get(id)?;
    crate::layout::is_text_input(node).then_some(id)
}

fn selected_text(renderer: &TuiRenderer, id: NodeId) -> Option<String> {
    let (start, end) = renderer.selection(id)?;
    let value = renderer
        .tree
        .get(id)?
        .attrs
        .value
        .clone()
        .unwrap_or_default();
    Some(value.chars().skip(start).take(end - start).collect())
}

/// Copy the focused selection after `oncopy` runs. Returns false when Ctrl+C
/// should fall through to quitting: nothing was selected and no handler
/// consumed the event.
pub(super) fn copy_selection(dom: &VirtualDom, renderer: &mut TuiRenderer) -> bool {
    let Some(id) = renderer.focus.focused_id() else {
        return false;
    };
    let is_text = renderer
        .tree
        .get(id)
        .is_some_and(crate::layout::is_text_input);
    let text = if is_text {
        selected_text(renderer, id)
    } else {
        None
    };
    if !dispatch_clipboard(dom, &renderer.tree, id, "copy", text.clone()) {
        return true;
    }
    match text {
        Some(text) => {
            crate::clipboard::copy(&text);
            renderer.clipboard = Some(text);
            true
        }
        None => false,
    }
}

/// Copy and delete the focused input's selection after `oncut` runs.
pub(super) fn cut_selection(dom: &VirtualDom, renderer: &mut TuiRenderer) {
    let Some(id) = focused_text_node(renderer) else {
        return;
    };
    let Some((start, end)) = renderer.selection(id) else {
        return;
    };
    let text = selected_text(renderer, id);
    if !dispatch_clipboard(dom, &renderer.tree, id, "cut", text.clone()) {
        return;
    }
    if let Some(text) = text {
        crate::clipboard::copy(&text);
        renderer.clipboard = Some(text);
    }
    let value = renderer
        .tree
        .get(id)
        .and_then(|node| node.attrs.value.clone())
        .unwrap_or_default();
    let mut chars: Vec<char> = value.chars().collect();
    chars.drain(start..end);
    let next: String = chars.into_iter().collect();
    renderer.input_cursors.insert(id, start);
    renderer.input_anchors.remove(&id);
    dispatch_form(dom, &renderer.tree, id, "input", next);
}

pub(super) fn paste(dom: &VirtualDom, renderer: &mut TuiRenderer, text: &str) {
    let Some(node_id) = renderer.focus.focused_id() else {
        return;
    };
    let Some(node) = renderer.tree.get(node_id) else {
        return;
    };
    let is_text = crate::layout::is_text_input(node);
    let multiline = crate::layout::is_textarea(node);
    if !dispatch_clipboard(
        dom,
        &renderer.tree,
        node_id,
        "paste",
        Some(text.to_string()),
    ) || !is_text
    {
        return;
    }
    let mut chars: Vec<char> = node
        .attrs
        .value
        .clone()
        .unwrap_or_default()
        .chars()
        .collect();
    let mut cursor = renderer
        .input_cursors
        .get(&node_id)
        .copied()
        .unwrap_or(chars.len())
        .min(chars.len());
    if let Some((start, end)) = renderer.selection(node_id) {
        chars.drain(start..end);
        cursor = start;
    }
    let pasted: Vec<char> = text
        .chars()
        .filter(|ch| (*ch == '\n' && multiline) || (!ch.is_control() && *ch != '\r'))
        .collect();
    for ch in pasted.iter().rev() {
        chars.insert(cursor, *ch);
    }
    cursor += pasted.len();
    renderer.input_cursors.insert(node_id, cursor);
    renderer.input_anchors.remove(&node_id);
    let next: String = chars.into_iter().collect();
    dispatch_form(dom, &renderer.tree, node_id, "input", next);
}
