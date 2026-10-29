//! Taffy style conversion from terminal style props.

use reactx_core::{Align, Dim, Edges, FlexDir, Justify, Overflow, Position, TuiNode};
use taffy::geometry::{Point, Rect as TaffyRect};
use taffy::prelude::*;
use taffy::style::{
    AlignItems, AlignSelf, Display, FlexDirection, JustifyContent, Overflow as TaffyOverflow,
};

use super::leaf::is_text_input;
use super::metrics::{default_widget_height, effective_border};
use super::text::widget_kind;

pub(super) fn taffy_style(node: &TuiNode, is_leaf: bool, is_root: bool) -> Style {
    let attrs = &node.attrs;
    let mut style = Style {
        display: if node.is_hidden() {
            Display::None
        } else {
            Display::Flex
        },
        ..Style::default()
    };

    style.flex_direction = match attrs.direction.unwrap_or(FlexDir::Column) {
        FlexDir::Row => FlexDirection::Row,
        FlexDir::RowReverse => FlexDirection::RowReverse,
        FlexDir::Column => FlexDirection::Column,
        FlexDir::ColumnReverse => FlexDirection::ColumnReverse,
    };

    style.justify_content = attrs.justify.map(|justify| match justify {
        Justify::Start => JustifyContent::Start,
        Justify::Center => JustifyContent::Center,
        Justify::End => JustifyContent::End,
        Justify::SpaceBetween => JustifyContent::SpaceBetween,
        Justify::SpaceAround => JustifyContent::SpaceAround,
        Justify::SpaceEvenly => JustifyContent::SpaceEvenly,
    });

    style.align_items = attrs.align.map(|align| match align {
        Align::Start => AlignItems::Start,
        Align::Center => AlignItems::Center,
        Align::End => AlignItems::End,
        Align::Stretch => AlignItems::Stretch,
    });

    style.align_self = attrs.align_self.map(|align| match align {
        Align::Start => AlignSelf::Start,
        Align::Center => AlignSelf::Center,
        Align::End => AlignSelf::End,
        Align::Stretch => AlignSelf::Stretch,
    });

    style.flex_grow = attrs.flex_grow.unwrap_or(0.0);
    style.flex_shrink = attrs.flex_shrink.unwrap_or(if is_leaf { 0.0 } else { 1.0 });

    style.size = Size {
        width: dimension(attrs.width),
        height: dimension(attrs.height),
    };
    style.min_size = Size {
        width: dimension(attrs.min_width),
        height: dimension(attrs.min_height),
    };
    style.max_size = Size {
        width: dimension(attrs.max_width),
        height: dimension(attrs.max_height),
    };

    let kind = widget_kind(node);
    if is_text_input(node) && attrs.width.is_none() && attrs.min_width.is_none() {
        style.min_size.width = Dimension::Length(10.0);
    }
    if node.tag == "spacer" && attrs.flex_grow.is_none() {
        style.flex_grow = 1.0;
    }
    if attrs.height.is_none() && attrs.min_height.is_none() {
        if let Some(height) = default_widget_height(node, kind) {
            style.size.height = Dimension::Length(height);
        }
    }
    if kind == "scrollbar" && attrs.width.is_none() && attrs.min_width.is_none() {
        style.size.width = Dimension::Length(1.0);
        if attrs.height.is_none() && attrs.min_height.is_none() {
            style.size.height = Dimension::Percent(1.0);
        }
    }

    style.padding = rect(attrs.padding);
    style.margin = margin(attrs.margin);

    if let Some((row, column)) = attrs.gap {
        style.gap = Size {
            width: LengthPercentage::Length(column.max(0.0)),
            height: LengthPercentage::Length(row.max(0.0)),
        };
    }

    style.position = match attrs.position.unwrap_or(Position::Relative) {
        Position::Relative => taffy::style::Position::Relative,
        Position::Absolute => taffy::style::Position::Absolute,
    };
    style.inset = TaffyRect {
        left: auto_dimension(attrs.left),
        right: auto_dimension(attrs.right),
        top: auto_dimension(attrs.top),
        bottom: auto_dimension(attrs.bottom),
    };

    style.overflow = Point {
        x: match attrs.overflow.unwrap_or(Overflow::Visible) {
            Overflow::Visible => TaffyOverflow::Visible,
            Overflow::Hidden => TaffyOverflow::Hidden,
        },
        y: match attrs.overflow.unwrap_or(Overflow::Visible) {
            Overflow::Visible => TaffyOverflow::Visible,
            Overflow::Hidden => TaffyOverflow::Hidden,
        },
    };

    if effective_border(node).is_visible() {
        style.border = TaffyRect {
            left: LengthPercentage::Length(1.0),
            right: LengthPercentage::Length(1.0),
            top: LengthPercentage::Length(1.0),
            bottom: LengthPercentage::Length(1.0),
        };
    }

    if is_root {
        style.size = Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        };
        style.flex_direction = FlexDirection::Column;
    }

    style
}

fn dimension(dim: Option<Dim>) -> Dimension {
    match dim.unwrap_or(Dim::Auto) {
        Dim::Auto => Dimension::Auto,
        Dim::Cells(value) => Dimension::Length(value.max(0.0)),
        Dim::Pct(value) => Dimension::Percent((value / 100.0).clamp(0.0, 1.0)),
    }
}

fn auto_dimension(dim: Option<Dim>) -> LengthPercentageAuto {
    match dim {
        None | Some(Dim::Auto) => LengthPercentageAuto::Auto,
        Some(Dim::Cells(value)) => LengthPercentageAuto::Length(value.max(0.0)),
        Some(Dim::Pct(value)) => LengthPercentageAuto::Percent((value / 100.0).clamp(0.0, 1.0)),
    }
}

fn rect(edges: Edges) -> TaffyRect<LengthPercentage> {
    TaffyRect {
        left: LengthPercentage::Length(edges.left.max(0.0)),
        right: LengthPercentage::Length(edges.right.max(0.0)),
        top: LengthPercentage::Length(edges.top.max(0.0)),
        bottom: LengthPercentage::Length(edges.bottom.max(0.0)),
    }
}

fn margin(edges: Edges) -> TaffyRect<LengthPercentageAuto> {
    TaffyRect {
        left: LengthPercentageAuto::Length(edges.left),
        right: LengthPercentageAuto::Length(edges.right),
        top: LengthPercentageAuto::Length(edges.top),
        bottom: LengthPercentageAuto::Length(edges.bottom),
    }
}
