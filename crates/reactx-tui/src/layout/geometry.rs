//! Rectangle intersection, clipping and interior computation.

use ratatui::layout::Rect;
use reactx_core::TuiNode;

use super::metrics::effective_border;

/// Intersection of two rectangles (empty when they do not overlap).
pub(crate) fn intersect(a: Rect, b: Rect) -> Rect {
    let x1 = a.x.max(b.x);
    let y1 = a.y.max(b.y);
    let x2 = a.x.saturating_add(a.width).min(b.x.saturating_add(b.width));
    let y2 =
        a.y.saturating_add(a.height)
            .min(b.y.saturating_add(b.height));
    Rect {
        x: x1,
        y: y1,
        width: x2.saturating_sub(x1),
        height: y2.saturating_sub(y1),
    }
}

/// A rectangle shifted by a signed offset, clipped to `clip`.
/// Returns `None` when nothing remains visible.
pub(crate) fn visible_rect(rect: Rect, dx: i32, dy: i32, clip: Rect) -> Option<Rect> {
    let x1 = rect.x as i32 + dx;
    let y1 = rect.y as i32 + dy;
    let x2 = x1 + rect.width as i32;
    let y2 = y1 + rect.height as i32;
    let cx1 = clip.x as i32;
    let cy1 = clip.y as i32;
    let cx2 = cx1 + clip.width as i32;
    let cy2 = cy1 + clip.height as i32;

    let vx1 = x1.max(cx1);
    let vy1 = y1.max(cy1);
    let vx2 = x2.min(cx2);
    let vy2 = y2.min(cy2);
    if vx2 <= vx1 || vy2 <= vy1 {
        return None;
    }
    Some(Rect {
        x: vx1 as u16,
        y: vy1 as u16,
        width: (vx2 - vx1) as u16,
        height: (vy2 - vy1) as u16,
    })
}

/// The rectangle inside a node's border and padding.
pub(crate) fn inner_rect(rect: Rect, node: &TuiNode) -> Rect {
    let border = if effective_border(node).is_visible() {
        1
    } else {
        0
    };
    let padding = &node.attrs.padding;
    let left = border + padding.left.max(0.0).round() as u16;
    let right = border + padding.right.max(0.0).round() as u16;
    let top = border + padding.top.max(0.0).round() as u16;
    let bottom = border + padding.bottom.max(0.0).round() as u16;
    Rect {
        x: rect.x.saturating_add(left),
        y: rect.y.saturating_add(top),
        width: rect.width.saturating_sub(left).saturating_sub(right),
        height: rect.height.saturating_sub(top).saturating_sub(bottom),
    }
}
