//! Continuous line chart and its dataset type.

use dioxus::prelude::*;
use reactx_core::wire;

use super::{border_style, css, title_style};
use crate::theme::use_theme;
use crate::widgets::style_push;

/// One line of a [`Chart`].
#[derive(Debug, Clone, PartialEq)]
pub struct ChartDataset {
    pub name: String,
    pub data: Vec<(f64, f64)>,
    pub color: Option<String>,
    /// `dot` (default), `braille` or `block`.
    pub marker: String,
}

impl ChartDataset {
    pub fn new(name: impl Into<String>, data: Vec<(f64, f64)>) -> Self {
        Self {
            name: name.into(),
            data,
            color: None,
            marker: String::new(),
        }
    }
}

fn encode_points(points: &[(f64, f64)]) -> String {
    wire::join_points(points)
}

/// A continuous 2D line chart with axis bounds and labels.
#[component]
pub fn Chart(
    datasets: Vec<ChartDataset>,
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
    #[props(default)] x_labels: Option<Vec<String>>,
    #[props(default)] y_labels: Option<Vec<String>>,
    #[props(default)] title: String,
    #[props(default)] border: String,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
) -> Element {
    let theme = use_theme();
    let records: Vec<Vec<String>> = datasets
        .iter()
        .map(|dataset| {
            vec![
                dataset.name.clone(),
                dataset.color.clone().unwrap_or_default(),
                dataset.marker.clone(),
                encode_points(&dataset.data),
            ]
        })
        .collect();

    let mut style = String::new();
    style_push(&mut style, "variant", "chart");
    style_push(&mut style, "chart-datasets", &wire::join_rows(&records));
    style_push(&mut style, "chart-xbounds", &wire::join_pair(x_bounds));
    style_push(&mut style, "chart-ybounds", &wire::join_pair(y_bounds));
    if let Some(labels) = x_labels {
        style_push(&mut style, "chart-xlabels", &wire::join_fields(&labels));
    }
    if let Some(labels) = y_labels {
        style_push(&mut style, "chart-ylabels", &wire::join_fields(&labels));
    }
    style_push(&mut style, "color", &css(theme.subtext));
    title_style(&mut style, &title);
    border_style(&mut style, &border, &theme);

    rsx! {
        div { class: class, style: style }
    }
}
