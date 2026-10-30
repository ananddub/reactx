//! Read-only queries: hit testing, selection state and node rectangles.

use ratatui::layout::Rect;
use reactx_core::NodeId;
use unicode_segmentation::UnicodeSegmentation;

use crate::layout;
use crate::renderer::TuiRenderer;

impl TuiRenderer {
    /// Refresh the hit-test cache from the latest layout pass.
    pub(super) fn update_hit_rects(&mut self) {
        self.hit_rects = self
            .tree
            .focusable_ids_in_paint_order()
            .into_iter()
            .filter_map(|id| self.node_visible_rect(id).map(|rect| (id, rect)))
            .collect();
    }

    /// The visible cell rectangle of a node after scroll offsets and clipping.
    pub fn node_visible_rect(&self, id: NodeId) -> Option<Rect> {
        let rect = *self.layout.get(&id)?;
        let clip = *self.clips.get(&id)?;
        let (dx, dy) = self.offsets.get(&id).copied().unwrap_or((0, 0));
        layout::visible_rect(rect, dx, dy, clip)
    }

    /// Find the topmost focusable node containing the given cell.
    pub fn hit_test(&self, x: u16, y: u16) -> Option<NodeId> {
        self.hit_rects
            .iter()
            .rev()
            .find(|(_, rect)| contains(*rect, x, y))
            .map(|(id, _)| *id)
    }

    /// Find the topmost visible node (focusable or not) containing the cell.
    pub fn node_at(&self, x: u16, y: u16) -> Option<NodeId> {
        self.tree.paint_order().into_iter().rev().find(|id| {
            self.node_visible_rect(*id)
                .map(|rect| contains(rect, x, y))
                .unwrap_or(false)
        })
    }

    pub fn input_cursor(&self, id: NodeId) -> Option<usize> {
        self.input_cursors.get(&id).copied()
    }

    pub fn input_anchor(&self, id: NodeId) -> Option<usize> {
        self.input_anchors.get(&id).copied()
    }

    /// The active selection of a text input as a half-open grapheme range.
    pub fn selection(&self, id: NodeId) -> Option<(usize, usize)> {
        let node = self.tree.get(id)?;
        let len = node
            .attrs
            .value
            .as_deref()
            .unwrap_or_default()
            .graphemes(true)
            .count();
        let anchor = (*self.input_anchors.get(&id)?).min(len);
        let cursor = self.input_cursors.get(&id).copied().unwrap_or(len).min(len);
        (anchor != cursor).then(|| (anchor.min(cursor), anchor.max(cursor)))
    }

    /// Selections for every text input, ready for the painter.
    pub fn selection_ranges(&self) -> rustc_hash::FxHashMap<NodeId, (usize, usize)> {
        self.input_anchors
            .keys()
            .filter_map(|id| self.selection(*id).map(|range| (*id, range)))
            .collect()
    }

    /// Remember the value a text input had when it gained focus so losing
    /// focus can raise `change`.
    pub fn sync_focus_baseline(&mut self) {
        let focused = self
            .focus
            .focused_id()
            .filter(|id| matches!(self.tree.get(*id), Some(node) if layout::is_text_input(node)));
        match focused {
            Some(id) => {
                if self.focus_baseline.as_ref().map(|(node, _)| *node) != Some(id) {
                    let value = self
                        .tree
                        .get(id)
                        .and_then(|node| node.attrs.value.clone())
                        .unwrap_or_default();
                    self.focus_baseline = Some((id, value));
                }
            }
            None => self.focus_baseline = None,
        }
    }
}

fn contains(rect: Rect, x: u16, y: u16) -> bool {
    x >= rect.x
        && y >= rect.y
        && x < rect.x.saturating_add(rect.width)
        && y < rect.y.saturating_add(rect.height)
}
