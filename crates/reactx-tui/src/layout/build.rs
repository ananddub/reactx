//! Taffy tree construction and leaf measurement.

use reactx_core::{NodeId, NodeTree};
use rustc_hash::FxHashMap;
use taffy::prelude::*;
use taffy::TaffyTree;
use unicode_width::UnicodeWidthStr;

use super::leaf::{leaf_for, Leaf};
use super::style::taffy_style;
use super::wrap::wrap_mode;

pub(super) fn build(
    taffy: &mut TaffyTree<Leaf>,
    mapping: &mut FxHashMap<taffy::NodeId, NodeId>,
    tree: &NodeTree,
    id: NodeId,
    style: Style,
) -> taffy::NodeId {
    let node = tree.get(id);

    if let Some(leaf) = node.and_then(|node| leaf_for(tree, node)) {
        let taffy_id = taffy
            .new_leaf_with_context(style, leaf)
            .expect("taffy leaf creation cannot fail");
        mapping.insert(taffy_id, id);
        return taffy_id;
    }

    let mut children = Vec::new();
    if let Some(node) = node {
        let mut ordered = node.children.clone();
        ordered.sort_by_key(|id| {
            tree.get(*id)
                .and_then(|child| child.attrs.order)
                .unwrap_or(0)
        });
        for child in ordered {
            if let Some(child_node) = tree.get(child) {
                if child_node.is_placeholder() || child_node.is_hidden() {
                    continue;
                }
                let child_style =
                    taffy_style(child_node, leaf_for(tree, child_node).is_some(), false);
                children.push(build(taffy, mapping, tree, child, child_style));
            }
        }
    }

    let taffy_id = taffy
        .new_with_children(style, &children)
        .expect("taffy node creation cannot fail");
    mapping.insert(taffy_id, id);
    taffy_id
}

pub(super) fn measure_leaf(
    leaf: &Leaf,
    known: Size<Option<f32>>,
    available: Size<AvailableSpace>,
) -> Size<f32> {
    let width_limit = if leaf.wrap {
        known.width.or(match available.width {
            AvailableSpace::Definite(width) if width.is_finite() && width > 0.0 => Some(width),
            _ => None,
        })
    } else {
        None
    };
    let lines = wrap_mode(
        &leaf.text,
        width_limit.map(|width| width.max(1.0) as usize),
        leaf.mode,
    );
    let width = lines
        .iter()
        .map(|line| UnicodeWidthStr::width(line.as_str()))
        .max()
        .unwrap_or(0) as f32;
    let height = lines.len() as f32;
    Size {
        width: known.width.unwrap_or(width),
        height: known.height.unwrap_or(height),
    }
}
