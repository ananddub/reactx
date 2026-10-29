//! Line chart painter.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::text::Line as TextLine;
use ratatui::widgets::{Axis, Chart, Dataset, GraphType, Widget};
use reactx_core::{wire, TuiColor, TuiNode};

use super::{custom, text_style};

pub(super) fn paint_chart(node: &TuiNode, buf: &mut Buffer, area: Rect, fg: Option<Color>) {
    if area.width < 2 || area.height < 2 {
        return;
    }
    let records = custom(node, "chart_datasets")
        .map(wire::split_rows)
        .unwrap_or_default();
    if records.is_empty() {
        return;
    }

    let points: Vec<Vec<(f64, f64)>> = records
        .iter()
        .map(|record| {
            record
                .get(3)
                .map(|encoded| wire::split_points(encoded))
                .unwrap_or_default()
        })
        .collect();

    let datasets: Vec<Dataset> = records
        .iter()
        .zip(points.iter())
        .filter(|(_, points)| !points.is_empty())
        .map(|(record, points)| {
            let name = record.first().cloned().unwrap_or_default();
            let color = record
                .get(1)
                .filter(|value| !value.is_empty())
                .and_then(|value| TuiColor::parse(value))
                .map(|value| value.to_ratatui())
                .or(fg);
            let marker = match record.get(2).map(String::as_str) {
                Some("braille") => Marker::Braille,
                Some("block") => Marker::Block,
                _ => Marker::Dot,
            };
            Dataset::default()
                .name(name)
                .data(points.as_slice())
                .marker(marker)
                .graph_type(GraphType::Line)
                .style(text_style(color))
        })
        .collect();
    if datasets.is_empty() {
        return;
    }

    let x_bounds = custom(node, "chart_xbounds")
        .and_then(wire::split_pair)
        .unwrap_or((0.0, 1.0));
    let y_bounds = custom(node, "chart_ybounds")
        .and_then(wire::split_pair)
        .unwrap_or((0.0, 1.0));
    let axis_style = text_style(fg);

    let mut x_axis = Axis::default().bounds([x_bounds.0, x_bounds.1]);
    if let Some(labels) = custom(node, "chart_xlabels") {
        x_axis = x_axis.labels(
            wire::split_fields(labels)
                .into_iter()
                .map(TextLine::from)
                .collect::<Vec<_>>(),
        );
    }
    let mut y_axis = Axis::default()
        .bounds([y_bounds.0, y_bounds.1])
        .style(axis_style);
    if let Some(labels) = custom(node, "chart_ylabels") {
        y_axis = y_axis.labels(
            wire::split_fields(labels)
                .into_iter()
                .map(TextLine::from)
                .collect::<Vec<_>>(),
        );
    }
    Chart::new(datasets)
        .x_axis(x_axis)
        .y_axis(y_axis)
        .render(area, buf);
}
