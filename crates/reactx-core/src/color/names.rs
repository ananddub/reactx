//! `NamedColor` parsing and Ratatui conversion.

use super::*;
use ratatui::style::Color as RColor;

impl NamedColor {
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim().to_ascii_lowercase();
        Some(match s.as_str() {
            "black" => Self::Black,
            "red" => Self::Red,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "blue" => Self::Blue,
            "magenta" | "fuchsia" => Self::Magenta,
            "cyan" | "aqua" => Self::Cyan,
            "white" => Self::White,
            "gray" | "grey" | "silver" | "light-gray" | "light-grey" | "dark-white" => Self::Gray,
            "dark-gray" | "dark-grey" | "dim-gray" | "dim-grey" => Self::DarkGray,
            "bright-red" | "light-red" => Self::LightRed,
            "bright-green" | "light-green" => Self::LightGreen,
            "bright-yellow" | "light-yellow" => Self::LightYellow,
            "bright-blue" | "light-blue" => Self::LightBlue,
            "bright-magenta" | "light-magenta" | "bright-purple" => Self::LightMagenta,
            "bright-cyan" | "light-cyan" => Self::LightCyan,
            "bright-white" => Self::BrightWhite,
            "orange" => Self::Orange,
            "pink" => Self::Pink,
            "purple" | "violet" => Self::Purple,
            "teal" => Self::Teal,
            _ => return None,
        })
    }

    /// A canonical name [`NamedColor::parse`] round-trips.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Black => "black",
            Self::Red => "red",
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Blue => "blue",
            Self::Magenta => "magenta",
            Self::Cyan => "cyan",
            Self::White => "white",
            Self::Gray => "gray",
            Self::DarkGray => "dark-gray",
            Self::LightRed => "bright-red",
            Self::LightGreen => "bright-green",
            Self::LightYellow => "bright-yellow",
            Self::LightBlue => "bright-blue",
            Self::LightMagenta => "bright-magenta",
            Self::LightCyan => "bright-cyan",
            Self::BrightWhite => "bright-white",
            Self::Orange => "orange",
            Self::Pink => "pink",
            Self::Purple => "purple",
            Self::Teal => "teal",
        }
    }

    pub fn to_ratatui(self) -> RColor {
        match self {
            Self::Black => RColor::Black,
            Self::Red => RColor::Red,
            Self::Green => RColor::Green,
            Self::Yellow => RColor::Yellow,
            Self::Blue => RColor::Blue,
            Self::Magenta => RColor::Magenta,
            Self::Cyan => RColor::Cyan,
            Self::White => RColor::White,
            Self::Gray => RColor::Gray,
            Self::DarkGray => RColor::DarkGray,
            Self::LightRed => RColor::LightRed,
            Self::LightGreen => RColor::LightGreen,
            Self::LightYellow => RColor::LightYellow,
            Self::LightBlue => RColor::LightBlue,
            Self::LightMagenta => RColor::LightMagenta,
            Self::LightCyan => RColor::LightCyan,
            Self::BrightWhite => RColor::White,
            Self::Orange => RColor::Rgb(255, 165, 0),
            Self::Pink => RColor::Rgb(255, 192, 203),
            Self::Purple => RColor::Rgb(128, 0, 128),
            Self::Teal => RColor::Rgb(0, 128, 128),
        }
    }
}
