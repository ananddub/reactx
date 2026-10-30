//! The `Animate` wrapper encodes animation attributes and reruns on trigger.

use crate::harness::Harness;
use reactx::prelude::*;

fn wrapped_app() -> Element {
    let mut trigger = use_signal(|| 0u64);
    rsx! {
        Box { direction: "column", gap: Some(1),
            Animate {
                effect: "dissolve",
                duration: 90,
                easing: "linear",
                trigger: trigger(),
                Text { "wrapped content" }
            }
            Button { on_click: move |_| trigger += 1, "again" }
        }
    }
}

fn attrs(harness: &Harness) -> (String, String, String) {
    let node = harness.node_where(|node| node.attrs.custom.contains_key("animate"));
    let custom = &harness.renderer.tree.get(node).unwrap().attrs.custom;
    (
        custom.get("animate").cloned().unwrap_or_default(),
        custom.get("animate_duration").cloned().unwrap_or_default(),
        custom.get("animate_key").cloned().unwrap_or_default(),
    )
}

#[test]
fn animate_wrapper_encodes_attrs_and_retriggers() {
    let mut harness = Harness::new(wrapped_app);
    assert!(harness.screen().contains("wrapped content"));
    assert_eq!(
        attrs(&harness),
        ("dissolve".into(), "90".into(), "0".into())
    );
    assert!(harness.renderer.effects_active(), "mount animation missing");

    harness.click_text("button", "again");
    assert_eq!(attrs(&harness).2, "1", "trigger did not reach animate-key");
    assert!(harness.renderer.effects_active(), "trigger did not replay");
}

fn decorated_app() -> Element {
    rsx! {
        Animate {
            effect: "dissolve",
            duration: 90,
            easing: "linear",
            exit: "slide-out",
            repeat: "3",
            ping_pong: true,
            delay: 20,
            index: 2,
            step: 150,
            Text { "wrapped content" }
        }
    }
}

#[test]
fn animate_wrapper_encodes_composition_props() {
    let harness = Harness::new(decorated_app);
    let node = harness.node_where(|node| node.attrs.custom.contains_key("animate"));
    let custom = &harness.renderer.tree.get(node).unwrap().attrs.custom;
    for (key, value) in [
        ("animate", "dissolve"),
        ("animate_duration", "90"),
        ("animate_easing", "linear"),
        ("animate_exit", "slide-out"),
        ("animate_repeat", "3"),
        ("animate_ping_pong", "1"),
        ("animate_delay", "20"),
        ("animate_index", "2"),
        ("animate_stagger", "150"),
    ] {
        assert_eq!(custom.get(key).map(String::as_str), Some(value), "{key}");
    }
}

fn class_app() -> Element {
    rsx! {
        div { class: "animate-fade duration-60 ease-linear animate-color-red", "tailwind" }
    }
}

#[test]
fn animate_classes_feed_the_node_effects() {
    let harness = Harness::new(class_app);
    let node = harness.node_where(|node| node.attrs.custom.contains_key("animate"));
    let custom = &harness.renderer.tree.get(node).unwrap().attrs.custom;
    for (key, value) in [
        ("animate", "fade"),
        ("animate_duration", "60"),
        ("animate_easing", "linear"),
        ("animate_color", "red"),
    ] {
        assert_eq!(custom.get(key).map(String::as_str), Some(value), "{key}");
    }
    assert!(
        harness.renderer.effects_active(),
        "class-driven animation did not start"
    );
}
