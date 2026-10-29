//! Ratatui painters for data widgets.
//!
//! Widget components encode their props into custom attributes (see
//! [`reactx_core::wire`]); this module decodes them and draws the matching
//! Ratatui widget into the area the layout pass reserved.

use std::str::FromStr;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use reactx_core::style::parse_bool;
use reactx_core::{TuiColor, TuiNode};

mod barchart;
mod canvas;
mod chart;
mod gauge;
mod scrollbar;
mod sparkline;
mod table;

/// Paint `kind` into `area`. Returns false when the kind is not a data widget.
pub fn paint(
    kind: &str,
    buf: &mut Buffer,
    node: &TuiNode,
    area: Rect,
    fg: Option<Color>,
    bg: Option<Color>,
) -> bool {
    match kind {
        "gauge" => gauge::paint_gauge(node, buf, area, fg),
        "line-gauge" => gauge::paint_line_gauge(node, buf, area, fg),
        "table" => table::paint_table(node, buf, area, fg),
        "barchart" => barchart::paint_barchart(node, buf, area, fg),
        "sparkline" => sparkline::paint_sparkline(node, buf, area, fg),
        "chart" => chart::paint_chart(node, buf, area, fg),
        "canvas" => canvas::paint_canvas(node, buf, area, fg, bg),
        "scrollbar" => scrollbar::paint_scrollbar(node, buf, area, fg),
        _ => return false,
    }
    true
}

pub(super) fn custom<'a>(node: &'a TuiNode, key: &str) -> Option<&'a str> {
    node.attrs.custom.get(key).map(String::as_str)
}

pub(super) fn number<T: FromStr>(text: Option<&str>) -> Option<T> {
    text?.trim().parse().ok()
}

pub(super) fn flag(node: &TuiNode, key: &str) -> bool {
    custom(node, key).is_some_and(parse_bool)
}

pub(super) fn color(node: &TuiNode, key: &str, fallback: Option<Color>) -> Option<Color> {
    custom(node, key)
        .and_then(TuiColor::parse)
        .map(TuiColor::to_ratatui)
        .or(fallback)
}

pub(super) fn text_style(fg: Option<Color>) -> Style {
    Style::default().fg(fg.unwrap_or(Color::Reset))
}
