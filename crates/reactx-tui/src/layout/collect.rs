//! Walking the computed Taffy tree into absolute rectangles and clips.

use ratatui::layout::Rect;
use reactx_core::{NodeId, NodeTree, Overflow, TuiNode};
use rustc_hash::FxHashMap;
use taffy::TaffyTree;

use super::geometry::{inner_rect, intersect};
use super::leaf::Leaf;
use super::LayoutResult;

#[allow(clippy::too_many_arguments)]
pub(super) fn collect_rects(
    taffy: &TaffyTree<Leaf>,
    mapping: &FxHashMap<taffy::NodeId, NodeId>,
    tree: &NodeTree,
    taffy_id: taffy::NodeId,
    origin_x: f32,
    origin_y: f32,
    offset_x: f32,
    offset_y: f32,
    clip: Rect,
    out: &mut LayoutResult,
) {
    let Ok(layout) = taffy.layout(taffy_id) else {
        return;
    };
    let abs_x = origin_x + layout.location.x;
    let abs_y = origin_y + layout.location.y;

    let mut child_offset_x = offset_x;
    let mut child_offset_y = offset_y;
    let mut child_clip = clip;

    if let Some(node_id) = mapping.get(&taffy_id) {
        if let Some(node) = tree.get(*node_id) {
            let rect = Rect {
                x: abs_x.max(0.0).round() as u16,
                y: abs_y.max(0.0).round() as u16,
                width: layout.size.width.max(0.0).round() as u16,
                height: layout.size.height.max(0.0).round() as u16,
            };
            out.rects.insert(*node_id, rect);
            out.clips.insert(*node_id, clip);
            out.offsets
                .insert(*node_id, (offset_x.round() as i32, offset_y.round() as i32));

            if node_clips(node) {
                child_clip = intersect(clip, inner_rect(rect, node));
            }
            child_offset_x -= node.attrs.scroll_x.round();
            child_offset_y -= node.attrs.scroll_y.round();
        }
    }

    if let Ok(children) = taffy.children(taffy_id) {
        for child in children {
            collect_rects(
                taffy,
                mapping,
                tree,
                child,
                abs_x,
                abs_y,
                child_offset_x,
                child_offset_y,
                child_clip,
                out,
            );
        }
    }
}

fn node_clips(node: &TuiNode) -> bool {
    matches!(node.attrs.overflow, Some(Overflow::Hidden))
        || node.attrs.scroll_x > 0.0
        || node.attrs.scroll_y > 0.0
}
