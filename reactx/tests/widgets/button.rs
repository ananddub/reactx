use crate::harness::Harness;
use reactx::prelude::*;

fn counter_app() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        ThemeProvider {
            Container { border: "rounded", padding: "1", gap: Some(1),
                Text { "Count: {count}" }
                Button { on_click: move |_| count += 1, "Increment" }
            }
        }
    }
}

#[test]
fn button_click_and_keyboard_activation() {
    let mut harness = Harness::new(counter_app);
    assert!(harness.screen().contains("Count: 0"));

    harness.click_text("button", "Increment");
    assert!(harness.screen().contains("Count: 1"));

    let button = harness.node_where(|node| node.tag == "button");
    harness.renderer.focus.set_focus(Some(button));
    harness.key(TuiKey::Enter);
    assert!(harness.screen().contains("Count: 2"));
}
