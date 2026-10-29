//! Vector canvas and its shape types.

use dioxus::prelude::*;
use reactx_core::wire;

use super::{border_style, css, title_style};
use crate::theme::use_theme;
use crate::widgets::style_push;

/// A shape drawn by [`Canvas`] in virtual coordinates.
#[derive(Debug, Clone, PartialEq)]
pub enum CanvasShape {
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: Option<String>,
    },
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: Option<String>,
    },
    Circle {
        x: f64,
        y: f64,
        radius: f64,
        color: Option<String>,
    },
    Points {
        coords: Vec<(f64, f64)>,
        color: Option<String>,
    },
    WorldMap {
        color: Option<String>,
    },
    Print {
        x: f64,
        y: f64,
        text: String,
        color: Option<String>,
    },
}

impl CanvasShape {
    fn encode(&self) -> Vec<String> {
        match self {
            Self::Line {
                x1,
                y1,
                x2,
                y2,
                color,
            } => vec![
                "line".into(),
                x1.to_string(),
                y1.to_string(),
                x2.to_string(),
                y2.to_string(),
                color.clone().unwrap_or_default(),
            ],
            Self::Rect {
                x,
                y,
                width,
                height,
                color,
            } => vec![
                "rect".into(),
                x.to_string(),
                y.to_string(),
                width.to_string(),
                height.to_string(),
                color.clone().unwrap_or_default(),
            ],
            Self::Circle {
                x,
                y,
                radius,
                color,
            } => vec![
                "circle".into(),
                x.to_string(),
                y.to_string(),
                radius.to_string(),
                color.clone().unwrap_or_default(),
            ],
            Self::Points { coords, color } => {
                let mut fields = vec!["points".into(), color.clone().unwrap_or_default()];
                fields.extend(coords.iter().map(|(x, y)| format!("{x},{y}")));
                fields
            }
            Self::WorldMap { color } => {
                vec!["map".into(), color.clone().unwrap_or_default()]
            }
            Self::Print { x, y, text, color } => vec![
                "print".into(),
                x.to_string(),
                y.to_string(),
                text.clone(),
                color.clone().unwrap_or_default(),
            ],
        }
    }
}

/// A vector canvas for lines, rectangles, circles, points, text and the world
/// map. `marker` is `braille` (default), `dot`, `block` or `halfblock`.
#[component]
pub fn Canvas(
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
    #[props(default)] shapes: Vec<CanvasShape>,
    #[props(default)] marker: String,
    #[props(default)] title: String,
    #[props(default)] border: String,
) -> Element {
    let theme = use_theme();
    let records: Vec<Vec<String>> = shapes.iter().map(CanvasShape::encode).collect();

    let mut style = String::new();
    style_push(&mut style, "variant", "canvas");
    style_push(&mut style, "canvas-xbounds", &wire::join_pair(x_bounds));
    style_push(&mut style, "canvas-ybounds", &wire::join_pair(y_bounds));
    style_push(&mut style, "canvas-shapes", &wire::join_rows(&records));
    style_push(&mut style, "canvas-marker", &marker);
    style_push(&mut style, "color", &css(theme.overlay));
    title_style(&mut style, &title);
    border_style(&mut style, &border, &theme);

    rsx! {
        div { style: style }
    }
}
