//! Attribute value parsers shared by the style props.

pub fn parse_bool(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "true" | "1" | "yes" | "on"
    )
}

pub(super) fn parse_scroll(value: &str) -> f32 {
    value
        .trim()
        .parse::<f32>()
        .ok()
        .filter(|value| value.is_finite())
        .unwrap_or(0.0)
        .max(0.0)
}

pub fn parse_cells(value: &str) -> f32 {
    parse_cells_opt(value).unwrap_or(0.0)
}

pub(super) fn parse_cells_opt(value: &str) -> Option<f32> {
    let value = value.trim();
    let value = value.strip_suffix("px").unwrap_or(value).trim();
    value
        .parse::<f32>()
        .ok()
        .filter(|v| v.is_finite() && *v >= 0.0)
}

pub(super) fn normalize(name: &str) -> String {
    name.trim().to_ascii_lowercase().replace('-', "_")
}
