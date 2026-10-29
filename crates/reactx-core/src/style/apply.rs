//! Attribute application and removal for [`StyleProps`].

use super::parse::{normalize, parse_bool, parse_cells, parse_cells_opt, parse_scroll};
use super::*;
use crate::color::TuiColor;

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
            "title" => self.title = Some(text.to_string()),
            "title_align" => self.title_align = TitleAlign::parse(value),
            "text_align" | "align_text" => self.text_align = TextAlign::parse(value),
            "bold" => self.bold = parse_bool(value),
            "italic" => self.italic = parse_bool(value),
            "underline" | "underlined" => self.underline = parse_bool(value),
            "dim" | "dimmed" => self.dim = parse_bool(value),
            "strikethrough" | "crossed_out" => self.strikethrough = parse_bool(value),
            "reversed" | "reverse" => self.reversed = parse_bool(value),
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

    /// Remove an attribute previously set through [`Self::apply_attr`].
    pub fn clear_attr(&mut self, name: &str) {
        let name = normalize(name);
        match name.as_str() {
            "direction" | "flex_direction" => self.direction = None,
            "justify" | "justify_content" => self.justify = None,
            "align" | "align_items" => self.align = None,
            "align_self" => self.align_self = None,
            "flex_grow" | "grow" => self.flex_grow = None,
            "flex_shrink" | "shrink" => self.flex_shrink = None,
            "width" => self.width = None,
            "height" => self.height = None,
            "min_width" => self.min_width = None,
            "min_height" => self.min_height = None,
            "max_width" => self.max_width = None,
            "max_height" => self.max_height = None,
            "padding" => self.padding = Edges::default(),
            "padding_top" => self.padding.top = 0.0,
            "padding_right" => self.padding.right = 0.0,
            "padding_bottom" => self.padding.bottom = 0.0,
            "padding_left" => self.padding.left = 0.0,
            "margin" => self.margin = Edges::default(),
            "margin_top" => self.margin.top = 0.0,
            "margin_right" => self.margin.right = 0.0,
            "margin_bottom" => self.margin.bottom = 0.0,
            "margin_left" => self.margin.left = 0.0,
            "gap" | "gap_row" | "row_gap" | "gap_col" | "column_gap" | "col_gap" => self.gap = None,
            "position" => self.position = None,
            "top" => self.top = None,
            "right" => self.right = None,
            "bottom" => self.bottom = None,
            "left" => self.left = None,
            "display" => self.display = None,
            "overflow" => self.overflow = None,
            "scroll_x" => self.scroll_x = 0.0,
            "scroll_y" => self.scroll_y = 0.0,
            "layer" | "z_index" | "z" => self.layer = None,
            "focus_trap" | "trap_focus" => self.focus_trap = false,
            "autofocus" | "auto_focus" => self.autofocus = false,
            "color" | "fg" | "foreground" => self.color = None,
            "bg" | "background" | "background_color" => self.bg = None,
            "border" => self.border = None,
            "border_color" => self.border_color = None,
            "title" => self.title = None,
            "title_align" => self.title_align = None,
            "text_align" | "align_text" => self.text_align = None,
            "bold" => self.bold = false,
            "italic" => self.italic = false,
            "underline" | "underlined" => self.underline = false,
            "dim" | "dimmed" => self.dim = false,
            "strikethrough" | "crossed_out" => self.strikethrough = false,
            "reversed" | "reverse" => self.reversed = false,
            "focusable" => self.focusable = None,
            "disabled" => self.disabled = false,
            "value" | "input_value" => self.value = None,
            "placeholder" => self.placeholder = None,
            "label" => self.label = None,
            "checked" => self.checked = None,
            "variant" => self.variant = None,
            _ => {
                self.custom.remove(&name);
            }
        }
    }
}
