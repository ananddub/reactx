//! Value parsers shared by the `animate*` attribute specs.

use tachyonfx::fx::RepeatMode;
use tachyonfx::{Interpolation, Motion};

/// Milliseconds from `250`, `250ms` or `0.25s`.
pub(crate) fn parse_ms(value: &str) -> Option<u32> {
    let value = value.trim();
    if let Some(ms) = value.strip_suffix("ms") {
        return ms.trim().parse().ok();
    }
    if let Some(seconds) = value.strip_suffix('s') {
        return seconds
            .trim()
            .parse::<f32>()
            .ok()
            .map(|seconds| (seconds * 1000.0) as u32);
    }
    value.parse().ok()
}

/// Comma-separated effect names, normalized to lowercase; `none` is dropped.
pub(crate) fn parse_names(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|name| name.trim().to_ascii_lowercase())
        .filter(|name| !name.is_empty() && name != "none")
        .collect()
}

/// `3`, `3x` or `forever`/`infinite`. One round trip is not a repeat.
pub(crate) fn parse_repeat(value: &str) -> Option<RepeatMode> {
    let value = value.trim().to_ascii_lowercase();
    match value.as_str() {
        "forever" | "infinite" | "inf" | "loop" => Some(RepeatMode::Forever),
        _ => {
            let times: u32 = value.trim_end_matches('x').trim().parse().ok()?;
            (times > 1).then_some(RepeatMode::Times(times))
        }
    }
}

pub(crate) fn parse_bool(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

pub(crate) fn parse_direction(value: &str) -> Motion {
    match value.trim().to_ascii_lowercase().replace('_', "-").as_str() {
        "right" | "right-to-left" | "rtl" => Motion::RightToLeft,
        "up" | "up-to-down" | "top-down" => Motion::UpToDown,
        "down" | "down-to-up" | "bottom-up" => Motion::DownToUp,
        _ => Motion::LeftToRight,
    }
}

pub(crate) fn parse_easing(value: &str) -> Interpolation {
    use Interpolation::*;
    match value.trim().to_ascii_lowercase().replace('_', "-").as_str() {
        "linear" => Linear,
        "reverse" => Reverse,
        "quad-in" => QuadIn,
        "quad-out" => QuadOut,
        "quad-in-out" => QuadInOut,
        "cubic-in" => CubicIn,
        "cubic-out" => CubicOut,
        "cubic-in-out" => CubicInOut,
        "quart-in" => QuartIn,
        "quart-out" => QuartOut,
        "quart-in-out" => QuartInOut,
        "quint-in" => QuintIn,
        "quint-out" => QuintOut,
        "quint-in-out" => QuintInOut,
        "sine-in" => SineIn,
        "sine-out" => SineOut,
        "sine-in-out" => SineInOut,
        "expo-in" => ExpoIn,
        "expo-out" => ExpoOut,
        "expo-in-out" => ExpoInOut,
        "circ-in" => CircIn,
        "circ-out" => CircOut,
        "circ-in-out" => CircInOut,
        "back-in" => BackIn,
        "back-out" => BackOut,
        "back-in-out" => BackInOut,
        "bounce-in" => BounceIn,
        "bounce-out" => BounceOut,
        "bounce-in-out" => BounceInOut,
        "elastic-in" => ElasticIn,
        "elastic-out" => ElasticOut,
        "elastic-in-out" => ElasticInOut,
        _ => QuadOut,
    }
}
