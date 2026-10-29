use crate::harness::Harness;
use reactx::prelude::*;

#[test]
fn input_typing_and_enter_submit() {
    static SUBMITTED: Store<Vec<String>> = Store::new("input_submitted", Vec::new);

    fn app() -> Element {
        let mut value = use_signal(String::new);
        let submitted = SUBMITTED.with(|entries| entries.len());
        let last = SUBMITTED.with(|entries| entries.last().cloned().unwrap_or_default());
        rsx! {
            div { style: "direction:column",
                Input {
                    value: value(),
                    placeholder: "name",
                    on_input: move |next| value.set(next),
                    on_submit: move |next| SUBMITTED.with_mut(|entries| entries.push(next)),
                }
                span { "submitted {submitted}" }
                span { "last {last}" }
            }
        }
    }

    let mut harness = Harness::new(app);
    let input = harness.node_where(|node| node.tag == "input");
    harness.renderer.focus.set_focus(Some(input));
    harness.type_text("hi");
    assert_eq!(
        harness
            .renderer
            .tree
            .get(input)
            .unwrap()
            .attrs
            .value
            .as_deref(),
        Some("hi")
    );
    assert!(harness.screen().contains("submitted 0"));

    harness.key(TuiKey::Enter);
    assert!(harness.screen().contains("submitted 1"));
    assert!(harness.screen().contains("last hi"));

    harness.key(TuiKey::Backspace);
    assert_eq!(
        harness
            .renderer
            .tree
            .get(input)
            .unwrap()
            .attrs
            .value
            .as_deref(),
        Some("h")
    );
}
