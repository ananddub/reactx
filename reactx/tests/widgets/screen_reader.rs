use crate::harness::Harness;
use reactx::prelude::*;
use reactx_core::screen;

fn labeled_app() -> Element {
    rsx! {
        ThemeProvider {
            Box { aria_label: "panel label", Text { "visible child" } }
            Text { aria_hidden: true, "hidden hint" }
        }
    }
}

#[test]
fn screen_reader_uses_labels_and_hides_marked_text() {
    screen::set_enabled(Some(true));
    let harness = Harness::new(labeled_app);
    let text = harness.screen();
    screen::set_enabled(None);
    assert!(text.contains("panel label"), "{text}");
    assert!(!text.contains("visible child"), "{text}");
    assert!(!text.contains("hidden hint"), "{text}");
}

#[test]
fn visual_mode_keeps_children_and_ignores_aria() {
    screen::set_enabled(Some(false));
    let harness = Harness::new(labeled_app);
    let text = harness.screen();
    screen::set_enabled(None);
    assert!(text.contains("visible child"), "{text}");
    assert!(text.contains("hidden hint"), "{text}");
    assert!(!text.contains("panel label"), "{text}");
}
