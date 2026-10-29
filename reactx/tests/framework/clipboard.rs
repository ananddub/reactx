use crate::harness::Harness;
use reactx::prelude::*;

static LOG: Store<String> = Store::new("framework_clipboard_log", String::new);
static PREVENTED: Store<u32> = Store::new("framework_clipboard_prevented", || 0);

fn clipboard_app() -> Element {
    let mut value = use_signal(|| "hello".to_string());
    rsx! {
        div { style: "direction:column",
            oncopy: move |_| LOG.with_mut(|log| log.push_str("copy;")),
            oncut: move |_| LOG.with_mut(|log| log.push_str("cut;")),
            onpaste: move |event| {
                let text = event
                    .data()
                    .downcast::<TuiClipboardEvent>()
                    .and_then(|data| data.text.clone())
                    .unwrap_or_default();
                LOG.with_mut(|log| log.push_str(&format!("paste:{text};")));
            },
            input {
                value: value(),
                oninput: move |event: FormEvent| value.set(event.value()),
            }
        }
    }
}

fn preventing_app() -> Element {
    rsx! {
        div {
            oncopy: move |event| {
                event.prevent_default();
                PREVENTED.with_mut(|count| *count += 1);
            },
            input { value: "abc" }
        }
    }
}

fn log(harness: &Harness) -> String {
    harness.dom.in_runtime(|| LOG.peek())
}

#[test]
fn clipboard_shortcuts_fire_events_before_the_default_action() {
    let mut harness = Harness::new(clipboard_app);
    harness.dom.in_runtime(|| LOG.set(String::new()));
    let input = harness.node_where(|node| node.tag == "input");
    harness.renderer.focus.set_focus(Some(input));

    harness.ctrl('a');
    harness.ctrl('c');
    assert_eq!(log(&harness), "copy;");
    assert_eq!(harness.renderer.clipboard.as_deref(), Some("hello"));

    harness.ctrl('x');
    assert_eq!(log(&harness), "copy;cut;");
    assert_eq!(harness.value(input), "");
    assert_eq!(harness.renderer.clipboard.as_deref(), Some("hello"));

    harness.event(TuiEvent::Paste("xy".into()));
    assert_eq!(log(&harness), "copy;cut;paste:xy;");
    assert_eq!(harness.value(input), "xy");
}

#[test]
fn prevent_default_stops_the_clipboard_write() {
    let mut harness = Harness::new(preventing_app);
    harness.dom.in_runtime(|| PREVENTED.set(0));
    let input = harness.node_where(|node| node.tag == "input");
    harness.renderer.focus.set_focus(Some(input));

    harness.ctrl('a');
    harness.ctrl('c');
    assert_eq!(harness.dom.in_runtime(|| PREVENTED.peek()), 1);
    assert_eq!(harness.renderer.clipboard, None);
    assert_eq!(harness.value(input), "abc");
}
