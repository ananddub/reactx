#![cfg(test)]

use super::*;

#[test]
fn parses_hex_rgb_and_names() {
    assert_eq!(
        TuiColor::parse("#89b4fa"),
        Some(TuiColor::Rgb(137, 180, 250))
    );
    assert_eq!(TuiColor::parse("#f00"), Some(TuiColor::Rgb(255, 0, 0)));
    assert_eq!(
        TuiColor::parse("rgb(166, 227, 161)"),
        Some(TuiColor::Rgb(166, 227, 161))
    );
    assert_eq!(
        TuiColor::parse("bright-green"),
        Some(TuiColor::Named(NamedColor::LightGreen))
    );
    assert_eq!(TuiColor::parse("ansi(42)"), Some(TuiColor::Indexed(42)));
    assert_eq!(TuiColor::parse("reset"), Some(TuiColor::Reset));
    assert_eq!(TuiColor::parse("not-a-color"), None);
}
