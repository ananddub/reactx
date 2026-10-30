//! Frame and title borders.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use reactx_core::{BorderKind, TitleAlign, TuiColor, TuiNode};

use super::cells::{put, put_line, truncate};
use super::layout;

/// Per-side border colours and the dim modifier.
#[derive(Clone, Copy)]
pub(super) struct BorderColors {
    pub top: Color,
    pub right: Color,
    pub bottom: Color,
    pub left: Color,
    pub dim: bool,
}

impl BorderColors {
    fn style(self, side: Color, bg: Option<Color>) -> Style {
        let mut style = Style::default().fg(side).bg(bg.unwrap_or(Color::Reset));
        if self.dim {
            style = style.add_modifier(Modifier::DIM);
        }
        style
    }
}

/// Resolve per-side colours, falling back to `base` (border colour, focus
/// accent or foreground) for sides without an override.
pub(super) fn border_colors(node: &TuiNode, base: Option<Color>) -> BorderColors {
    let side = |over: Option<TuiColor>| {
        over.map(TuiColor::to_ratatui)
            .or(base)
            .unwrap_or(Color::Reset)
    };
    BorderColors {
        top: side(node.attrs.border_top_color),
        right: side(node.attrs.border_right_color),
        bottom: side(node.attrs.border_bottom_color),
        left: side(node.attrs.border_left_color),
        dim: node.attrs.border_dim,
    }
}

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
    colors: BorderColors,
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
        BorderKind::SingleDouble => ('╓', '╖', '╙', '╜', '─', '║'),
        BorderKind::DoubleSingle => ('╒', '╕', '╘', '╛', '═', '│'),
        BorderKind::Classic => ('+', '+', '+', '+', '-', '|'),
        BorderKind::None => return,
    };
    let top_style = colors.style(colors.top, bg);
    let bottom_style = colors.style(colors.bottom, bg);
    let left_style = colors.style(colors.left, bg);
    let right_style = colors.style(colors.right, bg);

    let left = rect.x as i32 + dx;
    let top = rect.y as i32 + dy;
    let right = left + rect.width as i32 - 1;
    let bottom = top + rect.height as i32 - 1;

    put(buf, clip, left, top, top_left, top_style);
    put(buf, clip, right, top, top_right, top_style);
    put(buf, clip, left, bottom, bottom_left, bottom_style);
    put(buf, clip, right, bottom, bottom_right, bottom_style);

    for x in left + 1..right {
        put(buf, clip, x, top, horizontal, top_style);
        put(buf, clip, x, bottom, horizontal, bottom_style);
    }
    for y in top + 1..bottom {
        put(buf, clip, left, y, vertical, left_style);
        put(buf, clip, right, y, vertical, right_style);
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
        put_line(buf, x as i32 + dx, top, &title, top_style, clip, right);
    }
}
