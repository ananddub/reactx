//! The headless renderer state and its queries.

use std::io;
use std::time::Instant;

use dioxus_core::ElementId;
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::Terminal;
use reactx_core::{NodeId, NodeTree};
use rustc_hash::FxHashMap;

use crate::effects::Effects;
use crate::focus::FocusManager;
use crate::{layout, paint};

mod attrs;
mod classes;
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
    /// Time and cell of the previous left click, for double-click detection.
    pub last_click: Option<(Instant, u16, u16)>,
    /// Value of the focused text input when it gained focus, used to raise
    /// `change` when it loses focus after an edit.
    pub focus_baseline: Option<(NodeId, String)>,
    /// The last text copied to the clipboard (also sent as OSC 52).
    pub clipboard: Option<String>,
    /// Accent used for focused borders and text.
    pub focus_color: reactx_core::TuiColor,
    /// Screen effects applied after each frame is painted.
    pub effects: Effects,
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
            last_click: None,
            focus_baseline: None,
            clipboard: None,
            focus_color: reactx_core::TuiColor::Rgb(137, 180, 250),
            effects: Effects::default(),
            stack: Vec::new(),
        }
    }

    /// Recompute layout, rebuild the focus ring and draw one frame.
    ///
    /// The layout uses the frame's viewport, which is the whole screen for a
    /// fullscreen terminal and the live region for an inline one.
    pub fn paint<T: Backend>(&mut self, terminal: &mut Terminal<T>) -> io::Result<()> {
        terminal.draw(|frame| {
            self.area = frame.area();
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
            let effects = &mut self.effects;
            effects.begin_frame(frame);
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
            );
            effects.end_frame(frame);
        })?;
        Ok(())
    }

    /// Whether screen effects still need another frame.
    pub fn effects_active(&self) -> bool {
        self.effects.is_active()
    }

    /// Layout and paint without a terminal (useful for hit testing/tests).
    pub fn layout_only(&mut self, area: Rect) {
        self.area = area;
        self.recompute();
    }

    fn recompute(&mut self) {
        let focused_before = self.focus.focused_id();
        self.rebuild_styles();
        self.focus.rebuild(&self.tree);
        if self.focus.focused_id() != focused_before {
            self.rebuild_styles();
        }
        self.tree.apply_transforms();
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
        self.effects.sync_nodes(
            &self.tree,
            &self.layout,
            &self.clips,
            &self.offsets,
            self.area,
        );
        self.cursor = match crate::runtime::cursor::current() {
            crate::runtime::cursor::CursorOverride::Auto => paint::cursor_position(
                &self.tree,
                &self.layout,
                &self.clips,
                &self.offsets,
                &self.focus,
                &self.input_cursors,
            ),
            crate::runtime::cursor::CursorOverride::Hidden => None,
            crate::runtime::cursor::CursorOverride::At(x, y) => Some((x, y)),
        };
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
