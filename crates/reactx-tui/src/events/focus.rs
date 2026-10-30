//! Focus transitions and blur/change events.

use dioxus_core::VirtualDom;
use unicode_segmentation::UnicodeSegmentation;

use super::dispatch::{dispatch_focus, dispatch_form};
use crate::focus::FocusChange;
use crate::TuiRenderer;

pub(crate) fn apply_focus(dom: &VirtualDom, renderer: &mut TuiRenderer, change: FocusChange) {
    if !change.is_change() {
        return;
    }
    if let Some(previous) = change.previous {
        let was_text = matches!(
            renderer.tree.get(previous),
            Some(node) if crate::layout::is_text_input(node)
        );
        if was_text {
            if let Some((id, before)) = renderer.focus_baseline.take() {
                if id == previous {
                    let now = renderer
                        .tree
                        .get(previous)
                        .and_then(|node| node.attrs.value.clone())
                        .unwrap_or_default();
                    if now != before {
                        dispatch_form(dom, &renderer.tree, previous, "change", now);
                    }
                }
            }
        }
        renderer.input_anchors.remove(&previous);
        if let Some(element) = renderer.tree.element_for_node(previous) {
            dispatch_focus(dom, element, "blur");
        }
    }
    if let Some(current) = change.current {
        if let Some(node) = renderer.tree.get(current) {
            if crate::layout::is_text_input(node) {
                let value = node.attrs.value.clone().unwrap_or_default();
                renderer
                    .input_cursors
                    .entry(current)
                    .or_insert(value.graphemes(true).count());
                renderer.focus_baseline = Some((current, value));
            }
        }
        if let Some(element) = renderer.tree.element_for_node(current) {
            dispatch_focus(dom, element, "focus");
        }
    }
}
