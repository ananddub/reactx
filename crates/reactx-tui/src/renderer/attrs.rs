//! Attribute conversion and tag normalization.

use dioxus_core::AttributeValue;

/// Normalize element names from RSX/custom elements into the tags the painter understands.
pub fn normalize_tag(tag: &str) -> String {
    let tag = tag.trim().to_ascii_lowercase();
    let tag = tag.strip_prefix("tui-").unwrap_or(&tag);
    match tag {
        "div" | "section" | "main" | "nav" | "article" | "header" | "footer" | "view"
        | "container" | "col" | "column" => "box".to_string(),
        "span" | "p" | "label" | "paragraph" => "text".to_string(),
        other => other.to_string(),
    }
}

pub(super) fn attribute_to_string(value: &AttributeValue) -> Option<String> {
    match value {
        AttributeValue::Text(text) => Some(text.clone()),
        AttributeValue::Bool(value) => Some(value.to_string()),
        AttributeValue::Int(value) => Some(value.to_string()),
        AttributeValue::Float(value) => Some(value.to_string()),
        AttributeValue::Any(value) => {
            let any = value.as_any();
            if let Some(value) = any.downcast_ref::<String>() {
                Some(value.clone())
            } else if let Some(value) = any.downcast_ref::<&'static str>() {
                Some((*value).to_string())
            } else if let Some(value) = any.downcast_ref::<bool>() {
                Some(value.to_string())
            } else if let Some(value) = any.downcast_ref::<i32>() {
                Some(value.to_string())
            } else if let Some(value) = any.downcast_ref::<u32>() {
                Some(value.to_string())
            } else if let Some(value) = any.downcast_ref::<i64>() {
                Some(value.to_string())
            } else if let Some(value) = any.downcast_ref::<u64>() {
                Some(value.to_string())
            } else {
                any.downcast_ref::<f64>()
                    .map(|value| value.to_string())
                    .or_else(|| any.downcast_ref::<f32>().map(|value| value.to_string()))
            }
        }
        AttributeValue::None | AttributeValue::Listener(_) => None,
    }
}
