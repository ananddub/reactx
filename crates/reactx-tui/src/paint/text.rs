//! Multiline text fitting and alignment.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use reactx_core::TextAlign;

use super::cells::put_line;
use super::layout;

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_text(
    buf: &mut Buffer,
    area: Rect,
    dx: i32,
    dy: i32,
    clip: Rect,
    text: &str,
    align: TextAlign,
    style: Style,
    vertical_center: bool,
) {
    if area.width == 0 || area.height == 0 || text.is_empty() {
        return;
    }
    let lines = layout::wrap(text, Some(area.width as usize));
    let start_y = if vertical_center {
        area.y + area.height.saturating_sub(lines.len() as u16) / 2
    } else {
        area.y
    };

    for (index, line) in lines.iter().enumerate().take(area.height as usize) {
        let y = start_y.saturating_add(index as u16);
        if y >= area.y.saturating_add(area.height) {
            break;
        }
        let line_width = layout::str_width(line) as u16;
        let x = match align {
            TextAlign::Left => area.x,
            TextAlign::Center => area
                .x
                .saturating_add(area.width.saturating_sub(line_width) / 2),
            TextAlign::Right => area.x.saturating_add(area.width.saturating_sub(line_width)),
        };
        let origin_x = x as i32 + dx;
        let origin_y = y as i32 + dy;
        put_line(
            buf,
            origin_x,
            origin_y,
            line,
            style,
            clip,
            origin_x + area.width as i32 - (x - area.x) as i32,
        );
    }
}
