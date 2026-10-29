use crate::harness::Harness;
use reactx::prelude::*;

static TEXTAREA_SUBMITS: Store<u32> = Store::new("framework_textarea_submits", || 0);

fn textarea_app() -> Element {
    let mut value = use_signal(|| "one".to_string());
    rsx! {
        div { style: "direction:column;width:20",
            TextArea {
                value: value(),
                height: Some(4),
                on_input: move |text: String| value.set(text),
                on_submit: move |_| TEXTAREA_SUBMITS.with_mut(|count| *count += 1),
            }
        }
    }
}

#[test]
fn textarea_edits_multiple_lines_and_submits() {
    let mut harness = Harness::new(textarea_app);
    harness.dom.in_runtime(|| TEXTAREA_SUBMITS.set(0));
    let textarea = harness.node_where(|node| node.tag == "textarea");

    harness.key(TuiKey::End);
    harness.type_text("x");
    harness.key(TuiKey::Enter);
    harness.type_text("two");
    assert_eq!(harness.value(textarea), "onex\ntwo");

    harness.key(TuiKey::Up);
    harness.type_text("!");
    assert_eq!(harness.value(textarea), "one!x\ntwo");

    let screen = harness.screen();
    assert!(screen.contains("one!x"), "{screen}");
    assert!(screen.contains("two"), "{screen}");
    assert!(harness.renderer.cursor.is_some());

    harness.shift(TuiKey::End);
    assert_eq!(
        harness.renderer.selection(textarea),
        Some((4, 5)),
        "Shift+End selects to the end of the current line"
    );

    harness.key_mod(
        TuiKey::Enter,
        KeyModifiers {
            ctrl: true,
            ..KeyModifiers::NONE
        },
    );
    assert_eq!(
        harness
            .dom
            .in_runtime(|| TEXTAREA_SUBMITS.with(|count| *count)),
        1,
        "Ctrl+Enter submits"
    );
    assert_eq!(
        harness.value(textarea),
        "one!x\ntwo",
        "Ctrl+Enter must not insert a newline"
    );
}
