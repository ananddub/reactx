//! Ratatui paint pass: backgrounds, borders, text, widgets and the cursor.

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier};
use ratatui::Frame;
use reactx_core::{NodeId, NodeTree, TextAlign, TuiColor};
use rustc_hash::FxHashMap;

use crate::focus::FocusManager;
use crate::layout;

mod border;
mod cells;
mod cursor;
mod input;
mod node;
mod scratch;
mod text;
mod textarea;

pub use cursor::cursor_position;

#[derive(Debug, Clone, Copy, Default)]
struct Inherited {
    fg: Option<Color>,
    modifier: Modifier,
    align: TextAlign,
}

/// Shared, read-only inputs for one paint pass.
struct Context<'a> {
    tree: &'a NodeTree,
    layout: &'a FxHashMap<NodeId, Rect>,
    clips: &'a FxHashMap<NodeId, Rect>,
    offsets: &'a FxHashMap<NodeId, (i32, i32)>,
    cursors: &'a FxHashMap<NodeId, usize>,
    selections: &'a FxHashMap<NodeId, (usize, usize)>,
    focus: &'a FocusManager,
    focus_color: Color,
}

/// Paint the tree into the frame buffer.
#[allow(clippy::too_many_arguments)]
pub fn render(
    frame: &mut Frame,
    tree: &NodeTree,
    layout: &FxHashMap<NodeId, Rect>,
    clips: &FxHashMap<NodeId, Rect>,
    offsets: &FxHashMap<NodeId, (i32, i32)>,
    cursors: &FxHashMap<NodeId, usize>,
    selections: &FxHashMap<NodeId, (usize, usize)>,
    focus: &FocusManager,
    focus_color: TuiColor,
    cursor: Option<(u16, u16)>,
) {
    if let Some((x, y)) = cursor {
        frame.set_cursor_position((x, y));
    }
    let area = frame.area();
    let buf = frame.buffer_mut();
    let ctx = Context {
        tree,
        layout,
        clips,
        offsets,
        cursors,
        selections,
        focus,
        focus_color: focus_color.to_ratatui(),
    };
    node::paint(&ctx, buf, tree.root_id(), area, Inherited::default());
}
