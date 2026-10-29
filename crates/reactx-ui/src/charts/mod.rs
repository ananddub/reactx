//! Data widgets: gauges, tables, charts, canvas, scrollbars and a calendar.
//!
//! Each component encodes its props into the element's `style` attribute;
//! the terminal painter decodes them and draws the matching Ratatui widget
//! (see `reactx-tui::widgets`). Calendar is a regular Dioxus tree of
//! focusable cells, so keyboard and mouse selection work out of the box.

use reactx_core::TuiColor;

use crate::theme::Theme;
use crate::widgets::style_push;

mod barchart;
mod calendar;
mod canvas;
mod chart;
mod gauge;
mod scrollbar;
mod sparkline;
mod table;

pub use barchart::BarChart;
pub use calendar::Calendar;
pub use canvas::{Canvas, CanvasShape};
pub use chart::{Chart, ChartDataset};
pub use gauge::Gauge;
pub use scrollbar::Scrollbar;
pub use sparkline::Sparkline;
pub use table::Table;

fn css(color: TuiColor) -> String {
    color.to_css()
}

fn border_style(style: &mut String, border: &str, theme: &Theme) {
    let border = if border.is_empty() { "none" } else { border };
    style_push(style, "border", border);
    if border != "none" {
        style_push(style, "border-color", &css(theme.border));
    }
}

fn title_style(style: &mut String, title: &str) {
    if !title.is_empty() {
        style_push(style, "title", title);
    }
}
