//! Table painter and column sizing.

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Cell, Row, StatefulWidget, Table, TableState};
use reactx_core::{wire, TuiNode};

use super::{color, custom, number, text_style};
use crate::layout;

pub(super) fn paint_table(node: &TuiNode, buf: &mut Buffer, area: Rect, fg: Option<Color>) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let headers = custom(node, "table_headers")
        .map(wire::split_fields)
        .unwrap_or_default();
    let rows: Vec<Vec<String>> = custom(node, "table_rows")
        .map(wire::split_rows)
        .unwrap_or_default();
    if headers.is_empty() && rows.is_empty() {
        return;
    }

    let requested: Vec<u16> = custom(node, "table_widths")
        .map(wire::split_nums)
        .unwrap_or_default()
        .into_iter()
        .map(|width| width.min(u16::MAX as u64) as u16)
        .collect();
    let constraints = table_constraints(&headers, &rows, &requested, area.width);

    let style = text_style(fg);
    let body: Vec<Row> = rows
        .iter()
        .map(|row| Row::new(row.iter().cloned().map(Cell::from)))
        .collect();
    let mut table = Table::new(body, constraints).column_spacing(1).style(style);
    if !headers.is_empty() {
        let header =
            Row::new(headers.into_iter().map(Cell::from)).style(style.add_modifier(Modifier::BOLD));
        table = table.header(header);
    }

    let highlight = color(node, "table_highlight", None).unwrap_or(Color::Rgb(69, 71, 90));
    table = table.row_highlight_style(Style::default().bg(highlight).add_modifier(Modifier::BOLD));

    let mut state = TableState::default();
    if let Some(selected) = number::<usize>(custom(node, "table_selected")) {
        state.select(Some(selected));
    }
    StatefulWidget::render(table, area, buf, &mut state);
}

fn table_constraints(
    headers: &[String],
    rows: &[Vec<String>],
    requested: &[u16],
    width: u16,
) -> Vec<Constraint> {
    if !requested.is_empty() {
        return requested
            .iter()
            .map(|width| Constraint::Length(*width))
            .collect();
    }

    let columns = headers
        .len()
        .max(rows.iter().map(Vec::len).max().unwrap_or(0));
    let mut widths = vec![0usize; columns];
    for (index, header) in headers.iter().enumerate() {
        widths[index] = widths[index].max(layout::str_width(header));
    }
    for row in rows {
        for (index, cell) in row.iter().enumerate() {
            widths[index] = widths[index].max(layout::str_width(cell));
        }
    }

    let spacing = columns.saturating_sub(1);
    let available = (width as usize).saturating_sub(spacing).max(1);
    let total: usize = widths.iter().sum();
    if total > available && total > 0 {
        for width in &mut widths {
            *width = ((*width * available) / total).max(1);
        }
    }
    widths
        .into_iter()
        .map(|width| Constraint::Length(width.min(u16::MAX as usize) as u16))
        .collect()
}
