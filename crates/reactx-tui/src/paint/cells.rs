//! Low-level buffer cells: backgrounds, lines, clipping and column math.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use reactx_core::TuiNode;
use unicode_width::UnicodeWidthChar;

/// Opaque background: clearing the cells keeps overlays from showing the
/// content painted underneath them.
pub(super) fn fill_bg(buf: &mut Buffer, area: Rect, bg: Color) {
    for y in area.y..area.y.saturating_add(area.height) {
        for x in area.x..area.x.saturating_add(area.width) {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.reset();
                cell.set_bg(bg);
            }
        }
    }
}

pub(super) fn modifiers(node: &TuiNode) -> Modifier {
    let attrs = &node.attrs;
    let mut modifier = Modifier::empty();
    if attrs.bold {
        modifier |= Modifier::BOLD;
    }
    if attrs.italic {
        modifier |= Modifier::ITALIC;
    }
    if attrs.underline {
        modifier |= Modifier::UNDERLINED;
    }
    if attrs.dim {
        modifier |= Modifier::DIM;
    }
    if attrs.strikethrough {
        modifier |= Modifier::CROSSED_OUT;
    }
    if attrs.reversed {
        modifier |= Modifier::REVERSED;
    }
    modifier
}

/// Write one line of text, honouring a clip region and a right bound.
pub(super) fn put_line(
    buf: &mut Buffer,
    x: i32,
    y: i32,
    line: &str,
    style: Style,
    clip: Rect,
    right_bound: i32,
) {
    if y < clip.y as i32 || y >= (clip.y as i32 + clip.height as i32) {
        return;
    }
    let clip_left = clip.x as i32;
    let clip_right = (clip.x as i32 + clip.width as i32).min(right_bound);
    if clip_right <= clip_left {
        return;
    }
    let mut column = x;
    for ch in line.chars() {
        let width = UnicodeWidthChar::width(ch).unwrap_or(0) as i32;
        if width == 0 {
            continue;
        }
        let next = column + width;
        if next > right_bound {
            break;
        }
        if column >= clip_left
            && next <= clip_right
            && next <= buf.area.width as i32
            && column >= 0
            && next > 0
        {
            if let Some(cell) = buf.cell_mut((column as u16, y as u16)) {
                cell.set_symbol(ch.encode_utf8(&mut [0u8; 4]));
                cell.set_style(style);
            }
            for continuation in column + 1..next {
                if continuation >= 0 {
                    if let Some(cell) = buf.cell_mut((continuation as u16, y as u16)) {
                        cell.reset();
                    }
                }
            }
        }
        column = next;
    }
}

pub(super) fn char_prefix_width(text: &str, chars: usize) -> u16 {
    text.chars()
        .take(chars)
        .map(|ch| UnicodeWidthChar::width(ch).unwrap_or(0) as u16)
        .sum()
}

/// Drop leading display columns, matching terminal cell widths.
pub(super) fn skip_columns(text: &str, columns: u16) -> String {
    let mut skipped = 0u16;
    let mut out = String::new();
    let mut dropping = true;
    for ch in text.chars() {
        if dropping {
            let width = UnicodeWidthChar::width(ch).unwrap_or(0) as u16;
            if skipped.saturating_add(width) > columns {
                dropping = false;
            } else {
                skipped += width;
                continue;
            }
        }
        out.push(ch);
    }
    out
}

pub(super) fn cell_visible(x: i32, y: i32, clip: Rect) -> bool {
    x >= clip.x as i32
        && y >= clip.y as i32
        && x < clip.x as i32 + clip.width as i32
        && y < clip.y as i32 + clip.height as i32
}

pub(super) fn truncate(text: &str, max_width: usize) -> String {
    let mut out = String::new();
    let mut width = 0;
    for ch in text.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width > max_width {
            break;
        }
        out.push(ch);
        width += ch_width;
    }
    out
}

pub(super) fn put(buf: &mut Buffer, clip: Rect, x: i32, y: i32, symbol: char, style: Style) {
    if !cell_visible(x, y, clip) {
        return;
    }
    if let Some(cell) = buf.cell_mut((x as u16, y as u16)) {
        cell.set_symbol(&symbol.to_string());
        cell.set_style(style);
    }
}
