//! Bar chart painter.

use ratatui::buffer::Buffer;
use ratatui::layout::{Direction, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line as TextLine;
use ratatui::widgets::{Bar, BarChart, BarGroup, Widget};
use reactx_core::{wire, TuiNode};

use super::{color, custom, flag, number, text_style};
use crate::layout;

pub(super) fn paint_barchart(node: &TuiNode, buf: &mut Buffer, area: Rect, fg: Option<Color>) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let pairs: Vec<(String, u64)> = custom(node, "barchart_data")
        .map(wire::split_rows)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|row| {
            let label = row.first()?.clone();
            let value = row.get(1).and_then(|value| value.parse().ok()).unwrap_or(0);
            Some((label, value))
        })
        .collect();
    if pairs.is_empty() {
        return;
    }

    let fill = color(node, "barchart_color", fg).unwrap_or(Color::Reset);
    let bar_style = Style::default().fg(fill);
    let bars: Vec<Bar> = pairs
        .iter()
        .map(|(label, value)| {
            Bar::default()
                .value(*value)
                .label(TextLine::from(label.clone()))
                .style(bar_style)
        })
        .collect();

    let bar_width = number::<u16>(custom(node, "barchart_bar_width")).unwrap_or(4);
    let bar_gap = number::<u16>(custom(node, "barchart_bar_gap")).unwrap_or(1);
    let mut chart = BarChart::default()
        .bar_width(bar_width.max(1))
        .bar_gap(bar_gap)
        .bar_style(bar_style)
        .label_style(text_style(fg))
        .data(BarGroup::default().bars(&bars));
    if let Some(max) = number::<u64>(custom(node, "barchart_max")) {
        chart = chart.max(max);
    }
    if flag(node, "barchart_horizontal") {
        chart = chart.direction(Direction::Horizontal);
        // Horizontal bars reserve columns for their labels; bail out rather
        // than underflow when the area is too narrow.
        let label_width = pairs
            .iter()
            .map(|(label, _)| layout::str_width(label))
            .max()
            .unwrap_or(0) as u16;
        if label_width + 1 >= area.width {
            return;
        }
    }
    chart.render(area, buf);
}
