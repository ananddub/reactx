use crate::harness::Harness;
use reactx::prelude::*;

static CHANGE_EVENTS: Store<u32> = Store::new("framework_change_events", || 0);

fn change_app() -> Element {
    let mut value = use_signal(|| "abc".to_string());
    rsx! {
        div { style: "direction:column",
            input {
                value: value(),
                oninput: move |event: FormEvent| value.set(event.value()),
                onchange: move |_| CHANGE_EVENTS.with_mut(|count| *count += 1),
            }
            button { "other" }
        }
    }
}

#[test]
fn change_fires_once_on_blur_after_an_edit() {
    let mut harness = Harness::new(change_app);
    harness.dom.in_runtime(|| CHANGE_EVENTS.set(0));

    harness.type_text("d");
    harness.key(TuiKey::Tab);
    assert_eq!(
        harness
            .dom
            .in_runtime(|| CHANGE_EVENTS.with(|count| *count)),
        1,
        "leaving an edited input raises change"
    );

    harness.shift(TuiKey::Tab);
    harness.key(TuiKey::Tab);
    assert_eq!(
        harness
            .dom
            .in_runtime(|| CHANGE_EVENTS.with(|count| *count)),
        1,
        "leaving without an edit must not raise change"
    );

    harness.shift(TuiKey::Tab);
    harness.type_text("e");
    harness.key(TuiKey::Tab);
    assert_eq!(
        harness
            .dom
            .in_runtime(|| CHANGE_EVENTS.with(|count| *count)),
        2
    );
}
