//! Default widget heights and the effective border of a node.

use reactx_core::{BorderKind, TuiNode};

use super::leaf::is_text_input;

/// The height a widget needs when the element does not declare one.
pub(super) fn default_widget_height(node: &TuiNode, kind: &str) -> Option<f32> {
    let height = match kind {
        "gauge" => 3.0,
        "line-gauge" => 1.0,
        "sparkline" => 3.0,
        "barchart" => 6.0,
        "chart" => 8.0,
        "canvas" => 8.0,
        "table" => {
            let rows = node
                .attrs
                .custom
                .get("table_rows")
                .map(|encoded| reactx_core::wire::split_rows(encoded).len())
                .unwrap_or(0) as f32;
            let header = if node.attrs.custom.contains_key("table_headers") {
                1.0
            } else {
                0.0
            };
            let border = if effective_border(node).is_visible() {
                2.0
            } else {
                0.0
            };
            let padding = node.attrs.padding.top + node.attrs.padding.bottom;
            (rows + header + border + padding).max(3.0)
        }
        "textarea" => {
            let lines = node
                .attrs
                .value
                .as_deref()
                .unwrap_or_default()
                .split('\n')
                .count() as f32;
            let border = if effective_border(node).is_visible() {
                2.0
            } else {
                0.0
            };
            let padding = node.attrs.padding.top + node.attrs.padding.bottom;
            (lines + border + padding).max(3.0)
        }
        _ => return None,
    };
    Some(height)
}

/// The border the painter will actually draw. Text inputs get a single border
/// by default.
pub(crate) fn effective_border(node: &TuiNode) -> BorderKind {
    match node.attrs.border {
        Some(kind) => kind,
        None if is_text_input(node) => BorderKind::Single,
        None => BorderKind::None,
    }
}
