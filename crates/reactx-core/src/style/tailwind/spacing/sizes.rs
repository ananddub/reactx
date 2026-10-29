//! Size, gap and inset utilities.

use crate::style::StyleProps;

use super::super::value;
use super::edges::{set_inset, Edge};

pub(super) fn apply(props: &mut StyleProps, token: &str) -> bool {
    if let Some(rest) = token.strip_prefix("size-") {
        if let Some(dim) = value::dimension(rest) {
            props.width = Some(dim);
            props.height = Some(dim);
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("gap-x-") {
        if let Some(v) = value::scale(rest) {
            let row = props.gap.map(|gap| gap.0).unwrap_or(0.0);
            props.gap = Some((row, v));
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("gap-y-") {
        if let Some(v) = value::scale(rest) {
            let col = props.gap.map(|gap| gap.1).unwrap_or(0.0);
            props.gap = Some((v, col));
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("gap-") {
        if let Some(v) = value::scale(rest) {
            props.gap = Some((v, v));
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("space-x-") {
        if let Some(v) = value::scale(rest) {
            let row = props.gap.map(|gap| gap.0).unwrap_or(0.0);
            props.gap = Some((row, v));
        }
        return true;
    }
    if let Some(rest) = token.strip_prefix("space-y-") {
        if let Some(v) = value::scale(rest) {
            let col = props.gap.map(|gap| gap.1).unwrap_or(0.0);
            props.gap = Some((v, col));
        }
        return true;
    }

    if let Some(rest) = token.strip_prefix("min-w-") {
        set_dim(rest, |dim| props.min_width = Some(dim));
        return true;
    }
    if let Some(rest) = token.strip_prefix("min-h-") {
        set_dim(rest, |dim| props.min_height = Some(dim));
        return true;
    }
    if let Some(rest) = token.strip_prefix("max-w-") {
        set_dim(rest, |dim| props.max_width = Some(dim));
        return true;
    }
    if let Some(rest) = token.strip_prefix("max-h-") {
        set_dim(rest, |dim| props.max_height = Some(dim));
        return true;
    }
    if let Some(rest) = token.strip_prefix("w-") {
        set_dim(rest, |dim| props.width = Some(dim));
        return true;
    }
    if let Some(rest) = token.strip_prefix("h-") {
        set_dim(rest, |dim| props.height = Some(dim));
        return true;
    }

    if let Some(rest) = token.strip_prefix("inset-x-") {
        set_inset(props, Edge::X, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("inset-y-") {
        set_inset(props, Edge::Y, value::scale(rest));
        return true;
    }
    if let Some(rest) = token.strip_prefix("inset-") {
        set_inset(props, Edge::All, value::scale(rest));
        return true;
    }
    for (prefix, edge) in [
        ("top-", Edge::Top),
        ("right-", Edge::Right),
        ("bottom-", Edge::Bottom),
        ("left-", Edge::Left),
    ] {
        if let Some(rest) = token.strip_prefix(prefix) {
            set_inset(props, edge, value::scale(rest));
            return true;
        }
    }

    false
}

fn set_dim(value: &str, apply: impl FnOnce(crate::style::Dim)) {
    if let Some(dim) = value::dimension(value) {
        apply(dim);
    }
}
