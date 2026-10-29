//! Tailwind-style utility classes for terminal styles.
//!
//! The `class` attribute accepts a whitespace-separated list of utilities,
//! mirroring the Tailwind vocabulary that makes sense in a terminal:
//!
//! - layout: `flex`, `flex-row`, `flex-col`, `flex-wrap`, `grow`, `basis-*`,
//!   `justify-*`, `items-*`, `self-*`, `place-*`, `content-*`, `order-*`, `z-*`
//! - box: `p-*`, `px-*`, `m-*`, `-mt-*`, `gap-*`, `space-x-*`, `w-*`, `h-*`,
//!   `size-*`, `min-*`, `max-*`, `inset-*`, `top-*` (plus logical `ps/pe/ms/me`)
//! - display/position: `hidden`, `relative`, `absolute`, `overflow-hidden`
//! - paint: `text-*`, `bg-*`, `border-*`, `border`, `rounded`, `bold`,
//!   `italic`, `underline`, `line-through`, `dim`, `reverse`
//! - animation: `animate-<effect>`, `animate-exit-*`, `duration-*`, `ease-*`,
//!   `delay-*`, `animate-repeat-*`, `animate-ping-pong`, `animate-index-*`,
//!   `animate-stagger-*`, `animate-direction-*`, `animate-color-*`, and the
//!   Tailwind presets `animate-spin`/`ping`/`pulse`/`bounce`
//!
//! Values follow Tailwind's spacing scale with one unit per terminal cell, so
//! `p-2` is two cells and `p-0.5` is half a cell. Arbitrary values in brackets
//! work too: `w-[20]`, `h-[50%]`, `text-[#f38ba8]`.
//!
//! Variants: `hover:`, `focus:`, `focus-visible:`, `focus-within:`,
//! `disabled:`, `checked:`, `first:`, `last:`, `odd:`, `even:`, `group-hover:`,
//! `group-focus:` and the responsive `sm:` (40 columns), `md:` (60), `lg:` (80),
//! `xl:` (100), `2xl:` (120). Variants stack (`md:hover:bg-red-500`).
//!
//! Precedence matches CSS: plain utilities first, matching variants next,
//! then the inline `style` attribute.

mod animate;
mod layout;
mod palette;
mod spacing;
mod utility;
mod value;
mod visual;

#[cfg(test)]
mod parity;
#[cfg(test)]
mod tests;

use super::StyleProps;

/// State used to resolve variant utilities (`hover:`, `focus:`, `md:`).
#[derive(Debug, Clone, Copy)]
pub struct ClassState {
    pub hovered: bool,
    pub focused: bool,
    pub focus_within: bool,
    pub disabled: bool,
    /// An ancestor of this node (or the node itself) is hovered.
    pub group_hovered: bool,
    /// The node's `checked` attribute is true.
    pub checked: bool,
    /// Zero-based position among siblings and the sibling count, used by
    /// `first:`, `last:`, `odd:` and `even:`.
    pub index: usize,
    pub siblings: usize,
    /// Terminal width in cells, used by responsive variants.
    pub width: u16,
}

impl Default for ClassState {
    fn default() -> Self {
        Self {
            hovered: false,
            focused: false,
            focus_within: false,
            disabled: false,
            group_hovered: false,
            checked: false,
            index: 0,
            siblings: 1,
            width: 0,
        }
    }
}

impl StyleProps {
    /// Apply a whitespace-separated Tailwind-style class list.
    pub fn apply_classes(&mut self, classes: &str, state: &ClassState) {
        let mut variants = Vec::new();
        for token in classes.split_whitespace() {
            let (prefixes, utility) = split_variants(token);
            if prefixes.is_empty() {
                utility::apply(self, utility);
            } else {
                variants.push((prefixes, utility));
            }
        }
        for (prefixes, utility) in variants {
            if prefixes.iter().all(|prefix| variant_matches(prefix, state)) {
                utility::apply(self, utility);
            }
        }
    }
}

/// Split `md:hover:p-2` into (`["md", "hover"]`, `"p-2"`). Colons inside
/// arbitrary `[...]` values are ignored.
fn split_variants(token: &str) -> (Vec<&str>, &str) {
    let mut depth = 0u32;
    let mut split = None;
    for (index, ch) in token.char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => depth = depth.saturating_sub(1),
            ':' if depth == 0 => split = Some(index),
            _ => {}
        }
    }
    match split {
        Some(index) => (token[..index].split(':').collect(), &token[index + 1..]),
        None => (Vec::new(), token),
    }
}

fn variant_matches(prefix: &str, state: &ClassState) -> bool {
    match prefix {
        "hover" => state.hovered,
        "focus" | "focus-visible" => state.focused,
        "focus-within" => state.focus_within,
        "disabled" => state.disabled,
        "group-hover" => state.group_hovered,
        "group-focus" => state.focus_within,
        "checked" => state.checked,
        "first" => state.index == 0,
        "last" => state.index + 1 >= state.siblings,
        "odd" => state.index % 2 == 0,
        "even" => state.index % 2 == 1,
        "sm" => state.width >= 40,
        "md" => state.width >= 60,
        "lg" => state.width >= 80,
        "xl" => state.width >= 100,
        "2xl" => state.width >= 120,
        _ => false,
    }
}
