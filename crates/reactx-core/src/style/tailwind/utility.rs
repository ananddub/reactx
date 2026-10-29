//! Utility dispatch shared by every class token.

use crate::style::StyleProps;

use super::{animate, layout, spacing, visual};

pub(super) fn apply(props: &mut StyleProps, token: &str) {
    let token = token.strip_prefix('!').unwrap_or(token);
    if let Some(rest) = token.strip_prefix('-') {
        spacing::apply_negative(props, rest);
        return;
    }
    if animate::apply(props, token) {
        return;
    }
    if layout::apply(props, token) {
        return;
    }
    if visual::apply(props, token) {
        return;
    }
    spacing::apply(props, token);
}
