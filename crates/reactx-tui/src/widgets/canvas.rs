//! Canvas painter: shapes, markers and coordinate bounds.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::text::Line as TextLine;
use ratatui::widgets::canvas::{
    Canvas, Circle, Line as CanvasLine, Map, MapResolution, Points, Rectangle as CanvasRect,
};
use ratatui::widgets::Widget;
use reactx_core::{wire, TuiColor, TuiNode};

use super::custom;

pub(super) fn paint_canvas(
    node: &TuiNode,
    buf: &mut Buffer,
    area: Rect,
    fg: Option<Color>,
    bg: Option<Color>,
) {
    if area.width < 2 || area.height < 2 {
        return;
    }
    let shapes = custom(node, "canvas_shapes")
        .map(wire::split_rows)
        .unwrap_or_default();
    let x_bounds = custom(node, "canvas_xbounds")
        .and_then(wire::split_pair)
        .unwrap_or((0.0, 1.0));
    let y_bounds = custom(node, "canvas_ybounds")
        .and_then(wire::split_pair)
        .unwrap_or((0.0, 1.0));
    let fallback = fg.unwrap_or(Color::Reset);
    let marker = match custom(node, "canvas_marker") {
        Some("dot") => Marker::Dot,
        Some("block") => Marker::Block,
        Some("halfblock") | Some("half-block") => Marker::HalfBlock,
        _ => Marker::Braille,
    };

    let canvas = Canvas::default()
        .x_bounds([x_bounds.0, x_bounds.1])
        .y_bounds([y_bounds.0, y_bounds.1])
        .marker(marker)
        .background_color(bg.unwrap_or(Color::Reset))
        .paint(|ctx| {
            for shape in &shapes {
                let Some(kind) = shape.first().map(String::as_str) else {
                    continue;
                };
                let field = |index: usize, default: f64| -> f64 {
                    shape
                        .get(index)
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(default)
                };
                let draw_color = |index: usize| -> Color {
                    shape
                        .get(index)
                        .filter(|value| !value.is_empty())
                        .and_then(|value| TuiColor::parse(value))
                        .map(|value| value.to_ratatui())
                        .unwrap_or(fallback)
                };
                match kind {
                    "line" => {
                        ctx.draw(&CanvasLine {
                            x1: field(1, 0.0),
                            y1: field(2, 0.0),
                            x2: field(3, 0.0),
                            y2: field(4, 0.0),
                            color: draw_color(5),
                        });
                    }
                    "rect" => {
                        ctx.draw(&CanvasRect {
                            x: field(1, 0.0),
                            y: field(2, 0.0),
                            width: field(3, 0.0),
                            height: field(4, 0.0),
                            color: draw_color(5),
                        });
                    }
                    "circle" => {
                        ctx.draw(&Circle {
                            x: field(1, 0.0),
                            y: field(2, 0.0),
                            radius: field(3, 0.0),
                            color: draw_color(4),
                        });
                    }
                    "points" => {
                        let coords: Vec<(f64, f64)> = shape
                            .iter()
                            .skip(2)
                            .filter_map(|point| wire::split_pair(point))
                            .collect();
                        if !coords.is_empty() {
                            ctx.draw(&Points {
                                coords: &coords,
                                color: draw_color(1),
                            });
                        }
                    }
                    "map" => {
                        ctx.draw(&Map {
                            resolution: MapResolution::High,
                            color: draw_color(1),
                        });
                    }
                    "print" => {
                        let text = shape.get(3).cloned().unwrap_or_default();
                        let style = Style::default().fg(draw_color(4));
                        ctx.print(field(1, 0.0), field(2, 0.0), TextLine::styled(text, style));
                    }
                    _ => {}
                }
            }
        });
    canvas.render(area, buf);
}
