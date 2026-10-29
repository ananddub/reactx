//! Spacing utilities: padding, margin, gap, sizes and insets.

mod edges;
mod negative;
mod sizes;

pub(super) use negative::apply_negative;

use super::value;
use crate::style::StyleProps;
use edges::{set_margin, set_padding, Edge};

pub(super) fn apply(props: &mut StyleProps, token: &str) -> bool {
    if sizes::apply(props, token) {
        return true;
    }

    if let Some(rest) = token.strip_prefix("ps-") {
        set_padding(props, Edge::Left, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("pe-") {
        set_padding(props, Edge::Right, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("px-") {
        set_padding(props, Edge::X, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("py-") {
        set_padding(props, Edge::Y, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("pt-") {
        set_padding(props, Edge::Top, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("pr-") {
        set_padding(props, Edge::Right, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("pb-") {
        set_padding(props, Edge::Bottom, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("pl-") {
        set_padding(props, Edge::Left, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("p-") {
        set_padding(props, Edge::All, value::scale(rest));
        return true;
    }

    if let Some(rest) = token.strip_prefix("ms-") {
        set_margin(props, Edge::Left, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("me-") {
        set_margin(props, Edge::Right, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("mx-") {
        set_margin(props, Edge::X, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("my-") {
        set_margin(props, Edge::Y, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("mt-") {
        set_margin(props, Edge::Top, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("mr-") {
        set_margin(props, Edge::Right, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("mb-") {
        set_margin(props, Edge::Bottom, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("ml-") {
        set_margin(props, Edge::Left, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("m-") {
        set_margin(props, Edge::All, value::scale(rest));
        return true;
    }

    false
}
