//! Single-line input painting with horizontal scroll, selection and caret.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use reactx_core::TuiNode;
use unicode_width::UnicodeWidthChar;

use super::cells::{cell_visible, char_prefix_width, put_line, skip_columns};
use super::Context;

#[allow(clippy::too_many_arguments)]
pub(super) fn paint_input(
    ctx: &Context,
    buf: &mut Buffer,
    area: Rect,
    dx: i32,
    dy: i32,
    clip: Rect,
    node: &TuiNode,
    style: Style,
    focused: bool,
    id: reactx_core::NodeId,
) {
    let value = node.attrs.value.clone().unwrap_or_default();
    let empty = value.is_empty();
    let text = if empty {
        node.attrs.placeholder.clone().unwrap_or_default()
    } else {
        value.clone()
    };
    if area.width == 0 || area.height == 0 {
        return;
    }

    let cursor = ctx.cursors.get(&id).copied().unwrap_or_else(|| {
        node.attrs
            .value
            .as_deref()
            .unwrap_or_default()
            .chars()
            .count()
    });
    let cursor_chars = cursor.min(value.chars().count());
    let cursor_col = if empty {
        0
    } else {
        char_prefix_width(&value, cursor_chars)
    };
    let offset = if focused && !empty {
        cursor_col.saturating_sub(area.width.saturating_sub(1))
    } else {
        0
    };
    let visible_text = skip_columns(&text, offset);
    let style = if empty {
        style.add_modifier(Modifier::DIM)
    } else {
        style
    };
    let origin_x = area.x as i32 + dx;
    let origin_y = area.y as i32 + dy;
    put_line(
        buf,
        origin_x,
        origin_y,
        &visible_text,
        style,
        clip,
        origin_x + area.width as i32,
    );

    if !empty {
        if let Some((start, end)) = ctx.selections.get(&id).copied() {
            let mut column = 0u16;
            for (index, ch) in value.chars().enumerate() {
                let width = UnicodeWidthChar::width(ch).unwrap_or(0) as u16;
                if index >= start && index < end && column >= offset {
                    let x = origin_x + column.saturating_sub(offset) as i32;
                    if cell_visible(x, origin_y, clip) {
                        if let Some(cell) = buf.cell_mut((x as u16, origin_y as u16)) {
                            cell.modifier |= Modifier::REVERSED;
                        }
                    }
                }
                column = column.saturating_add(width);
                if column.saturating_sub(offset) >= area.width {
                    break;
                }
            }
        }
    }

    if !focused {
        return;
    }
    let x = origin_x + cursor_col.saturating_sub(offset) as i32;
    let y = origin_y;
    if !cell_visible(x, y, clip) {
        return;
    }
    if let Some(cell) = buf.cell_mut((x as u16, y as u16)) {
        cell.modifier |= Modifier::REVERSED;
    }
}
