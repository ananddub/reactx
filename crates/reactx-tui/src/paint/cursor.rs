//! Terminal cursor placement for the focused text widget.

use ratatui::layout::Rect;
use reactx_core::{NodeId, NodeTree};
use rustc_hash::FxHashMap;
use unicode_segmentation::UnicodeSegmentation;

use super::cells::{cell_visible, prefix_width};
use super::layout;
use super::textarea::caret_line_col;
use crate::focus::FocusManager;

/// Where the terminal cursor should be placed for the focused input.
#[allow(clippy::too_many_arguments)]
pub fn cursor_position(
    tree: &NodeTree,
    layout: &FxHashMap<NodeId, Rect>,
    clips: &FxHashMap<NodeId, Rect>,
    offsets: &FxHashMap<NodeId, (i32, i32)>,
    focus: &FocusManager,
    cursors: &FxHashMap<NodeId, usize>,
) -> Option<(u16, u16)> {
    let id = focus.focused_id()?;
    let node = tree.get(id)?;
    if !layout::is_text_input(node) {
        return None;
    }
    let rect = layout.get(&id)?;
    let clip = clips.get(&id).copied()?;
    let (dx, dy) = offsets.get(&id).copied().unwrap_or((0, 0));
    let area = layout::inner_rect(*rect, node);
    if area.width == 0 || area.height == 0 {
        return None;
    }
    let value = node.attrs.value.clone().unwrap_or_default();
    let len = value.graphemes(true).count();
    let cursor = cursors.get(&id).copied().unwrap_or(len).min(len);

    let (x, y) = if layout::is_textarea(node) {
        let (row, column) = caret_line_col(&value, cursor);
        let rows = area.height as usize;
        let columns = area.width as usize;
        let vscroll = row.saturating_sub(rows - 1);
        let hscroll = column.saturating_sub(columns - 1);
        (
            area.x as i32 + dx + (column - hscroll) as i32,
            area.y as i32 + dy + (row - vscroll) as i32,
        )
    } else {
        let cursor_col = if value.is_empty() {
            0
        } else {
            prefix_width(&value, cursor)
        };
        let offset = if value.is_empty() {
            0
        } else {
            cursor_col.saturating_sub(area.width.saturating_sub(1))
        };
        (
            area.x as i32 + dx + cursor_col.saturating_sub(offset) as i32,
            area.y as i32 + dy,
        )
    };

    let x = x.min(area.x as i32 + dx + area.width as i32 - 1);
    let y = y.min(area.y as i32 + dy + area.height as i32 - 1);
    if !cell_visible(x, y, clip) || y < 0 || x < 0 {
        return None;
    }
    Some((x as u16, y as u16))
}
