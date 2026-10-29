#![cfg(test)]

use super::*;

#[test]
fn parses_layout_attrs() {
    let mut p = StyleProps::default();
    p.apply_attr("direction", "row");
    p.apply_attr("justify", "space-between");
    p.apply_attr("width", "50%");
    p.apply_attr("height", "10");
    p.apply_attr("padding", "1");
    p.apply_attr("margin_top", "2");
    p.apply_attr("gap", "1");

    assert_eq!(p.direction, Some(FlexDir::Row));
    assert_eq!(p.justify, Some(Justify::SpaceBetween));
    assert_eq!(p.width, Some(Dim::Pct(50.0)));
    assert_eq!(p.height, Some(Dim::Cells(10.0)));
    assert_eq!(p.padding, Edges::uniform(1.0));
    assert_eq!(p.margin.top, 2.0);
    assert_eq!(p.gap, Some((1.0, 1.0)));
}

#[test]
fn parses_visual_attrs() {
    let mut p = StyleProps::default();
    p.apply_attr("border", "rounded");
    p.apply_attr("border-color", "#89b4fa");
    p.apply_attr("bg", "#1e1e2e");
    p.apply_attr("bold", "true");
    p.apply_attr("title", "Hello");

    assert_eq!(p.border, Some(BorderKind::Rounded));
    assert_eq!(p.border_color, Some(TuiColor::Rgb(137, 180, 250)));
    assert!(p.bold);
    assert_eq!(p.title.as_deref(), Some("Hello"));

    p.clear_attr("bold");
    p.clear_attr("border");
    assert!(!p.bold);
    assert_eq!(p.border, None);
}

#[test]
fn custom_attrs_are_preserved_and_cleared() {
    let mut p = StyleProps::default();
    p.apply_attr("table-rows", "a\u{1f}b");
    p.apply_attr("gauge-ratio", "0.75");

    assert_eq!(
        p.custom.get("table_rows").map(String::as_str),
        Some("a\u{1f}b")
    );
    assert_eq!(
        p.custom.get("gauge_ratio").map(String::as_str),
        Some("0.75")
    );

    p.clear_attr("table-rows");
    assert!(!p.custom.contains_key("table_rows"));
}

#[test]
fn text_attrs_keep_whitespace() {
    let mut p = StyleProps::default();
    p.apply_attr("value", "two words ");
    assert_eq!(p.value.as_deref(), Some("two words "));
}

#[test]
fn parses_scroll_layer_and_trap_attrs() {
    let mut p = StyleProps::default();
    p.apply_attr("scroll-y", "3");
    p.apply_attr("scroll-x", "-2");
    p.apply_attr("z-index", "10");
    p.apply_attr("focus-trap", "true");
    p.apply_attr("autofocus", "on");

    assert_eq!(p.scroll_y, 3.0);
    assert_eq!(p.scroll_x, 0.0);
    assert_eq!(p.layer, Some(10));
    assert!(p.focus_trap);
    assert!(p.autofocus);
}

#[test]
fn parses_edges() {
    assert_eq!(Edges::parse("1"), Some(Edges::uniform(1.0)));
    assert_eq!(
        Edges::parse("1, 2"),
        Some(Edges {
            top: 1.0,
            bottom: 1.0,
            right: 2.0,
            left: 2.0
        })
    );
    assert_eq!(
        Edges::parse("1 2 3 4"),
        Some(Edges {
            top: 1.0,
            right: 2.0,
            bottom: 3.0,
            left: 4.0
        })
    );
}
