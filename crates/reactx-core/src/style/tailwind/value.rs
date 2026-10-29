//! Shared value parsers for the utility handlers.

use crate::color::TuiColor;
use crate::style::Dim;

use super::palette;

pub(super) fn bracket(value: &str) -> Option<&str> {
    value.strip_prefix('[').and_then(|v| v.strip_suffix(']'))
}

/// Tailwind spacing scale: `2` is two cells, `0.5` half a cell, `px` one.
pub(super) fn scale(value: &str) -> Option<f32> {
    match bracket(value) {
        Some(inner) => scale_number(inner),
        None if value == "px" => Some(1.0),
        None => scale_number(value),
    }
}

fn scale_number(value: &str) -> Option<f32> {
    let value = value.trim();
    let value = value.strip_suffix("px").unwrap_or(value);
    value
        .parse::<f32>()
        .ok()
        .filter(|value| value.is_finite() && *value >= 0.0)
}

pub(super) fn dimension(value: &str) -> Option<Dim> {
    if let Some(inner) = bracket(value) {
        let inner = inner.trim();
        if inner.ends_with('%') || inner.eq_ignore_ascii_case("auto") {
            return Dim::parse(inner);
        }
        return scale_number(inner).map(Dim::Cells);
    }
    match value {
        "full" | "screen" => Some(Dim::Pct(100.0)),
        "auto" | "fit" | "min" | "max" => Some(Dim::Auto),
        _ => {
            if let Some((numerator, denominator)) = value.split_once('/') {
                let numerator = numerator.parse::<f32>().ok()?;
                let denominator = denominator.parse::<f32>().ok()?;
                return (denominator != 0.0)
                    .then(|| Dim::Pct((numerator / denominator * 100.0).clamp(0.0, 100.0)));
            }
            scale(value).map(Dim::Cells)
        }
    }
}

pub(super) fn color(value: &str) -> Option<TuiColor> {
    match bracket(value) {
        Some(inner) => TuiColor::parse(inner),
        None => palette::parse(value),
    }
}
