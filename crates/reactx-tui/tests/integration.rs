use dioxus::prelude::*;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use reactx_core::{KeyModifiers, TuiEvent, TuiKey, TuiKeyEvent};
use reactx_tui::events::{handle_event, Flow};
use reactx_tui::TuiRenderer;

fn counter_app() -> Element {
    let mut count = use_signal(|| 0);
    let mut name = use_signal(String::new);
    rsx! {
        div { style: "direction:column;border:rounded;width:100%;height:100%;gap:1;padding:1",
            span { style: "color:#a6e3a1", "Count: {count}" }
            button { style: "border:rounded;padding:0 1", onclick: move |_| count += 1, "Increment" }
            input {
                style: "padding:0 1",
                value: "{name}",
                placeholder: "name",
                oninput: move |event| name.set(event.value()),
            }
        }
    }
}

fn setup() -> (VirtualDom, TuiRenderer, Terminal<TestBackend>) {
    let mut dom = VirtualDom::new(counter_app);
    let mut renderer = TuiRenderer::new();
    dom.rebuild(&mut renderer);
    let mut terminal = Terminal::new(TestBackend::new(40, 12)).unwrap();
    renderer.paint(&mut terminal).unwrap();
    (dom, renderer, terminal)
}

fn screen(terminal: &Terminal<TestBackend>) -> String {
    let buffer = terminal.backend().buffer();
    let mut out = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            out.push_str(buffer[(x, y)].symbol());
        }
        out.push('\n');
    }
    out
}

fn key(key: TuiKey) -> TuiEvent {
    TuiEvent::Key(TuiKeyEvent {
        key,
        modifiers: KeyModifiers::NONE,
    })
}

#[test]
fn renders_counter_and_reacts_to_click() {
    let (mut dom, mut renderer, mut terminal) = setup();
    let text = screen(&terminal);
    assert!(text.contains("Count: 0"), "{text}");
    assert!(text.contains("Increment"), "{text}");

    let button = renderer
        .tree
        .find_first_tag(renderer.tree.root_id(), "button")
        .expect("button node");
    let rect = renderer.layout[&button];
    let click = TuiEvent::MouseClick {
        x: rect.x + 1,
        y: rect.y,
    };
    assert_eq!(handle_event(&dom, &mut renderer, click), Flow::Continue);
    dom.render_immediate(&mut renderer);
    renderer.paint(&mut terminal).unwrap();

    let text = screen(&terminal);
    assert!(text.contains("Count: 1"), "{text}");
}

#[test]
fn keyboard_activation_and_typing() {
    let (mut dom, mut renderer, mut terminal) = setup();
    let button = renderer
        .tree
        .find_first_tag(renderer.tree.root_id(), "button")
        .expect("button node");

    renderer.focus.set_focus(Some(button));
    handle_event(&dom, &mut renderer, key(TuiKey::Enter));
    dom.render_immediate(&mut renderer);
    renderer.paint(&mut terminal).unwrap();
    assert!(screen(&terminal).contains("Count: 1"));

    // Tab to the input, then type.
    handle_event(&dom, &mut renderer, key(TuiKey::Tab));
    for ch in ['h', 'i'] {
        handle_event(&dom, &mut renderer, key(TuiKey::Char(ch)));
        dom.render_immediate(&mut renderer);
    }
    renderer.paint(&mut terminal).unwrap();
    let input = renderer
        .tree
        .find_first_tag(renderer.tree.root_id(), "input")
        .expect("input node");
    assert_eq!(
        renderer.tree.get(input).unwrap().attrs.value.as_deref(),
        Some("hi")
    );
    assert!(screen(&terminal).contains("hi"));

    handle_event(&dom, &mut renderer, key(TuiKey::Backspace));
    dom.render_immediate(&mut renderer);
    let input = renderer
        .tree
        .find_first_tag(renderer.tree.root_id(), "input")
        .unwrap();
    assert_eq!(
        renderer.tree.get(input).unwrap().attrs.value.as_deref(),
        Some("h")
    );
}
