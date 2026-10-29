//! Negative margin and inset utilities.

use crate::style::StyleProps;

use super::super::value;
use super::edges::{set_inset, set_margin, Edge};

/// Negative utilities only exist for margins and insets.
pub(crate) fn apply_negative(props: &mut StyleProps, token: &str) {
    let negate = |value: Option<f32>| value.map(|value| -value);

    if let Some(rest) = token.strip_prefix("ms-") {
        set_margin(props, Edge::Left, negate(value::scale(rest)));
        return;
    }
    if let Some(rest) = token.strip_prefix("me-") {
        set_margin(props, Edge::Right, negate(value::scale(rest)));
        return;
    }
    if let Some(rest) = token.strip_prefix("mx-") {
        set_margin(props, Edge::X, negate(value::scale(rest)));
        return;
    }
    if let Some(rest) = token.strip_prefix("my-") {
        set_margin(props, Edge::Y, negate(value::scale(rest)));
        return;
    }
    if let Some(rest) = token.strip_prefix("mt-") {
        set_margin(props, Edge::Top, negate(value::scale(rest)));
        return;
    }
    if let Some(rest) = token.strip_prefix("mr-") {
        set_margin(props, Edge::Right, negate(value::scale(rest)));
        return;
    }
    if let Some(rest) = token.strip_prefix("mb-") {
        set_margin(props, Edge::Bottom, negate(value::scale(rest)));
        return;
    }
    if let Some(rest) = token.strip_prefix("ml-") {
        set_margin(props, Edge::Left, negate(value::scale(rest)));
        return;
    }
    if let Some(rest) = token.strip_prefix("m-") {
        set_margin(props, Edge::All, negate(value::scale(rest)));
        return;
    }

    if let Some(rest) = token.strip_prefix("inset-x-") {
        set_inset(props, Edge::X, negate(value::scale(rest)));
        return;
    }
    if let Some(rest) = token.strip_prefix("inset-y-") {
        set_inset(props, Edge::Y, negate(value::scale(rest)));
        return;
    }
    if let Some(rest) = token.strip_prefix("inset-") {
        set_inset(props, Edge::All, negate(value::scale(rest)));
        return;
    }
    for (prefix, edge) in [
        ("top-", Edge::Top),
        ("right-", Edge::Right),
        ("bottom-", Edge::Bottom),
        ("left-", Edge::Left),
    ] {
        if let Some(rest) = token.strip_prefix(prefix) {
            set_inset(props, edge, negate(value::scale(rest)));
            return;
        }
    }
}
