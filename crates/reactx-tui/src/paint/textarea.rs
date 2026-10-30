//! Multiline textarea painting with vertical and horizontal caret scroll.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use reactx_core::TuiNode;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::cells::{cell_visible, put_line, skip_columns};
use super::Context;

/// Multiline painting: hard newlines scroll vertically to keep the caret
/// visible; the caret line also scrolls horizontally.
#[allow(clippy::too_many_arguments)]
pub(super) fn paint_textarea(
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
    if area.width == 0 || area.height == 0 {
        return;
    }
    let value = node.attrs.value.clone().unwrap_or_default();
    let empty = value.is_empty();
    let text = if empty {
        node.attrs.placeholder.clone().unwrap_or_default()
    } else {
        value.clone()
    };

    let cursor = ctx.cursors.get(&id).copied().unwrap_or_else(|| {
        node.attrs
            .value
            .as_deref()
            .unwrap_or_default()
            .graphemes(true)
            .count()
    });
    let cursor_index = cursor.min(value.graphemes(true).count());
    let (caret_row, caret_col) = caret_line_col(&value, cursor_index);
    let rows = area.height as usize;
    let columns = area.width as usize;
    let vscroll = if focused {
        caret_row.saturating_sub(rows - 1)
    } else {
        0
    };
    let hscroll = if focused {
        caret_col.saturating_sub(columns - 1)
    } else {
        0
    };

    let style = if empty {
        style.add_modifier(Modifier::DIM)
    } else {
        style
    };
    let origin_x = area.x as i32 + dx;
    let origin_y = area.y as i32 + dy;

    for (index, line) in text.split('\n').enumerate() {
        if index < vscroll {
            continue;
        }
        let row = index - vscroll;
        if row >= rows {
            break;
        }
        let visible = skip_columns(line, hscroll.min(u16::MAX as usize) as u16);
        put_line(
            buf,
            origin_x,
            origin_y + row as i32,
            &visible,
            style,
            clip,
            origin_x + area.width as i32,
        );
    }

    if !empty {
        if let Some((start, end)) = ctx.selections.get(&id).copied() {
            let mut row = 0usize;
            let mut column = 0usize;
            for (index, grapheme) in value.graphemes(true).enumerate() {
                if grapheme == "\n" {
                    row += 1;
                    column = 0;
                    continue;
                }
                if index >= start
                    && index < end
                    && row >= vscroll
                    && row - vscroll < rows
                    && column >= hscroll
                    && column - hscroll < columns
                {
                    let x = origin_x + (column - hscroll) as i32;
                    let y = origin_y + (row - vscroll) as i32;
                    if cell_visible(x, y, clip) {
                        if let Some(cell) = buf.cell_mut((x as u16, y as u16)) {
                            cell.modifier |= Modifier::REVERSED;
                        }
                    }
                }
                column += UnicodeWidthStr::width(grapheme);
                if index > end {
                    break;
                }
            }
        }
    }

    if !focused {
        return;
    }
    if caret_row < vscroll || caret_row - vscroll >= rows {
        return;
    }
    let x = origin_x + caret_col.saturating_sub(hscroll) as i32;
    let y = origin_y + (caret_row - vscroll) as i32;
    if !cell_visible(x, y, clip) {
        return;
    }
    if let Some(cell) = buf.cell_mut((x as u16, y as u16)) {
        cell.modifier |= Modifier::REVERSED;
    }
}

/// Convert a grapheme offset into a (row, display column) pair.
pub(super) fn caret_line_col(value: &str, cursor: usize) -> (usize, usize) {
    let mut row = 0usize;
    let mut column = 0usize;
    for (index, grapheme) in value.graphemes(true).enumerate() {
        if index >= cursor {
            break;
        }
        if grapheme == "\n" {
            row += 1;
            column = 0;
        } else {
            column += UnicodeWidthStr::width(grapheme);
        }
    }
    (row, column)
}

#[cfg(test)]
mod tests {
    use super::caret_line_col;

    #[test]
    fn caret_counts_graphemes() {
        assert_eq!(caret_line_col("a👨‍👩‍👧\nb", 2), (0, 3));
        assert_eq!(caret_line_col("a👨‍👩‍👧\nb", 3), (1, 0));
    }
}
