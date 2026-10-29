use crate::harness::Harness;
use ratatui::style::Modifier;
use reactx::prelude::*;

fn selection_app() -> Element {
    let mut value = use_signal(|| "hello world".to_string());
    rsx! {
        div { style: "direction:column",
            input {
                value: value(),
                oninput: move |event: FormEvent| value.set(event.value()),
            }
        }
    }
}

#[test]
fn selection_copies_cuts_and_replaces() {
    let mut harness = Harness::new(selection_app);
    let input = harness.node_where(|node| node.tag == "input");

    harness.key(TuiKey::Home);
    for _ in 0..5 {
        harness.shift(TuiKey::Right);
    }
    assert_eq!(harness.renderer.selection(input), Some((0, 5)));

    let rect = harness.renderer.layout[&input];
    let buffer = harness.terminal.backend().buffer();
    let highlighted = (1..6).any(|offset| {
        buffer[(rect.x + offset, rect.y + 1)]
            .modifier
            .contains(Modifier::REVERSED)
    });
    assert!(
        highlighted,
        "selected cells paint reversed:\n{}",
        harness.screen()
    );

    harness.ctrl('c');
    assert_eq!(harness.renderer.clipboard.as_deref(), Some("hello"));

    harness.type_text("HI");
    assert_eq!(harness.value(input), "HI world");
    assert_eq!(harness.renderer.input_cursor(input), Some(2));

    harness.ctrl('a');
    assert_eq!(harness.renderer.selection(input), Some((0, 8)));
    harness.ctrl('x');
    assert_eq!(harness.value(input), "");
    assert_eq!(harness.renderer.clipboard.as_deref(), Some("HI world"));

    let flow = handle_event(
        &harness.dom,
        &mut harness.renderer,
        TuiEvent::Key(TuiKeyEvent {
            key: TuiKey::Char('c'),
            modifiers: KeyModifiers {
                ctrl: true,
                ..KeyModifiers::NONE
            },
        }),
    );
    assert_eq!(flow, Flow::Quit, "Ctrl+C without a selection still quits");
}
