use crate::harness::Harness;
use reactx::prelude::*;
use reactx::TextAlign;

fn styling_app() -> Element {
    rsx! {
        div { class: "p-2 bg-blue-500 text-center", "Padded" }
    }
}

#[test]
fn box_utilities_apply_to_layout_and_paint() {
    let harness = Harness::new(styling_app);
    let node = harness.node_where(|node| node.tag == "box");
    let attrs = harness.attrs(node);
    assert_eq!(attrs.padding.top, 2.0);
    assert_eq!(attrs.bg, Some(TuiColor::Rgb(0x3b, 0x82, 0xf6)));
    assert_eq!(attrs.text_align, Some(TextAlign::Center));
    assert!(harness.screen().contains("Padded"));
}

fn row_app() -> Element {
    rsx! {
        div { class: "flex flex-row gap-2",
            Text { "aa" }
            Text { "bb" }
        }
    }
}

#[test]
fn row_and_gap_utilities_position_children() {
    let harness = Harness::new(row_app);
    let first = harness.node_where(|node| node.tag == "text" && harness.node_text(node.id) == "aa");
    let second =
        harness.node_where(|node| node.tag == "text" && harness.node_text(node.id) == "bb");
    let (first, second) = (harness.rect(first), harness.rect(second));
    assert_eq!(second.x, first.x + first.width + 2);
    assert_eq!(second.y, first.y);
}

fn sizing_app() -> Element {
    rsx! {
        div { class: "flex flex-row",
            div { class: "w-[8] h-[2] border border-red-500" }
            div { class: "hidden", "secret" }
            div { class: "w-1/2", "half" }
        }
    }
}

#[test]
fn arbitrary_sizes_fractions_and_hidden() {
    let harness = Harness::new(sizing_app);
    let fixed = harness
        .node_where(|node| node.class.as_deref() == Some("w-[8] h-[2] border border-red-500"));
    let rect = harness.rect(fixed);
    assert_eq!((rect.width, rect.height), (8, 2));
    assert_eq!(harness.attrs(fixed).border, Some(BorderKind::Single));
    assert_eq!(
        harness.attrs(fixed).border_color,
        Some(TuiColor::Rgb(0xef, 0x44, 0x44))
    );

    let half = harness.node_where(|node| node.class.as_deref() == Some("w-1/2"));
    assert_eq!(harness.rect(half).width, 30);

    assert!(!harness.screen().contains("secret"));
}

fn stack_app() -> Element {
    rsx! {
        div { class: "flex flex-col gap-1",
            Text { "one" }
            Text { "two" }
        }
    }
}

#[test]
fn column_gap_stacks_rows() {
    let harness = Harness::new(stack_app);
    let first =
        harness.node_where(|node| node.tag == "text" && harness.node_text(node.id) == "one");
    let second =
        harness.node_where(|node| node.tag == "text" && harness.node_text(node.id) == "two");
    let (first, second) = (harness.rect(first), harness.rect(second));
    assert_eq!(second.x, first.x);
    assert_eq!(second.y, first.y + first.height + 1);
}
