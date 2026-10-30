use crate::harness::Harness;
use reactx::prelude::*;

fn ratio_app() -> Element {
    rsx! {
        ThemeProvider {
            Box { width: "10", aspect_ratio: Some(2.0), border: "single" }
        }
    }
}

#[test]
fn aspect_ratio_sets_the_box_height() {
    let harness = Harness::new(ratio_app);
    let node = harness.node_where(|node| node.attrs.aspect_ratio == Some(2.0));
    let rect = harness.renderer.layout[&node];
    assert_eq!((rect.width, rect.height), (10, 5));
}

fn border_app() -> Element {
    rsx! {
        ThemeProvider {
            Box { width: "12", height: "4", border: "doubleSingle" }
        }
    }
}

#[test]
fn double_single_border_uses_mixed_glyphs() {
    let harness = Harness::new(border_app);
    let screen = harness.screen();
    assert!(screen.contains('╒') && screen.contains('╕'), "{screen}");
    assert!(screen.contains('═'), "{screen}");
}

fn styled_app() -> Element {
    rsx! {
        ThemeProvider {
            Text { bg: "#101010", inverse: true, "styled" }
        }
    }
}

#[test]
fn text_background_and_inverse_map_to_styles() {
    let harness = Harness::new(styled_app);
    let id = harness.node_where(|node| node.attrs.reversed);
    let node = harness.renderer.tree.get(id).unwrap();
    assert!(node.attrs.bg.is_some());
}
