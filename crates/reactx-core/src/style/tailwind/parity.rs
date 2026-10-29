//! Tailwind parity tests for the newer utilities and variants.

use crate::color::TuiColor;
use crate::style::{Align, ClassState, Justify, StyleProps};

fn styled(classes: &str) -> StyleProps {
    styled_at(classes, ClassState::default())
}

fn styled_at(classes: &str, state: ClassState) -> StyleProps {
    let mut props = StyleProps::default();
    props.apply_classes(classes, &state);
    props
}

#[test]
fn place_aliases_cover_items_content_and_self() {
    let props = styled("place-items-center place-content-between place-self-end");
    assert_eq!(props.align, Some(Align::Center));
    assert_eq!(props.align_content, Some(Justify::SpaceBetween));
    assert_eq!(props.align_self, Some(Align::End));
}

#[test]
fn grow_and_shrink_take_numbers() {
    let props = styled("grow-2 shrink-[0.5]");
    assert_eq!(props.flex_grow, Some(2.0));
    assert_eq!(props.flex_shrink, Some(0.5));

    let props = styled("flex-grow-3 flex-shrink-0");
    assert_eq!(props.flex_grow, Some(3.0));
    assert_eq!(props.flex_shrink, Some(0.0));
}

#[test]
fn arbitrary_layer_and_logical_spacing() {
    let props = styled("z-[5] ps-2 pe-3 ms-1 me-4");
    assert_eq!(props.layer, Some(5));
    assert_eq!(props.padding.left, 2.0);
    assert_eq!(props.padding.right, 3.0);
    assert_eq!(props.margin.left, 1.0);
    assert_eq!(props.margin.right, 4.0);

    let props = styled("-ms-2 -me-1");
    assert_eq!(props.margin.left, -2.0);
    assert_eq!(props.margin.right, -1.0);
}

#[test]
fn structural_variants_follow_sibling_position() {
    let classes = "first:underline last:italic odd:bold even:dim";
    let first = styled_at(
        classes,
        ClassState {
            index: 0,
            siblings: 4,
            ..ClassState::default()
        },
    );
    assert!(first.underline && first.bold);
    assert!(!first.italic && !first.dim);

    let last = styled_at(
        classes,
        ClassState {
            index: 3,
            siblings: 4,
            ..ClassState::default()
        },
    );
    assert!(last.italic && last.dim);
    assert!(!last.underline && !last.bold);
}

#[test]
fn checked_and_group_variants() {
    let state = ClassState {
        checked: true,
        group_hovered: true,
        focus_within: true,
        ..ClassState::default()
    };
    let props = styled_at(
        "checked:bg-green-500 group-hover:underline group-focus:bold",
        state,
    );
    assert_eq!(props.bg, Some(TuiColor::Rgb(0x22, 0xc5, 0x5e)));
    assert!(props.underline);
    assert!(props.bold);
}
