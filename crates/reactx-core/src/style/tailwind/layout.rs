//! Layout utilities: display, flex, alignment, position and order.

use crate::style::{
    Align, Dim, Display, FlexDir, FlexWrap, Justify, Overflow, Position, StyleProps,
};

use super::value;

pub(super) fn apply(props: &mut StyleProps, token: &str) -> bool {
    if let Some(rest) = token.strip_prefix("place-items-") {
        if let Some(value) = align(rest) {
            props.align = Some(value);
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("place-content-") {
        if let Some(value) = content(rest) {
            props.align_content = Some(value);
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("place-self-") {
        props.align_self = if rest == "auto" { None } else { align(rest) };
        return true;
    }
    if let Some(rest) = token.strip_prefix("justify-") {
        if let Some(value) = justify(rest) {
            props.justify = Some(value);
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("items-") {
        if let Some(value) = align(rest) {
            props.align = Some(value);
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("self-") {
        props.align_self = if rest == "auto" { None } else { align(rest) };
        return true;
    }
    if let Some(rest) = token.strip_prefix("content-") {
        if let Some(value) = content(rest) {
            props.align_content = Some(value);
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("basis-") {
        if let Some(value) = value::dimension(rest) {
            props.flex_basis = Some(value);
        }
        return true;
    }
    if let Some(rest) = token
        .strip_prefix("grow-")
        .or_else(|| token.strip_prefix("flex-grow-"))
    {
        if let Some(factor) = value::scale(rest) {
            props.flex_grow = Some(factor);
        }
        return true;
    }
    if let Some(rest) = token
        .strip_prefix("shrink-")
        .or_else(|| token.strip_prefix("flex-shrink-"))
    {
        if let Some(factor) = value::scale(rest) {
            props.flex_shrink = Some(factor);
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("order-") {
        if let Some(value) = order(rest) {
            props.order = Some(value);
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("z-") {
        props.layer = if rest == "auto" {
            None
        } else {
            let raw = value::bracket(rest).unwrap_or(rest);
            raw.parse::<i32>().ok()
        };
        return true;
    }
    if let Some(rest) = token.strip_prefix("overflow-") {
        if let Some(value) = Overflow::parse(rest) {
            props.overflow = Some(value);
        }
        return true;
    }

    match token {
        "flex" | "block" | "inline" | "inline-block" | "inline-flex" => {
            props.display = Some(Display::Flex);
        }
        "hidden" => props.display = Some(Display::None),
        "flex-row" => props.direction = Some(FlexDir::Row),
        "flex-row-reverse" => props.direction = Some(FlexDir::RowReverse),
        "flex-col" => props.direction = Some(FlexDir::Column),
        "flex-col-reverse" => props.direction = Some(FlexDir::ColumnReverse),
        "flex-wrap" => props.flex_wrap = Some(FlexWrap::Wrap),
        "flex-wrap-reverse" => props.flex_wrap = Some(FlexWrap::WrapReverse),
        "flex-nowrap" => props.flex_wrap = Some(FlexWrap::NoWrap),
        "flex-1" => {
            props.flex_grow = Some(1.0);
            props.flex_shrink = Some(1.0);
            props.flex_basis = Some(Dim::Cells(0.0));
        }
        "flex-auto" => {
            props.flex_grow = Some(1.0);
            props.flex_shrink = Some(1.0);
            props.flex_basis = Some(Dim::Auto);
        }
        "flex-initial" => {
            props.flex_grow = Some(0.0);
            props.flex_shrink = Some(1.0);
            props.flex_basis = Some(Dim::Auto);
        }
        "flex-none" => {
            props.flex_grow = Some(0.0);
            props.flex_shrink = Some(0.0);
            props.flex_basis = Some(Dim::Auto);
        }
        "grow" | "flex-grow" => props.flex_grow = Some(1.0),
        "shrink" | "flex-shrink" => props.flex_shrink = Some(1.0),
        "relative" | "static" => props.position = Some(Position::Relative),
        "absolute" | "fixed" => props.position = Some(Position::Absolute),
        _ => return false,
    }
    true
}

fn justify(value: &str) -> Option<Justify> {
    Some(match value {
        "start" | "flex-start" | "normal" => Justify::Start,
        "center" => Justify::Center,
        "end" | "flex-end" => Justify::End,
        "between" | "space-between" => Justify::SpaceBetween,
        "around" | "space-around" => Justify::SpaceAround,
        "evenly" | "space-evenly" => Justify::SpaceEvenly,
        _ => return None,
    })
}

/// `content-*` shares `justify`'s values plus the line-pack keywords.
fn content(value: &str) -> Option<Justify> {
    match value {
        "baseline" | "stretch" => Some(Justify::Start),
        _ => justify(value),
    }
}

fn align(value: &str) -> Option<Align> {
    Some(match value {
        "start" | "flex-start" | "baseline" => Align::Start,
        "center" => Align::Center,
        "end" | "flex-end" => Align::End,
        "stretch" | "normal" => Align::Stretch,
        _ => return None,
    })
}

fn order(value: &str) -> Option<i32> {
    match value {
        "first" => Some(-9999),
        "last" => Some(9999),
        "none" => Some(0),
        _ => value::bracket(value).unwrap_or(value).parse().ok(),
    }
}
