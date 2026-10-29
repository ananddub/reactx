//! Gauge and line-gauge painters.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Gauge, LineGauge, Widget};
use reactx_core::{TuiColor, TuiNode};

use super::{custom, number};

fn ratio(node: &TuiNode) -> f64 {
    let value = number::<f64>(custom(node, "gauge_ratio")).unwrap_or(0.0);
    let ratio = if value > 1.0 { value / 100.0 } else { value };
    if ratio.is_finite() {
        ratio.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

pub(super) fn paint_gauge(node: &TuiNode, buf: &mut Buffer, area: Rect, fg: Option<Color>) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let fill = node.attrs.color.map(TuiColor::to_ratatui).or(fg);
    let track = node.attrs.bg.map(TuiColor::to_ratatui);
    let style = Style::default()
        .fg(fill.unwrap_or(Color::Green))
        .bg(track.unwrap_or(Color::Reset));
    let mut gauge = Gauge::default().gauge_style(style).ratio(ratio(node));
    if let Some(label) = custom(node, "gauge_label").filter(|label| !label.is_empty()) {
        gauge = gauge.label(label);
    }
    gauge.render(area, buf);
}

pub(super) fn paint_line_gauge(node: &TuiNode, buf: &mut Buffer, area: Rect, fg: Option<Color>) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let fill = node.attrs.color.map(TuiColor::to_ratatui).or(fg);
    let track = node.attrs.bg.map(TuiColor::to_ratatui);
    let line = Rect {
        y: area.y + (area.height - 1) / 2,
        height: 1,
        ..area
    };
    let mut gauge = LineGauge::default()
        .filled_style(Style::default().fg(fill.unwrap_or(Color::Green)))
        .unfilled_style(Style::default().fg(track.unwrap_or(Color::DarkGray)))
        .ratio(ratio(node));
    if let Some(label) = custom(node, "gauge_label").filter(|label| !label.is_empty()) {
        gauge = gauge.label(label);
    }
    gauge.render(line, buf);
}
