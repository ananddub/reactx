use crate::harness::Harness;
use reactx::prelude::*;

fn hover_app() -> Element {
    rsx! {
        div { class: "p-1 hover:bg-red-500", "Hover me" }
    }
}

#[test]
fn hover_variant_covers_container_and_children() {
    let mut harness = Harness::new(hover_app);
    let node = harness.node_where(|node| node.tag == "box");
    assert_eq!(harness.attrs(node).bg, None);

    let rect = harness.rect(node);
    harness.mouse_move(rect.x + 1, rect.y);
    assert_eq!(
        harness.attrs(node).bg,
        Some(TuiColor::Rgb(0xef, 0x44, 0x44))
    );
}

fn focus_app() -> Element {
    rsx! {
        button { class: "focus:bg-blue-500", "Go" }
    }
}

#[test]
fn focus_variant_follows_the_focus_ring() {
    let harness = Harness::new(focus_app);
    let node = harness.node_where(|node| node.tag == "button");
    assert_eq!(
        harness.attrs(node).bg,
        Some(TuiColor::Rgb(0x3b, 0x82, 0xf6))
    );
}

fn disabled_app() -> Element {
    rsx! {
        button { disabled: true, class: "disabled:dim disabled:bg-red-500", "Off" }
    }
}

#[test]
fn disabled_variant_uses_the_disabled_attribute() {
    let harness = Harness::new(disabled_app);
    let node = harness.node_where(|node| node.tag == "button");
    assert!(harness.attrs(node).dim);
    assert_eq!(
        harness.attrs(node).bg,
        Some(TuiColor::Rgb(0xef, 0x44, 0x44))
    );
}

fn responsive_app() -> Element {
    rsx! {
        div { class: "p-1 md:p-4", "R" }
    }
}

#[test]
fn responsive_variants_follow_terminal_width() {
    let mut harness = Harness::new(responsive_app);
    let node = harness.node_where(|node| node.tag == "box");
    assert_eq!(harness.attrs(node).padding.top, 4.0);

    harness.resize(50, 16);
    assert_eq!(harness.attrs(node).padding.top, 1.0);
}

fn dynamic_app() -> Element {
    let mut hot = use_signal(|| false);
    rsx! {
        div {
            class: if hot() { "bg-red-500" } else { "bg-blue-500" },
            "state"
        }
        button { onclick: move |_| hot.set(!hot()), "Toggle" }
    }
}

#[test]
fn class_changes_swap_utilities_at_runtime() {
    let mut harness = Harness::new(dynamic_app);
    let node = harness.node_where(|node| node.tag == "box");
    assert_eq!(
        harness.attrs(node).bg,
        Some(TuiColor::Rgb(0x3b, 0x82, 0xf6))
    );

    harness.click_text("button", "Toggle");
    assert_eq!(
        harness.attrs(node).bg,
        Some(TuiColor::Rgb(0xef, 0x44, 0x44))
    );
}

fn list_app() -> Element {
    rsx! {
        div { class: "flex flex-col",
            span { class: "first:underline odd:bold", "one" }
            span { class: "even:dim", "two" }
            span { class: "last:italic", "three" }
        }
    }
}

#[test]
fn sibling_variants_apply_per_position() {
    let harness = Harness::new(list_app);
    let spans: Vec<_> = harness
        .renderer
        .tree
        .nodes_in_order()
        .into_iter()
        .filter(|id| {
            harness
                .renderer
                .tree
                .get(*id)
                .is_some_and(|n| n.tag == "text")
        })
        .collect();
    assert_eq!(spans.len(), 3);
    assert!(harness.attrs(spans[0]).underline);
    assert!(harness.attrs(spans[0]).bold);
    assert!(harness.attrs(spans[1]).dim);
    assert!(harness.attrs(spans[2]).italic);
    assert!(!harness.attrs(spans[1]).italic);
}

fn group_app() -> Element {
    rsx! {
        div { class: "group p-1",
            span { class: "group-hover:bg-red-500", "child" }
        }
    }
}

#[test]
fn group_hover_follows_the_ancestor() {
    let mut harness = Harness::new(group_app);
    let container = harness.node_where(|node| node.class.as_deref() == Some("group p-1"));
    let child = harness.node_where(|node| node.tag == "text");
    assert_eq!(harness.attrs(child).bg, None);

    let rect = harness.rect(container);
    harness.mouse_move(rect.x, rect.y);
    assert_eq!(
        harness.attrs(child).bg,
        Some(TuiColor::Rgb(0xef, 0x44, 0x44))
    );
}

fn checked_app() -> Element {
    rsx! {
        Checkbox {
            checked: true,
            class: "checked:bg-green-500",
            label: "on",
            on_change: move |_| {},
        }
    }
}

#[test]
fn checked_variant_reads_the_checkbox_state() {
    let harness = Harness::new(checked_app);
    let node = harness.node_where(|node| node.attrs.variant.as_deref() == Some("checkbox"));
    assert_eq!(
        harness.attrs(node).bg,
        Some(TuiColor::Rgb(0x22, 0xc5, 0x5e))
    );
}
