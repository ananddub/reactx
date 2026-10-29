//! Colour themes and the theme context.

use dioxus::prelude::*;
use reactx_core::TuiColor;

const fn rgb(r: u8, g: u8, b: u8) -> TuiColor {
    TuiColor::Rgb(r, g, b)
}

/// A terminal colour theme. [`Theme::MOCHA`] is Catppuccin Mocha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub name: &'static str,
    pub base: TuiColor,
    pub mantle: TuiColor,
    pub crust: TuiColor,
    pub surface: TuiColor,
    pub border: TuiColor,
    pub overlay: TuiColor,
    pub text: TuiColor,
    pub subtext: TuiColor,
    pub accent: TuiColor,
    pub green: TuiColor,
    pub yellow: TuiColor,
    pub red: TuiColor,
    pub peach: TuiColor,
    pub mauve: TuiColor,
    pub teal: TuiColor,
    pub pink: TuiColor,
    pub selection: TuiColor,
    pub focus: TuiColor,
}

impl Theme {
    /// Catppuccin Mocha.
    pub const MOCHA: Theme = Theme {
        name: "catppuccin-mocha",
        base: rgb(0x1e, 0x1e, 0x2e),
        mantle: rgb(0x18, 0x18, 0x25),
        crust: rgb(0x11, 0x11, 0x1b),
        surface: rgb(0x31, 0x32, 0x44),
        border: rgb(0x58, 0x5b, 0x70),
        overlay: rgb(0x6c, 0x70, 0x86),
        text: rgb(0xcd, 0xd6, 0xf4),
        subtext: rgb(0xa6, 0xad, 0xc8),
        accent: rgb(0x89, 0xb4, 0xfa),
        green: rgb(0xa6, 0xe3, 0xa1),
        yellow: rgb(0xf9, 0xe2, 0xaf),
        red: rgb(0xf3, 0x8b, 0xa8),
        peach: rgb(0xfa, 0xb3, 0x87),
        mauve: rgb(0xcb, 0xa6, 0xf7),
        teal: rgb(0x94, 0xe2, 0xd5),
        pink: rgb(0xf5, 0xc2, 0xe7),
        selection: rgb(0x45, 0x47, 0x5a),
        focus: rgb(0x89, 0xb4, 0xfa),
    };

    pub fn mocha() -> Self {
        Self::MOCHA
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::MOCHA
    }
}

/// Read the nearest theme, falling back to Catppuccin Mocha.
pub fn use_theme() -> Theme {
    try_use_context::<Theme>().unwrap_or(Theme::MOCHA)
}

/// Provide a [`Theme`] to all descendants.
#[component]
pub fn ThemeProvider(#[props(default)] theme: Theme, children: Element) -> Element {
    use_context_provider(|| theme);
    rsx! { {children} }
}
