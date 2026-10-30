//! Attribute removal for [`StyleProps`].

use crate::style::parse::normalize;
use crate::style::*;

impl StyleProps {
    /// Remove an attribute previously set through [`StyleProps::apply_attr`].
    pub fn clear_attr(&mut self, name: &str) {
        let name = normalize(name);
        match name.as_str() {
            "direction" | "flex_direction" => self.direction = None,
            "justify" | "justify_content" => self.justify = None,
            "align" | "align_items" => self.align = None,
            "align_self" => self.align_self = None,
            "align_content" | "content" => self.align_content = None,
            "flex_wrap" | "wrap" => self.flex_wrap = None,
            "flex_basis" | "basis" => self.flex_basis = None,
            "order" => self.order = None,
            "flex_grow" | "grow" => self.flex_grow = None,
            "flex_shrink" | "shrink" => self.flex_shrink = None,
            "width" => self.width = None,
            "height" => self.height = None,
            "min_width" => self.min_width = None,
            "min_height" => self.min_height = None,
            "max_width" => self.max_width = None,
            "max_height" => self.max_height = None,
            "aspect_ratio" => self.aspect_ratio = None,
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
            "border_top_color" => self.border_top_color = None,
            "border_right_color" => self.border_right_color = None,
            "border_bottom_color" => self.border_bottom_color = None,
            "border_left_color" => self.border_left_color = None,
            "border_dim_color" | "border_dim" => self.border_dim = false,
            "title" => self.title = None,
            "title_align" => self.title_align = None,
            "text_align" | "align_text" => self.text_align = None,
            "text_wrap" | "text_overflow" | "wrap_text" => self.text_wrap = None,
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
