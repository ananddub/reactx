//! Class/style layer recomputation for the node tree.

use reactx_core::style::parse_bool;
use reactx_core::ClassState;

use super::TuiRenderer;

impl TuiRenderer {
    /// Rebuild every node's effective styles before layout, so hover, focus,
    /// disabled, checked, sibling-position and responsive variants see the
    /// current frame.
    pub(super) fn rebuild_styles(&mut self) {
        let width = self.area.width;
        let hover = self.hover;
        let focused = self.focus.focused_id();
        let ids = self.tree.nodes_in_order();

        for id in ids {
            let (disabled, checked, index, siblings) = self
                .tree
                .get(id)
                .map(|node| {
                    let (index, siblings) = node
                        .parent
                        .and_then(|parent| self.tree.get(parent))
                        .map(|parent| {
                            let index = parent
                                .children
                                .iter()
                                .position(|child| *child == id)
                                .unwrap_or(0);
                            (index, parent.children.len().max(1))
                        })
                        .unwrap_or((0, 1));
                    let style = node.style.as_deref();
                    let disabled =
                        style_flag(style, "disabled").unwrap_or(node.base_attrs.disabled);
                    let checked = style_flag(style, "checked")
                        .unwrap_or(node.base_attrs.checked == Some(true));
                    (disabled, checked, index, siblings)
                })
                .unwrap_or((false, false, 0, 1));
            let state = ClassState {
                hovered: hover.is_some_and(|node| self.tree.is_within(node, id)),
                focused: focused == Some(id),
                focus_within: focused.is_some_and(|node| self.tree.is_within(node, id)),
                group_hovered: hover.is_some_and(|node| self.tree.is_within(id, node)),
                disabled,
                checked,
                index,
                siblings,
                width,
            };
            if let Some(node) = self.tree.get_mut(id) {
                node.rebuild_styles(&state);
            }
        }
    }
}

/// Read a boolean flag from an inline `style` declaration list, e.g. the
/// `disabled:true` that widgets pass for their own state.
fn style_flag(style: Option<&str>, key: &str) -> Option<bool> {
    for declaration in style?.split(';') {
        let Some((name, value)) = declaration.split_once(':') else {
            continue;
        };
        if name.trim().eq_ignore_ascii_case(key) {
            return Some(parse_bool(value));
        }
    }
    None
}
