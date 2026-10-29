//! Selection, clipboard and paste handling for text fields.

use dioxus_core::VirtualDom;
use reactx_core::NodeId;

use super::dispatch::dispatch_form;
use crate::TuiRenderer;

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

/// Copy the focused input's selection. Returns false when there is none.
pub(super) fn copy_selection(renderer: &mut TuiRenderer) -> bool {
    let Some(id) = focused_text_node(renderer) else {
        return false;
    };
    let Some(text) = selected_text(renderer, id) else {
        return false;
    };
    crate::clipboard::copy(&text);
    renderer.clipboard = Some(text);
    true
}

/// Copy and delete the focused input's selection.
pub(super) fn cut_selection(dom: &VirtualDom, renderer: &mut TuiRenderer) {
    let Some(id) = focused_text_node(renderer) else {
        return;
    };
    let Some((start, end)) = renderer.selection(id) else {
        return;
    };
    if let Some(text) = selected_text(renderer, id) {
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
    if !crate::layout::is_text_input(node) {
        return;
    }
    let multiline = crate::layout::is_textarea(node);
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
