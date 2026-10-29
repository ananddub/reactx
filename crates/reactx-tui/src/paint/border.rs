//! Frame and title borders.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use reactx_core::{BorderKind, TitleAlign};

use super::cells::{put, put_line, truncate};
use super::layout;

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_border(
    buf: &mut Buffer,
    rect: Rect,
    dx: i32,
    dy: i32,
    clip: Rect,
    kind: BorderKind,
    title: Option<&str>,
    title_align: TitleAlign,
    color: Option<Color>,
    bg: Option<Color>,
) {
    if rect.width < 2 || rect.height < 2 {
        return;
    }
    let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) = match kind {
        BorderKind::Single => ('┌', '┐', '└', '┘', '─', '│'),
        BorderKind::Rounded => ('╭', '╮', '╰', '╯', '─', '│'),
        BorderKind::Double => ('╔', '╗', '╚', '╝', '═', '║'),
        BorderKind::Thick => ('┏', '┓', '┗', '┛', '━', '┃'),
        BorderKind::None => return,
    };

    let style = Style::default()
        .fg(color.unwrap_or(Color::Reset))
        .bg(bg.unwrap_or(Color::Reset));
    let left = rect.x as i32 + dx;
    let top = rect.y as i32 + dy;
    let right = left + rect.width as i32 - 1;
    let bottom = top + rect.height as i32 - 1;

    put(buf, clip, left, top, top_left, style);
    put(buf, clip, right, top, top_right, style);
    put(buf, clip, left, bottom, bottom_left, style);
    put(buf, clip, right, bottom, bottom_right, style);

    for x in left + 1..right {
        put(buf, clip, x, top, horizontal, style);
        put(buf, clip, x, bottom, horizontal, style);
    }
    for y in top + 1..bottom {
        put(buf, clip, left, y, vertical, style);
        put(buf, clip, right, y, vertical, style);
    }

    if let Some(title) = title.filter(|title| !title.is_empty()) {
        let max = rect.width.saturating_sub(2) as usize;
        if max == 0 {
            return;
        }
        let title = truncate(title, max);
        let width = layout::str_width(&title) as u16;
        let x = match title_align {
            TitleAlign::Left => rect.x + 1,
            TitleAlign::Center => {
                rect.x + 1 + (rect.width.saturating_sub(2).saturating_sub(width)) / 2
            }
            TitleAlign::Right => (right as u16).saturating_sub(width),
        };
        put_line(buf, x as i32 + dx, top, &title, style, clip, right);
    }
}
