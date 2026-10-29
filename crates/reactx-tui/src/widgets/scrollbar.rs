//! Scrollbar painter.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget};
use reactx_core::TuiNode;

use super::{color, custom, number, text_style};

pub(super) fn paint_scrollbar(node: &TuiNode, buf: &mut Buffer, area: Rect, fg: Option<Color>) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let orientation = match custom(node, "scrollbar_orientation") {
        Some("vertical-left") => ScrollbarOrientation::VerticalLeft,
        Some("horizontal-bottom") => ScrollbarOrientation::HorizontalBottom,
        Some("horizontal-top") => ScrollbarOrientation::HorizontalTop,
        _ => ScrollbarOrientation::VerticalRight,
    };
    let track = color(node, "scrollbar_track", fg);
    let thumb = color(node, "scrollbar_thumb", fg);
    let position = number::<usize>(custom(node, "scrollbar_position")).unwrap_or(0);
    let content = number::<usize>(custom(node, "scrollbar_content_length")).unwrap_or(0);

    let bar = Scrollbar::new(orientation)
        .style(text_style(track))
        .thumb_style(text_style(thumb));
    let mut state = ScrollbarState::new(content).position(position.min(content));
    StatefulWidget::render(bar, area, buf, &mut state);
}
