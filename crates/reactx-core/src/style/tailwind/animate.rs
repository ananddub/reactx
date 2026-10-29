//! Animation utilities: `animate-*`, `duration-*`, `ease-*` and `delay-*`.
//!
//! The classes write the same custom attributes the inline `style` form uses,
//! so everything the renderer understands as `animate:...` also works here:
//!
//! - `animate-<effect>` / `animate-[fade-in,hsl-shift]` (no spaces inside)
//! - `animate-exit-<effect>`
//! - `animate-repeat-<n|forever>`, `animate-ping-pong`
//! - `duration-<ms>`, `delay-<ms>`, `ease-<curve>` (also long forms with an
//!   `animate-` prefix), `animate-index-*`, `animate-stagger-*`
//! - `animate-direction-*`, `animate-color-*`, `animate-gradient-*`,
//!   `animate-key-*`
//!
//! Arbitrary values go in brackets: `duration-[450ms]`,
//! `animate-color-[#f38ba8]`. Tailwind's `ease-in`/`ease-out`/`ease-in-out`
//! map to the cubic curves.

use crate::style::StyleProps;

use super::value;

pub(super) fn apply(props: &mut StyleProps, token: &str) -> bool {
    if let Some(rest) = token.strip_prefix("duration-") {
        return set(props, "animate_duration", unbracket(rest));
    }
    if let Some(rest) = token.strip_prefix("delay-") {
        return set(props, "animate_delay", unbracket(rest));
    }
    if let Some(rest) = token.strip_prefix("ease-") {
        return set(props, "animate_easing", easing(rest));
    }
    let Some(rest) = token.strip_prefix("animate-") else {
        return false;
    };
    match rest {
        "none" => {
            props.custom.remove("animate");
            props.custom.remove("animate_exit");
            props.custom.remove("animate_repeat");
            props.custom.remove("animate_ping_pong");
            return true;
        }
        "ping-pong" => return set(props, "animate_ping_pong", "1"),
        // Tailwind's built-in loops, mapped to terminal effects.
        "spin" => return preset(props, "hsl-shift", false),
        "ping" => return preset(props, "explode", false),
        "pulse" => return preset(props, "fade", true),
        "bounce" => return preset(props, "slide-in-bottom", true),
        _ => {}
    }
    let Some((key, value)) = split(rest) else {
        return false;
    };
    set(props, key, value)
}

fn split(rest: &str) -> Option<(&'static str, String)> {
    let pair = if let Some(v) = rest.strip_prefix("exit-") {
        ("animate_exit", unbracket(v))
    } else if let Some(v) = rest.strip_prefix("repeat-") {
        ("animate_repeat", unbracket(v))
    } else if let Some(v) = rest.strip_prefix("duration-") {
        ("animate_duration", unbracket(v))
    } else if let Some(v) = rest.strip_prefix("delay-") {
        ("animate_delay", unbracket(v))
    } else if let Some(v) = rest.strip_prefix("easing-") {
        ("animate_easing", easing(v))
    } else if let Some(v) = rest.strip_prefix("index-") {
        ("animate_index", unbracket(v))
    } else if let Some(v) = rest.strip_prefix("stagger-") {
        ("animate_stagger", unbracket(v))
    } else if let Some(v) = rest.strip_prefix("direction-") {
        ("animate_direction", unbracket(v))
    } else if let Some(v) = rest.strip_prefix("gradient-") {
        ("animate_gradient", unbracket(v))
    } else if let Some(v) = rest.strip_prefix("key-") {
        ("animate_key", unbracket(v))
    } else if let Some(v) = rest.strip_prefix("color-") {
        return value::color(v).map(|color| ("animate_color", color.to_css()));
    } else {
        ("animate", unbracket(rest))
    };
    Some((pair.0, pair.1.to_string()))
}

fn set(props: &mut StyleProps, key: &str, value: impl Into<String>) -> bool {
    props.custom.insert(key.to_string(), value.into());
    true
}

/// Map a Tailwind built-in animation to one of ours, looping forever.
fn preset(props: &mut StyleProps, effect: &str, ping_pong: bool) -> bool {
    set(props, "animate", effect);
    set(props, "animate_repeat", "forever");
    if ping_pong {
        set(props, "animate_ping_pong", "1");
    }
    true
}

fn unbracket(input: &str) -> &str {
    value::bracket(input).unwrap_or(input)
}

fn easing(input: &str) -> &str {
    match unbracket(input) {
        "in" => "cubic-in",
        "out" => "cubic-out",
        "in-out" => "cubic-in-out",
        other => other,
    }
}

#[cfg(test)]
mod tests;
