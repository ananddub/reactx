//! Colour values and terminal palette names.

mod names;
mod parse;

#[cfg(test)]
mod tests;

/// A colour parsed from an RSX attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiColor {
    Reset,
    Named(NamedColor),
    Rgb(u8, u8, u8),
    Indexed(u8),
}

/// The ANSI-16 palette plus a few well-known extended names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    BrightWhite,
    Orange,
    Pink,
    Purple,
    Teal,
}
