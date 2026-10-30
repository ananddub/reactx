//! Style values extracted from RSX attributes.
//!
//! The module is split into the layout/visual enums, the [`StyleProps`]
//! container and its attribute parser.

mod apply;
mod kinds_layout;
mod kinds_visual;
mod parse;
mod tailwind;

#[cfg(test)]
mod tests;

pub use kinds_layout::{Align, Display, FlexDir, FlexWrap, Justify, Overflow, Position};
pub use kinds_visual::{BorderKind, Dim, Edges, TextAlign, TextWrap, TitleAlign};
pub use parse::{parse_bool, parse_cells};
pub use tailwind::ClassState;

use crate::color::TuiColor;

/// Visual and layout properties extracted from RSX attributes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StyleProps {
    // Layout (Taffy)
    pub direction: Option<FlexDir>,
    pub justify: Option<Justify>,
    pub align: Option<Align>,
    pub align_self: Option<Align>,
    /// Alignment of wrapped lines.
    pub align_content: Option<Justify>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_wrap: Option<FlexWrap>,
    pub flex_basis: Option<Dim>,
    /// Paint/layout order among siblings (Tailwind `order-*`).
    pub order: Option<i32>,
    pub width: Option<Dim>,
    pub height: Option<Dim>,
    pub min_width: Option<Dim>,
    pub min_height: Option<Dim>,
    pub max_width: Option<Dim>,
    pub max_height: Option<Dim>,
    /// Preferred width-to-height ratio.
    pub aspect_ratio: Option<f32>,
    pub padding: Edges,
    pub margin: Edges,
    /// (row gap, column gap)
    pub gap: Option<(f32, f32)>,
    pub position: Option<Position>,
    pub top: Option<Dim>,
    pub right: Option<Dim>,
    pub bottom: Option<Dim>,
    pub left: Option<Dim>,
    pub display: Option<Display>,
    pub overflow: Option<Overflow>,
    /// Scrolled content offset in cells (only meaningful with clipped overflow).
    pub scroll_x: f32,
    pub scroll_y: f32,
    /// Paint order among siblings; higher layers paint later.
    pub layer: Option<i32>,
    /// Restrict keyboard focus and pointer events to this subtree while visible.
    pub focus_trap: bool,
    /// Preferred focus target when its focus scope opens.
    pub autofocus: bool,

    // Visual
    pub color: Option<TuiColor>,
    pub bg: Option<TuiColor>,
    pub border: Option<BorderKind>,
    pub border_color: Option<TuiColor>,
    /// Per-side overrides for [`Self::border_color`].
    pub border_top_color: Option<TuiColor>,
    pub border_right_color: Option<TuiColor>,
    pub border_bottom_color: Option<TuiColor>,
    pub border_left_color: Option<TuiColor>,
    /// Draw the border with the dim modifier.
    pub border_dim: bool,
    pub title: Option<String>,
    pub title_align: Option<TitleAlign>,
    pub text_align: Option<TextAlign>,
    /// How overflowing text is wrapped or truncated.
    pub text_wrap: Option<TextWrap>,

    // Text modifiers
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub strikethrough: bool,
    pub reversed: bool,

    // Widget state
    pub focusable: Option<bool>,
    pub disabled: bool,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub label: Option<String>,
    pub checked: Option<bool>,
    pub variant: Option<String>,
    /// Attributes the engine does not interpret itself. Widgets use these to
    /// carry their data (see [`crate::wire`]).
    pub custom: rustc_hash::FxHashMap<String, String>,
}
