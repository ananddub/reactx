//! Attribute setters for [`StyleProps`].

use crate::color::TuiColor;
use crate::style::parse::{normalize, parse_bool, parse_cells, parse_cells_opt, parse_scroll};
use crate::style::*;

impl StyleProps {
    pub fn apply_attr(&mut self, name: &str, value: &str) {
        let name = normalize(name);
        let text = value;
        let value = value.trim();
        match name.as_str() {
            "direction" | "flex_direction" => {
                if let Some(v) = FlexDir::parse(value) {
                    self.direction = Some(v);
                }
            }
            "justify" | "justify_content" => {
                if let Some(v) = Justify::parse(value) {
                    self.justify = Some(v);
                }
            }
            "align" | "align_items" => {
                if let Some(v) = Align::parse(value) {
                    self.align = Some(v);
                }
            }
            "align_self" => {
                if let Some(v) = Align::parse(value) {
                    self.align_self = Some(v);
                }
            }
            "align_content" | "content" => {
                if let Some(v) = Justify::parse(value) {
                    self.align_content = Some(v);
                }
            }
            "flex_wrap" | "wrap" => {
                self.flex_wrap = FlexWrap::parse(value);
                if self.flex_wrap.is_none() {
                    self.text_wrap = TextWrap::parse(value);
                }
            }
            "flex_basis" | "basis" => self.flex_basis = Dim::parse(value),
            "order" => {
                if let Ok(v) = value.parse::<i32>() {
                    self.order = Some(v);
                }
            }
            "flex_grow" | "grow" => {
                if let Ok(v) = value.parse::<f32>() {
                    self.flex_grow = Some(v.max(0.0));
                }
            }
            "flex_shrink" | "shrink" => {
                if let Ok(v) = value.parse::<f32>() {
                    self.flex_shrink = Some(v.max(0.0));
                }
            }
            "width" => self.width = Dim::parse(value),
            "height" => self.height = Dim::parse(value),
            "min_width" => self.min_width = Dim::parse(value),
            "min_height" => self.min_height = Dim::parse(value),
            "max_width" => self.max_width = Dim::parse(value),
            "max_height" => self.max_height = Dim::parse(value),
            "aspect_ratio" => {
                self.aspect_ratio = value
                    .parse::<f32>()
                    .ok()
                    .filter(|ratio| ratio.is_finite() && *ratio > 0.0);
            }
            "padding" => {
                if let Some(v) = Edges::parse(value) {
                    self.padding = v;
                }
            }
            "padding_top" => self.padding.top = parse_cells(value),
            "padding_right" => self.padding.right = parse_cells(value),
            "padding_bottom" => self.padding.bottom = parse_cells(value),
            "padding_left" => self.padding.left = parse_cells(value),
            "margin" => {
                if let Some(v) = Edges::parse(value) {
                    self.margin = v;
                }
            }
            "margin_top" => self.margin.top = parse_cells(value),
            "margin_right" => self.margin.right = parse_cells(value),
            "margin_bottom" => self.margin.bottom = parse_cells(value),
            "margin_left" => self.margin.left = parse_cells(value),
            "gap" => {
                if let Some(v) = parse_cells_opt(value) {
                    self.gap = Some((v, v));
                }
            }
            "gap_row" | "row_gap" => {
                if let Some(v) = parse_cells_opt(value) {
                    let col = self.gap.map(|(_, c)| c).unwrap_or(0.0);
                    self.gap = Some((v, col));
                }
            }
            "gap_col" | "column_gap" | "col_gap" => {
                if let Some(v) = parse_cells_opt(value) {
                    let row = self.gap.map(|(r, _)| r).unwrap_or(0.0);
                    self.gap = Some((row, v));
                }
            }
            "position" => self.position = Position::parse(value),
            "top" => self.top = Dim::parse(value),
            "right" => self.right = Dim::parse(value),
            "bottom" => self.bottom = Dim::parse(value),
            "left" => self.left = Dim::parse(value),
            "display" => self.display = Display::parse(value),
            "overflow" => self.overflow = Overflow::parse(value),
            "scroll_x" => self.scroll_x = parse_scroll(value),
            "scroll_y" => self.scroll_y = parse_scroll(value),
            "layer" | "z_index" | "z" => {
                if let Ok(layer) = value.parse::<i32>() {
                    self.layer = Some(layer);
                }
            }
            "focus_trap" | "trap_focus" => self.focus_trap = parse_bool(value),
            "autofocus" | "auto_focus" => self.autofocus = parse_bool(value),
            "color" | "fg" | "foreground" => self.color = TuiColor::parse(value),
            "bg" | "background" | "background_color" => self.bg = TuiColor::parse(value),
            "border" => self.border = BorderKind::parse(value),
            "border_color" => self.border_color = TuiColor::parse(value),
            "border_top_color" => self.border_top_color = TuiColor::parse(value),
            "border_right_color" => self.border_right_color = TuiColor::parse(value),
            "border_bottom_color" => self.border_bottom_color = TuiColor::parse(value),
            "border_left_color" => self.border_left_color = TuiColor::parse(value),
            "border_dim_color" | "border_dim" => self.border_dim = parse_bool(value),
            "title" => self.title = Some(text.to_string()),
            "title_align" => self.title_align = TitleAlign::parse(value),
            "text_align" | "align_text" => self.text_align = TextAlign::parse(value),
            "text_wrap" | "text_overflow" | "wrap_text" => self.text_wrap = TextWrap::parse(value),
            "bold" => self.bold = parse_bool(value),
            "italic" => self.italic = parse_bool(value),
            "underline" | "underlined" => self.underline = parse_bool(value),
            "dim" | "dimmed" => self.dim = parse_bool(value),
            "strikethrough" | "crossed_out" => self.strikethrough = parse_bool(value),
            "reversed" | "reverse" | "inverse" => self.reversed = parse_bool(value),
            "focusable" => self.focusable = Some(parse_bool(value)),
            "disabled" => self.disabled = parse_bool(value),
            "value" | "input_value" => self.value = Some(text.to_string()),
            "placeholder" => self.placeholder = Some(text.to_string()),
            "label" => self.label = Some(text.to_string()),
            "checked" => self.checked = Some(parse_bool(value)),
            "variant" => self.variant = Some(text.to_string()),
            _ => {
                self.custom.insert(name.clone(), text.to_string());
            }
        }
    }

    /// Apply a `style="key:value;..."` declaration list. Later declarations
    /// win, matching CSS.
    pub fn apply_style_string(&mut self, text: &str) {
        for declaration in text.split(';') {
            if let Some((key, value)) = declaration.split_once(':') {
                let key = key.trim();
                if !key.is_empty() {
                    self.apply_attr(key, value.trim());
                }
            }
        }
    }
}
