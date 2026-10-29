//! The headless renderer state and its queries.

use std::io;

use dioxus_core::ElementId;
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::Terminal;
use reactx_core::{NodeId, NodeTree};
use rustc_hash::FxHashMap;

use crate::focus::FocusManager;
use crate::{layout, paint};

mod attrs;
mod mutations;
mod query;
mod tree_ops;

pub use attrs::normalize_tag;

/// The headless renderer state. A terminal handle is passed in for each frame so
/// the same renderer can be driven by Crossterm, a test backend, or another sink.
pub struct TuiRenderer {
    pub tree: NodeTree,
    pub focus: FocusManager,
    /// Absolute cell rectangles from the most recent layout pass.
    pub layout: FxHashMap<NodeId, Rect>,
    /// Visible clip region of every node from the most recent layout pass.
    pub clips: FxHashMap<NodeId, Rect>,
    /// Signed scroll offsets accumulated from clipping ancestors.
    pub offsets: FxHashMap<NodeId, (i32, i32)>,
    /// Focusable node rectangles in paint order.
    pub hit_rects: Vec<(NodeId, Rect)>,
    pub area: Rect,
    pub cursor: Option<(u16, u16)>,
    /// Caret position (in chars) for each text input node.
    pub input_cursors: FxHashMap<NodeId, usize>,
    /// Selection anchor (in chars) for text inputs with an active selection.
    pub input_anchors: FxHashMap<NodeId, usize>,
    /// The node currently under the pointer, used for `mouseover`/`mouseout`.
    pub hover: Option<NodeId>,
    /// Value of the focused text input when it gained focus, used to raise
    /// `change` when it loses focus after an edit.
    pub focus_baseline: Option<(NodeId, String)>,
    /// The last text copied to the clipboard (also sent as OSC 52).
    pub clipboard: Option<String>,
    /// Accent used for focused borders and text.
    pub focus_color: reactx_core::TuiColor,
    stack: Vec<NodeId>,
}

impl Default for TuiRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiRenderer {
    pub fn new() -> Self {
        Self {
            tree: NodeTree::new(),
            focus: FocusManager::default(),
            layout: FxHashMap::default(),
            clips: FxHashMap::default(),
            offsets: FxHashMap::default(),
            hit_rects: Vec::new(),
            area: Rect::new(0, 0, 0, 0),
            cursor: None,
            input_cursors: FxHashMap::default(),
            input_anchors: FxHashMap::default(),
            hover: None,
            focus_baseline: None,
            clipboard: None,
            focus_color: reactx_core::TuiColor::Rgb(137, 180, 250),
            stack: Vec::new(),
        }
    }

    /// Recompute layout, rebuild the focus ring and draw one frame.
    pub fn paint<T: Backend>(&mut self, terminal: &mut Terminal<T>) -> io::Result<()> {
        let size = terminal.size()?;
        self.area = Rect::new(0, 0, size.width, size.height);
        self.recompute();

        let tree = &self.tree;
        let layout = &self.layout;
        let clips = &self.clips;
        let offsets = &self.offsets;
        let cursors = &self.input_cursors;
        let selections = self.selection_ranges();
        let focus = &self.focus;
        let cursor = self.cursor;
        let focus_color = self.focus_color;
        terminal.draw(|frame| {
            paint::render(
                frame,
                tree,
                layout,
                clips,
                offsets,
                cursors,
                &selections,
                focus,
                focus_color,
                cursor,
            )
        })?;
        Ok(())
    }

    /// Layout and paint without a terminal (useful for hit testing/tests).
    pub fn layout_only(&mut self, area: Rect) {
        self.area = area;
        self.recompute();
    }

    fn recompute(&mut self) {
        self.focus.rebuild(&self.tree);
        let result = layout::compute_full(&self.tree, self.area);
        self.layout = result.rects;
        self.clips = result.clips;
        self.offsets = result.offsets;
        self.input_cursors.retain(
            |id, _| matches!(self.tree.get(*id), Some(node) if layout::is_text_input(node)),
        );
        self.input_anchors.retain(
            |id, _| matches!(self.tree.get(*id), Some(node) if layout::is_text_input(node)),
        );
        if self.hover.is_some_and(|id| self.tree.get(id).is_none()) {
            self.hover = None;
        }
        if self
            .focus_baseline
            .as_ref()
            .is_some_and(|(id, _)| self.tree.get(*id).is_none())
        {
            self.focus_baseline = None;
        }
        self.update_hit_rects();
        self.cursor = paint::cursor_position(
            &self.tree,
            &self.layout,
            &self.clips,
            &self.offsets,
            &self.focus,
            &self.input_cursors,
        );
    }

    fn update_hit_rects(&mut self) {
        self.hit_rects = self
            .tree
            .focusable_ids_in_paint_order()
            .into_iter()
            .filter_map(|id| self.node_visible_rect(id).map(|rect| (id, rect)))
            .collect();
    }

    pub fn focused_element(&self) -> Option<ElementId> {
        self.focus
            .focused_id()
            .and_then(|node| self.tree.element_for_node(node))
    }

    pub fn node_dioxus_id(&self, node: NodeId) -> Option<ElementId> {
        self.tree.element_for_node(node)
    }
}
