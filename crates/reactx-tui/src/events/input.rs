//! Caret movement and value editing inside input and textarea widgets.

use dioxus_core::VirtualDom;
use reactx_core::{NodeId, TuiKey, TuiKeyEvent};
use unicode_segmentation::UnicodeSegmentation;

use super::dispatch::dispatch_form;
use crate::TuiRenderer;

pub(super) fn handle_input_key(
    dom: &VirtualDom,
    renderer: &mut TuiRenderer,
    node_id: NodeId,
    key: TuiKeyEvent,
) {
    let Some(node) = renderer.tree.get(node_id) else {
        return;
    };
    let multiline = crate::layout::is_textarea(node);
    let value = node.attrs.value.clone().unwrap_or_default();
    let mut graphemes: Vec<String> = value.graphemes(true).map(str::to_string).collect();
    let len = graphemes.len();
    let mut cursor = renderer
        .input_cursors
        .get(&node_id)
        .copied()
        .unwrap_or(len)
        .min(len);
    let mut anchor = renderer.input_anchors.get(&node_id).copied();
    let mut changed = false;
    let shift = key.modifiers.shift;

    if key.modifiers.ctrl && matches!(key.key, TuiKey::Char('a') | TuiKey::Char('A')) {
        if !graphemes.is_empty() {
            renderer.input_anchors.insert(node_id, 0);
            renderer.input_cursors.insert(node_id, len);
        }
        return;
    }

    let selection = anchor
        .filter(|anchor| *anchor != cursor)
        .map(|anchor| (anchor.min(cursor), anchor.max(cursor)));

    match key.key {
        TuiKey::Char(ch) if !key.modifiers.ctrl && !key.modifiers.alt && !ch.is_control() => {
            if let Some((start, end)) = selection {
                graphemes.drain(start..end);
                cursor = start;
            }
            graphemes.insert(cursor, ch.to_string());
            cursor += 1;
            anchor = None;
            changed = true;
        }
        TuiKey::Enter if multiline && !key.modifiers.ctrl => {
            if let Some((start, end)) = selection {
                graphemes.drain(start..end);
                cursor = start;
            }
            graphemes.insert(cursor, "\n".to_string());
            cursor += 1;
            anchor = None;
            changed = true;
        }
        TuiKey::Backspace => {
            if let Some((start, end)) = selection {
                graphemes.drain(start..end);
                cursor = start;
                anchor = None;
                changed = true;
            } else if cursor > 0 {
                graphemes.remove(cursor - 1);
                cursor -= 1;
                changed = true;
            }
        }
        TuiKey::Delete => {
            if let Some((start, end)) = selection {
                graphemes.drain(start..end);
                cursor = start;
                anchor = None;
                changed = true;
            } else if cursor < graphemes.len() {
                graphemes.remove(cursor);
                changed = true;
            }
        }
        TuiKey::Left => {
            let target = cursor.saturating_sub(1);
            cursor = move_caret(&mut anchor, shift, cursor, target);
        }
        TuiKey::Right => {
            let target = (cursor + 1).min(graphemes.len());
            cursor = move_caret(&mut anchor, shift, cursor, target);
        }
        TuiKey::Home => {
            let target = if multiline {
                line_bounds(&graphemes, cursor).0
            } else {
                0
            };
            cursor = move_caret(&mut anchor, shift, cursor, target);
        }
        TuiKey::End => {
            let target = if multiline {
                line_bounds(&graphemes, cursor).1
            } else {
                graphemes.len()
            };
            cursor = move_caret(&mut anchor, shift, cursor, target);
        }
        TuiKey::Up if multiline => {
            let target = vertical_target(&graphemes, cursor, -1);
            cursor = move_caret(&mut anchor, shift, cursor, target);
        }
        TuiKey::Down if multiline => {
            let target = vertical_target(&graphemes, cursor, 1);
            cursor = move_caret(&mut anchor, shift, cursor, target);
        }
        _ => {}
    }

    renderer.input_cursors.insert(node_id, cursor);
    match anchor {
        Some(anchor) => {
            renderer.input_anchors.insert(node_id, anchor);
        }
        None => {
            renderer.input_anchors.remove(&node_id);
        }
    }
    if changed {
        let next = graphemes.concat();
        dispatch_form(dom, &renderer.tree, node_id, "input", next.clone());
        // Batched keys arrive before the next render, so keep the tree value
        // current or every key would restart from the value of the last frame.
        if let Some(node) = renderer.tree.get_mut(node_id) {
            node.attrs.value = Some(next);
        }
    }
}

fn move_caret(anchor: &mut Option<usize>, shift: bool, from: usize, to: usize) -> usize {
    if shift {
        anchor.get_or_insert(from);
        to
    } else {
        *anchor = None;
        to
    }
}

fn line_bounds(graphemes: &[String], cursor: usize) -> (usize, usize) {
    let start = graphemes[..cursor]
        .iter()
        .rposition(|grapheme| grapheme == "\n")
        .map(|index| index + 1)
        .unwrap_or(0);
    let end = graphemes[cursor..]
        .iter()
        .position(|grapheme| grapheme == "\n")
        .map(|index| cursor + index)
        .unwrap_or(graphemes.len());
    (start, end)
}

/// Move one visual line up or down, keeping the column where possible.
fn vertical_target(graphemes: &[String], cursor: usize, delta: isize) -> usize {
    let (start, end) = line_bounds(graphemes, cursor);
    let column = cursor - start;
    let (target_start, target_end) = if delta < 0 {
        if start == 0 {
            return cursor;
        }
        let previous_end = start - 1;
        let previous_start = graphemes[..previous_end]
            .iter()
            .rposition(|grapheme| grapheme == "\n")
            .map(|index| index + 1)
            .unwrap_or(0);
        (previous_start, previous_end)
    } else {
        if end >= graphemes.len() {
            return cursor;
        }
        let next_start = end + 1;
        let next_end = graphemes[next_start..]
            .iter()
            .position(|grapheme| grapheme == "\n")
            .map(|index| next_start + index)
            .unwrap_or(graphemes.len());
        (next_start, next_end)
    };
    (target_start + column).min(target_end)
}
