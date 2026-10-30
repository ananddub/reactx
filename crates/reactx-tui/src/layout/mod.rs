//! Flexbox layout via Taffy.
//!
//! A fresh Taffy tree is built from the terminal DOM each frame. Terminal
//! coordinates are rounded to whole cells and stored as absolute rectangles.

use ratatui::layout::Rect;
use reactx_core::{NodeId, NodeTree};
use rustc_hash::FxHashMap;
use taffy::prelude::*;
use taffy::TaffyTree;

mod build;
mod collect;
mod geometry;
mod leaf;
mod metrics;
mod style;
mod text;
mod wrap;

pub(crate) use geometry::{inner_rect, visible_rect};
pub(crate) use leaf::{is_text_input, is_textarea};
pub(crate) use metrics::effective_border;
pub(crate) use text::checkbox_text;
pub use text::{node_label, str_width, widget_kind, wrap};
pub(crate) use wrap::wrap_mode;

use build::{build, measure_leaf};
use collect::collect_rects;
use leaf::Leaf;
use style::taffy_style;

/// Absolute cell rectangles and clipping regions.
#[derive(Debug, Clone, Default)]
pub struct LayoutResult {
    /// Absolute rectangle of every laid-out node, before scroll offsets.
    pub rects: FxHashMap<NodeId, Rect>,
    /// Visible region (ancestor clips) for every laid-out node.
    pub clips: FxHashMap<NodeId, Rect>,
    /// Signed scroll offset inherited from clipping ancestors.
    pub offsets: FxHashMap<NodeId, (i32, i32)>,
}

/// Compute absolute cell rectangles for every node under `area`.
pub fn compute(tree: &NodeTree, area: Rect) -> FxHashMap<NodeId, Rect> {
    compute_full(tree, area).rects
}

/// Compute rectangles plus the visible clip region of every node.
pub fn compute_full(tree: &NodeTree, area: Rect) -> LayoutResult {
    let mut taffy: TaffyTree<Leaf> = TaffyTree::new();
    let mut mapping: FxHashMap<taffy::NodeId, NodeId> = FxHashMap::default();

    let root = tree.root_id();
    let root_style = taffy_style(tree.get(root).unwrap(), false, true);
    let root_taffy = build(&mut taffy, &mut mapping, tree, root, root_style);

    let available = Size {
        width: AvailableSpace::Definite(area.width as f32),
        height: AvailableSpace::Definite(area.height as f32),
    };

    let _ = taffy.compute_layout_with_measure(
        root_taffy,
        available,
        |known, available, _id, context, _style| match context {
            Some(leaf) => measure_leaf(leaf, known, available),
            None => Size {
                width: known.width.unwrap_or(0.0),
                height: known.height.unwrap_or(0.0),
            },
        },
    );

    let mut out = LayoutResult::default();
    collect_rects(
        &taffy,
        &mapping,
        tree,
        root_taffy,
        area.x as f32,
        area.y as f32,
        0.0,
        0.0,
        area,
        &mut out,
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use reactx_core::{BorderKind, NodeTree, StyleProps, TuiNode};

    fn styled(tag: &str, attrs: StyleProps) -> TuiNode {
        TuiNode {
            tag: tag.to_string(),
            attrs,
            ..Default::default()
        }
    }

    #[test]
    fn wrap_respects_words_and_long_words() {
        assert_eq!(wrap("hello world", Some(5)), vec!["hello", "world"]);
        assert_eq!(wrap("abcdefgh", Some(3)), vec!["abc", "def", "gh"]);
        assert_eq!(wrap("a\nb", None), vec!["a", "b"]);
    }

    #[test]
    fn borders_add_layout_space() {
        let mut tree = NodeTree::new();
        let root = tree.root_id();
        let attrs = StyleProps {
            border: Some(BorderKind::Rounded),
            ..StyleProps::default()
        };
        let node = tree.insert_node("box", None);
        tree.get_mut(node).unwrap().attrs = attrs;
        tree.append_child(root, node);

        let layout = compute(&tree, Rect::new(0, 0, 20, 10));
        let rect = layout[&node];
        assert_eq!(rect.width, 20);
        assert_eq!(rect.height, 2);
    }

    #[test]
    fn aspect_ratio_derives_height_from_width() {
        let mut tree = NodeTree::new();
        let root = tree.root_id();
        let attrs = StyleProps {
            width: Some(reactx_core::Dim::Cells(10.0)),
            aspect_ratio: Some(2.0),
            ..StyleProps::default()
        };
        let node = tree.insert_node("box", None);
        tree.get_mut(node).unwrap().attrs = attrs;
        tree.append_child(root, node);

        let layout = compute(&tree, Rect::new(0, 0, 20, 20));
        assert_eq!(layout[&node].width, 10);
        assert_eq!(layout[&node].height, 5);
    }

    #[test]
    fn effective_border_defaults_for_input() {
        let node = styled("input", StyleProps::default());
        assert_eq!(effective_border(&node), BorderKind::Single);
    }

    #[test]
    fn rects_are_absolute_to_the_area() {
        let mut tree = NodeTree::new();
        let root = tree.root_id();
        let node = tree.insert_node("span", None);
        tree.append_child(root, node);

        let layout = compute(&tree, Rect::new(5, 3, 20, 10));
        assert_eq!(layout[&root], Rect::new(5, 3, 20, 10));
        assert_eq!(layout[&node].x, 5);
        assert_eq!(layout[&node].y, 3);
    }
}
