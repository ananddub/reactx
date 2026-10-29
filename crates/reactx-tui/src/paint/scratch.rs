//! Widget rendering, including the scratch buffer used when scrolled or clipped.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use reactx_core::TuiNode;

use super::layout;

/// Render a widget into a scratch buffer first when scroll or clip crops it, so
/// the visible rows keep their true positions instead of restarting.
#[allow(clippy::too_many_arguments)]
pub(super) fn paint_widget(
    kind: &str,
    buf: &mut Buffer,
    node: &TuiNode,
    area: Rect,
    dx: i32,
    dy: i32,
    clip: Rect,
    fg: Option<Color>,
    bg: Option<Color>,
) -> bool {
    let Some(visible) = layout::visible_rect(area, dx, dy, clip) else {
        return true;
    };
    if dx == 0 && dy == 0 && visible == area {
        return crate::widgets::paint(kind, buf, node, area, fg, bg);
    }
    let mut scratch = Buffer::empty(area);
    for y in area.y..area.y.saturating_add(area.height) {
        for x in area.x..area.x.saturating_add(area.width) {
            if let (Some(dst), Some(src)) = (scratch.cell_mut((x, y)), buf.cell((x, y))) {
                *dst = src.clone();
            }
        }
    }
    if !crate::widgets::paint(kind, &mut scratch, node, area, fg, bg) {
        return false;
    }
    for y in visible.y..visible.y.saturating_add(visible.height) {
        for x in visible.x..visible.x.saturating_add(visible.width) {
            let sx = (x as i32 - dx) as u16;
            let sy = (y as i32 - dy) as u16;
            if let Some(cell) = scratch.cell((sx, sy)) {
                buf[(x, y)] = cell.clone();
            }
        }
    }
    true
}
