//! Depth-first node painting and widget dispatch.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use reactx_core::{NodeId, TextAlign, TuiColor};

use super::border::draw_border;
use super::cells::{fill_bg, modifiers};
use super::input::paint_input;
use super::scratch::paint_widget;
use super::text::draw_text;
use super::textarea::paint_textarea;
use super::{layout, Context, Inherited};

pub(super) fn paint(
    ctx: &Context,
    buf: &mut Buffer,
    id: NodeId,
    inherited_clip: Rect,
    inherited: Inherited,
) {
    let Some(node) = ctx.tree.get(id) else {
        return;
    };
    if node.is_placeholder() || node.is_hidden() {
        return;
    }
    let Some(rect) = ctx.layout.get(&id).copied() else {
        return;
    };
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let clip = ctx.clips.get(&id).copied().unwrap_or(inherited_clip);
    let (dx, dy) = ctx.offsets.get(&id).copied().unwrap_or((0, 0));
    let Some(visible) = layout::visible_rect(rect, dx, dy, clip) else {
        return;
    };

    let fg = node.attrs.color.map(TuiColor::to_ratatui).or(inherited.fg);
    let bg = node.attrs.bg.map(TuiColor::to_ratatui);
    let mut modifier = inherited.modifier | modifiers(node);
    let focused = ctx.focus.is_focused(id);
    let align = node.attrs.text_align.unwrap_or(inherited.align);
    let border = layout::effective_border(node);
    let bordered = border.is_visible();

    if let Some(bg) = bg {
        fill_bg(buf, visible, bg);
    }

    let border_color = node
        .attrs
        .border_color
        .map(TuiColor::to_ratatui)
        .or(if focused { Some(ctx.focus_color) } else { None })
        .or(fg);
    if bordered {
        draw_border(
            buf,
            rect,
            dx,
            dy,
            clip,
            border,
            node.attrs.title.as_deref(),
            node.attrs.title_align.unwrap_or_default(),
            border_color,
            bg,
        );
    }

    let interior = layout::inner_rect(rect, node);
    let content_style = Style::default()
        .fg(fg.unwrap_or(Color::Reset))
        .bg(bg.unwrap_or(Color::Reset))
        .add_modifier(modifier);

    match layout::widget_kind(node) {
        "#text" => {
            let text = node.text.clone().unwrap_or_default();
            draw_text(buf, rect, dx, dy, clip, &text, align, content_style, false);
        }
        "text" => {
            let text = ctx.tree.collect_text(id);
            draw_text(
                buf,
                interior,
                dx,
                dy,
                clip,
                &text,
                align,
                content_style,
                false,
            );
        }
        "button" => {
            let text = ctx.tree.collect_text(id);
            if focused && !bordered {
                modifier |= Modifier::REVERSED;
            }
            let style = content_style.add_modifier(Modifier::BOLD);
            draw_text(buf, interior, dx, dy, clip, &text, align, style, true);
        }
        "input" => paint_input(
            ctx,
            buf,
            interior,
            dx,
            dy,
            clip,
            node,
            content_style,
            focused,
            id,
        ),
        "textarea" => paint_textarea(
            ctx,
            buf,
            interior,
            dx,
            dy,
            clip,
            node,
            content_style,
            focused,
            id,
        ),
        "checkbox" => {
            let label = layout::checkbox_text(node, &ctx.tree.collect_text(id));
            let text = if focused {
                format!("› {label}")
            } else {
                format!("  {label}")
            };
            let style = if focused {
                content_style.fg(ctx.focus_color)
            } else {
                content_style
            };
            draw_text(
                buf,
                rect,
                dx,
                dy,
                clip,
                &text,
                TextAlign::Left,
                style,
                false,
            );
        }
        "spacer" => {}
        kind => {
            if paint_widget(kind, buf, node, interior, dx, dy, clip, fg, bg) {
                return;
            }
            let child_inherited = Inherited {
                fg,
                modifier,
                align,
            };
            for child in ctx.tree.sorted_children(node) {
                paint(ctx, buf, child, clip, child_inherited);
            }
        }
    }
}
