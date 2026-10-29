use crate::harness::Harness;
use reactx::prelude::*;

static INPUT_EVENTS: Store<u32> = Store::new("framework_input_events", || 0);

fn editor_app() -> Element {
    let mut value = use_signal(|| "abc".to_string());
    rsx! {
        div { style: "direction:column",
            input {
                value: value(),
                oninput: move |event: FormEvent| {
                    INPUT_EVENTS.with_mut(|count| *count += 1);
                    value.set(event.value());
                },
            }
        }
    }
}

#[test]
fn caret_editing_uses_arrow_home_end_and_paste() {
    let mut harness = Harness::new(editor_app);
    let input = harness.node_where(|node| node.tag == "input");
    harness.renderer.focus.set_focus(Some(input));

    harness.key(TuiKey::Home);
    harness.type_text("X");
    assert_eq!(harness.value(input), "Xabc");
    assert_eq!(harness.renderer.input_cursor(input), Some(1));

    harness.key(TuiKey::Left);
    assert_eq!(harness.renderer.input_cursor(input), Some(0));
    harness.key(TuiKey::Delete);
    assert_eq!(harness.value(input), "abc");

    harness.key(TuiKey::End);
    harness.key(TuiKey::Backspace);
    assert_eq!(harness.value(input), "ab");

    harness.event(TuiEvent::Paste("ZZ".into()));
    assert_eq!(harness.value(input), "abZZ");
    assert_eq!(harness.renderer.input_cursor(input), Some(4));

    // Only real value changes fire `input`; caret movement does not.
    let inputs = harness.dom.in_runtime(|| INPUT_EVENTS.with(|count| *count));
    assert_eq!(inputs, 4);
    assert!(
        harness.renderer.cursor.is_some(),
        "the terminal cursor follows the focused input"
    );
}
