//! Unit tests for the utility-class parser.

use crate::color::TuiColor;
use crate::style::{
    Align, BorderKind, ClassState, Dim, Display, FlexDir, FlexWrap, Justify, Overflow, Position,
    StyleProps, TextAlign,
};

fn styled(classes: &str) -> StyleProps {
    styled_at(classes, 120)
}

fn styled_at(classes: &str, width: u16) -> StyleProps {
    let mut props = StyleProps::default();
    let state = ClassState {
        width,
        ..ClassState::default()
    };
    props.apply_classes(classes, &state);
    props
}

#[test]
fn layout_flex_direction_and_alignment() {
    let props = styled("flex flex-row items-center justify-between");
    assert_eq!(props.direction, Some(FlexDir::Row));
    assert_eq!(props.align, Some(Align::Center));
    assert_eq!(props.justify, Some(Justify::SpaceBetween));
}

#[test]
fn layout_wrap_grow_and_basis() {
    let props = styled("flex flex-wrap grow basis-1/2");
    assert_eq!(props.flex_wrap, Some(FlexWrap::Wrap));
    assert_eq!(props.flex_grow, Some(1.0));
    assert_eq!(props.flex_basis, Some(Dim::Pct(50.0)));
}

#[test]
fn layout_self_content_order_and_layer() {
    let props = styled("self-end content-center order-2 z-10");
    assert_eq!(props.align_self, Some(Align::End));
    assert_eq!(props.align_content, Some(Justify::Center));
    assert_eq!(props.order, Some(2));
    assert_eq!(props.layer, Some(10));
}

#[test]
fn display_and_position() {
    assert_eq!(styled("hidden").display, Some(Display::None));
    assert_eq!(styled("absolute").position, Some(Position::Absolute));
    assert_eq!(styled("truncate").overflow, Some(Overflow::Hidden));
}

#[test]
fn spacing_edges_compose_in_order() {
    let props = styled("p-2 px-4 pt-1");
    assert_eq!(props.padding.top, 1.0);
    assert_eq!(props.padding.right, 4.0);
    assert_eq!(props.padding.bottom, 2.0);
    assert_eq!(props.padding.left, 4.0);
}

#[test]
fn negative_margin_and_inset() {
    let props = styled("-mt-2 -mx-1 inset-x-2");
    assert_eq!(props.margin.top, -2.0);
    assert_eq!(props.margin.left, -1.0);
    assert_eq!(props.margin.right, -1.0);
    assert_eq!(props.left, Some(Dim::Cells(2.0)));
    assert_eq!(props.right, Some(Dim::Cells(2.0)));
}

#[test]
fn negative_padding_is_ignored() {
    assert!(styled("-p-2").padding.is_zero());
}

#[test]
fn sizes_fractions_and_arbitrary_dimensions() {
    let half = styled("w-1/2");
    assert_eq!(half.width, Some(Dim::Pct(50.0)));

    let arbitrary = styled("h-[20]");
    assert_eq!(arbitrary.height, Some(Dim::Cells(20.0)));

    let sized = styled("size-3");
    assert_eq!(sized.width, Some(Dim::Cells(3.0)));
    assert_eq!(sized.height, Some(Dim::Cells(3.0)));

    assert_eq!(styled("max-w-full").max_width, Some(Dim::Pct(100.0)));
}

#[test]
fn gap_utilities() {
    let props = styled("gap-2 gap-x-3");
    assert_eq!(props.gap, Some((2.0, 3.0)));

    let props = styled("space-y-1");
    assert_eq!(props.gap, Some((1.0, 0.0)));
}

#[test]
fn palette_colors() {
    let props = styled("bg-blue-500 text-red-500 border border-blue-100");
    assert_eq!(props.bg, Some(TuiColor::Rgb(0x3b, 0x82, 0xf6)));
    assert_eq!(props.color, Some(TuiColor::Rgb(0xef, 0x44, 0x44)));
    assert_eq!(props.border, Some(BorderKind::Single));
    assert_eq!(props.border_color, Some(TuiColor::Rgb(0xdb, 0xea, 0xfe)));
}

#[test]
fn arbitrary_colors_and_dimensions() {
    let props = styled("text-[#f38ba8] w-[20]");
    assert_eq!(props.color, Some(TuiColor::Rgb(0xf3, 0x8b, 0xa8)));
    assert_eq!(props.width, Some(Dim::Cells(20.0)));
}

#[test]
fn text_alignment_and_modifiers() {
    let props = styled("text-center bold italic underline line-through dim reverse");
    assert_eq!(props.text_align, Some(TextAlign::Center));
    assert!(props.bold);
    assert!(props.italic);
    assert!(props.underline);
    assert!(props.strikethrough);
    assert!(props.dim);
    assert!(props.reversed);
}

#[test]
fn border_styles() {
    assert_eq!(styled("rounded").border, Some(BorderKind::Rounded));
    assert_eq!(
        styled("border rounded-none").border,
        Some(BorderKind::Single)
    );
    assert_eq!(styled("border-none").border, Some(BorderKind::None));
}

#[test]
fn hover_variant_applies_only_when_hovered() {
    let base = ClassState {
        width: 120,
        ..ClassState::default()
    };
    let mut props = StyleProps::default();
    props.apply_classes("p-2 hover:p-4", &base);
    assert_eq!(props.padding.top, 2.0);

    let hovered = ClassState {
        hovered: true,
        ..base
    };
    props.apply_classes("p-2 hover:p-4", &hovered);
    assert_eq!(props.padding.top, 4.0);
}

#[test]
fn responsive_variants_follow_width() {
    assert_eq!(styled_at("p-2 md:p-6", 50).padding.top, 2.0);
    assert_eq!(styled_at("p-2 md:p-6", 60).padding.top, 6.0);
}

#[test]
fn stacked_variants_require_all_prefixes() {
    let state = ClassState {
        hovered: true,
        width: 80,
        ..ClassState::default()
    };
    let mut props = StyleProps::default();
    props.apply_classes("bg-blue-500 md:hover:bg-red-500", &state);
    assert_eq!(props.bg, Some(TuiColor::Rgb(0xef, 0x44, 0x44)));
}

#[test]
fn disabled_and_focus_within_variants() {
    let state = ClassState {
        disabled: true,
        focus_within: true,
        width: 120,
        ..ClassState::default()
    };
    let mut props = StyleProps::default();
    props.apply_classes("disabled:dim focus-within:underline", &state);
    assert!(props.dim);
    assert!(props.underline);
}

#[test]
fn inline_style_wins_over_classes() {
    let mut props = styled("p-2 text-center");
    props.apply_style_string("padding:1;text_align:right");
    assert_eq!(props.padding.top, 1.0);
    assert_eq!(props.text_align, Some(TextAlign::Right));
}
