use crate::harness::Harness;
use reactx::prelude::*;

fn focus_app() -> Element {
    let focused = use_terminal_focus();
    rsx! {
        span { "focused {focused}" }
    }
}

#[test]
fn terminal_focus_events_update_the_hook() {
    let mut harness = Harness::new(focus_app);
    assert!(harness.screen().contains("focused true"));

    harness.event(TuiEvent::FocusLost);
    assert!(harness.screen().contains("focused false"));

    harness.event(TuiEvent::FocusGained);
    assert!(harness.screen().contains("focused true"));
}
