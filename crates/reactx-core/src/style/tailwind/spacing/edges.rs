//! Edge setters shared by the spacing utilities.

use crate::style::{Dim, Edges, StyleProps};

#[derive(Clone, Copy)]
pub(super) enum Edge {
    All,
    X,
    Y,
    Top,
    Right,
    Bottom,
    Left,
}

pub(super) fn set_padding(props: &mut StyleProps, edge: Edge, value: Option<f32>) {
    let Some(value) = value else {
        return;
    };
    let padding = &mut props.padding;
    match edge {
        Edge::All => *padding = Edges::uniform(value),
        Edge::X => {
            padding.left = value;
            padding.right = value;
        }
        Edge::Y => {
            padding.top = value;
            padding.bottom = value;
        }
        Edge::Top => padding.top = value,
        Edge::Right => padding.right = value,
        Edge::Bottom => padding.bottom = value,
        Edge::Left => padding.left = value,
    }
}

pub(super) fn set_margin(props: &mut StyleProps, edge: Edge, value: Option<f32>) {
    let Some(value) = value else {
        return;
    };
    let margin = &mut props.margin;
    match edge {
        Edge::All => *margin = Edges::uniform(value),
        Edge::X => {
            margin.left = value;
            margin.right = value;
        }
        Edge::Y => {
            margin.top = value;
            margin.bottom = value;
        }
        Edge::Top => margin.top = value,
        Edge::Right => margin.right = value,
        Edge::Bottom => margin.bottom = value,
        Edge::Left => margin.left = value,
    }
}

pub(super) fn set_inset(props: &mut StyleProps, edge: Edge, value: Option<f32>) {
    let Some(value) = value else {
        return;
    };
    let dim = Dim::Cells(value);
    match edge {
        Edge::All => {
            props.top = Some(dim);
            props.right = Some(dim);
            props.bottom = Some(dim);
            props.left = Some(dim);
        }
        Edge::X => {
            props.left = Some(dim);
            props.right = Some(dim);
        }
        Edge::Y => {
            props.top = Some(dim);
            props.bottom = Some(dim);
        }
        Edge::Top => props.top = Some(dim),
        Edge::Right => props.right = Some(dim),
        Edge::Bottom => props.bottom = Some(dim),
        Edge::Left => props.left = Some(dim),
    }
}
