use crate::harness::Harness;
use reactx::prelude::*;

#[test]
fn checkbox_toggles_with_space() {
    fn app() -> Element {
        let mut checked = use_signal(|| false);
        rsx! {
            div { style: "direction:column",
                Checkbox {
                    checked: checked(),
                    label: "remember",
                    on_change: move |next| checked.set(next),
                }
                span { "checked: {checked()}" }
            }
        }
    }

    let mut harness = Harness::new(app);
    assert!(harness.screen().contains("[ ] remember"));

    let checkbox = harness.node_where(|node| node.attrs.variant.as_deref() == Some("checkbox"));
    harness.renderer.focus.set_focus(Some(checkbox));
    harness.key(TuiKey::Char(' '));
    assert!(harness.screen().contains("[x] remember"));
    assert!(harness.screen().contains("checked: true"));
}
