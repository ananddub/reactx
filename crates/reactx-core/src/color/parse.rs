//! `TuiColor` parsing and conversions.

use super::*;
use ratatui::style::Color as RColor;

impl TuiColor {
    /// Parse `#RGB`, `#RRGGBB`, `rgb(r, g, b)`, `ansi(n)` or a colour name.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }

        if let Some(hex) = s.strip_prefix('#') {
            let hex = hex.trim();
            return match hex.len() {
                3 => {
                    let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                    let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                    let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                    Some(Self::Rgb(r * 17, g * 17, b * 17))
                }
                6 | 8 => {
                    // Ignore an optional alpha channel.
                    let n = u32::from_str_radix(&hex[0..6], 16).ok()?;
                    Some(Self::Rgb(
                        ((n >> 16) & 0xff) as u8,
                        ((n >> 8) & 0xff) as u8,
                        (n & 0xff) as u8,
                    ))
                }
                _ => None,
            };
        }

        if let Some(inner) = s
            .strip_prefix("rgb(")
            .or_else(|| s.strip_prefix("RGB("))
            .and_then(|v| v.strip_suffix(')'))
        {
            let parts: Vec<u8> = inner
                .split(',')
                .filter_map(|part| part.trim().parse::<u8>().ok())
                .collect();
            if parts.len() == 3 {
                return Some(Self::Rgb(parts[0], parts[1], parts[2]));
            }
            return None;
        }

        if let Some(inner) = s.strip_prefix("ansi(").and_then(|v| v.strip_suffix(')')) {
            return inner.trim().parse::<u8>().ok().map(Self::Indexed);
        }

        if s.eq_ignore_ascii_case("reset") || s.eq_ignore_ascii_case("default") {
            return Some(Self::Reset);
        }

        NamedColor::parse(s).map(Self::Named)
    }

    pub fn to_ratatui(self) -> RColor {
        match self {
            Self::Reset => RColor::Reset,
            Self::Named(named) => named.to_ratatui(),
            Self::Rgb(r, g, b) => RColor::Rgb(r, g, b),
            Self::Indexed(i) => RColor::Indexed(i),
        }
    }

    /// Render the colour in a form [`TuiColor::parse`] accepts, so it can be
    /// embedded in an RSX `style` string.
    pub fn to_css(self) -> String {
        match self {
            Self::Reset => "reset".to_string(),
            Self::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
            Self::Indexed(i) => format!("ansi({i})"),
            Self::Named(named) => named.as_str().to_string(),
        }
    }
}
