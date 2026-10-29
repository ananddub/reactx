//! Paint utilities: colours, borders and text modifiers.

use crate::style::{BorderKind, Overflow, StyleProps, TextAlign};

use super::value;

pub(super) fn apply(props: &mut StyleProps, token: &str) -> bool {
    match token {
        "border" => {
            if props.border.is_none() {
                props.border = Some(BorderKind::Single);
            }
            return true;
        }
        "border-none" | "border-0" => {
            props.border = Some(BorderKind::None);
            return true;
        }
        "border-single" => {
            props.border = Some(BorderKind::Single);
            return true;
        }
        "border-double" => {
            props.border = Some(BorderKind::Double);
            return true;
        }
        "border-2" | "border-4" | "border-thick" => {
            props.border = Some(BorderKind::Thick);
            return true;
        }
        "rounded" | "rounded-sm" | "rounded-md" | "rounded-lg" | "rounded-xl" | "rounded-full" => {
            props.border = Some(BorderKind::Rounded);
            return true;
        }
        "rounded-none" => {
            if props.border == Some(BorderKind::Rounded) {
                props.border = Some(BorderKind::Single);
            }
            return true;
        }
        "bold" | "font-bold" => {
            props.bold = true;
            return true;
        }
        "font-normal" | "not-bold" => {
            props.bold = false;
            return true;
        }
        "italic" => {
            props.italic = true;
            return true;
        }
        "not-italic" => {
            props.italic = false;
            return true;
        }
        "underline" => {
            props.underline = true;
            return true;
        }
        "no-underline" => {
            props.underline = false;
            return true;
        }
        "line-through" | "strikethrough" => {
            props.strikethrough = true;
            return true;
        }
        "no-line-through" => {
            props.strikethrough = false;
            return true;
        }
        "dim" => {
            props.dim = true;
            return true;
        }
        "reverse" | "reverse-video" => {
            props.reversed = true;
            return true;
        }
        "truncate" => {
            props.overflow = Some(Overflow::Hidden);
            return true;
        }
        _ => {}
    }

    if let Some(rest) = token.strip_prefix("text-") {
        if let Some(align) = TextAlign::parse(rest) {
            props.text_align = Some(align);
            return true;
        }
        if rest == "justify" {
            props.text_align = Some(TextAlign::Left);
            return true;
        }
        if let Some(color) = value::color(rest) {
            props.color = Some(color);
            return true;
        }
        return false;
    }
    if let Some(rest) = token.strip_prefix("bg-") {
        if let Some(color) = value::color(rest) {
            props.bg = Some(color);
            return true;
        }
        return false;
    }
    if let Some(rest) = token.strip_prefix("border-") {
        if let Some(color) = value::color(rest) {
            props.border_color = Some(color);
            return true;
        }
        return false;
    }
    false
}
