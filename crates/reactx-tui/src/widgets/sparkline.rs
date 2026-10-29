//! Sparkline painter.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::{Sparkline, Widget};
use reactx_core::{wire, TuiNode};

use super::{color, custom, number, text_style};

pub(super) fn paint_sparkline(node: &TuiNode, buf: &mut Buffer, area: Rect, fg: Option<Color>) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let data: Vec<u64> = custom(node, "sparkline_data")
        .map(wire::split_nums)
        .unwrap_or_default();
    if data.is_empty() {
        return;
    }
    let mut sparkline = Sparkline::default()
        .data(data.as_slice())
        .style(text_style(color(node, "sparkline_color", fg)));
    if let Some(max) = number::<u64>(custom(node, "sparkline_max")) {
        sparkline = sparkline.max(max);
    }
    sparkline.render(area, buf);
}
