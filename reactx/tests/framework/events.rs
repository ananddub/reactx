//! Keyup, global keyboard fallback, mouse modifiers, context menu,
//! double-click and non-bubbling enter/leave events.

use crate::harness::Harness;
use reactx::prelude::*;

static KEY_LOG: Store<String> = Store::new("framework_events_keys", String::new);
static KEYUP_LOG: Store<String> = Store::new("framework_events_keyup", String::new);
static MOUSE_LOG: Store<String> = Store::new("framework_events_mouse2", String::new);

fn keys_app() -> Element {
    jsx! {
        <div
            style="direction:column"
            onkeydown={move |_| KEY_LOG.with_mut(|log| log.push_str("down;"))}
            onkeyup={move |_| KEY_LOG.with_mut(|log| log.push_str("up;"))}
        >
            <span>plain text</span>
        </div>
    }
}

fn keys_without_focus_app() -> Element {
    jsx! {
        <div onkeyup={move |_| KEYUP_LOG.with_mut(|log| log.push_str("up;"))}>
            nothing here is focusable
        </div>
    }
}

fn mouse_app() -> Element {
    jsx! {
        <div style="direction:column">
            <div
                style="height:2"
                onmousedown={move |event: Event<MouseData>| {
                    MOUSE_LOG.with_mut(|log| {
                        if event.modifiers().contains(Modifiers::CONTROL) {
                            log.push_str("ctrl-down;");
                        } else {
                            log.push_str("down;");
                        }
                    })
                }}
                oncontextmenu={move |_| MOUSE_LOG.with_mut(|log| log.push_str("menu;"))}
                ondoubleclick={move |_| MOUSE_LOG.with_mut(|log| log.push_str("double;"))}
                onmouseenter={move |_| MOUSE_LOG.with_mut(|log| log.push_str("enter;"))}
                onmouseleave={move |_| MOUSE_LOG.with_mut(|log| log.push_str("leave;"))}
                onmouseover={move |_| MOUSE_LOG.with_mut(|log| log.push_str("over;"))}
                onmouseout={move |_| MOUSE_LOG.with_mut(|log| log.push_str("out;"))}
            >alpha</div>
            <div style="height:2">beta</div>
        </div>
    }
}

fn text_position(harness: &Harness, text: &str) -> (u16, u16) {
    let node = harness.node_where(|node| node.tag == "#text" && node.text.as_deref() == Some(text));
    let rect = harness.renderer.layout[&node];
    (rect.x, rect.y)
}

fn mouse_log(harness: &Harness) -> String {
    harness.dom.in_runtime(|| MOUSE_LOG.peek())
}

#[test]
fn keydown_and_keyup_dispatch_to_focus() {
    let mut harness = Harness::new(keys_app);
    harness.dom.in_runtime(|| KEY_LOG.set(String::new()));

    harness.key(TuiKey::Char('a'));
    harness.key_up(TuiKey::Char('a'));

    assert_eq!(harness.dom.in_runtime(|| KEY_LOG.peek()), "down;up;");
}

#[test]
fn keyup_reaches_root_without_focus() {
    let mut harness = Harness::new(keys_without_focus_app);
    harness.dom.in_runtime(|| KEYUP_LOG.set(String::new()));

    harness.key_up(TuiKey::Escape);

    assert_eq!(harness.dom.in_runtime(|| KEYUP_LOG.peek()), "up;");
}

#[test]
fn right_click_raises_contextmenu() {
    let mut harness = Harness::new(mouse_app);
    harness.dom.in_runtime(|| MOUSE_LOG.set(String::new()));
    let (x, y) = text_position(&harness, "alpha");

    harness.mouse_down(x, y, MouseButtonKind::Right);

    assert_eq!(mouse_log(&harness), "down;menu;");
}

#[test]
fn ctrl_click_carries_modifiers() {
    let mut harness = Harness::new(mouse_app);
    harness.dom.in_runtime(|| MOUSE_LOG.set(String::new()));
    let (x, y) = text_position(&harness, "alpha");

    harness.mouse_down_mod(
        x,
        y,
        MouseButtonKind::Left,
        KeyModifiers {
            ctrl: true,
            ..KeyModifiers::NONE
        },
    );

    assert_eq!(mouse_log(&harness), "ctrl-down;");
}

#[test]
fn double_click_raises_doubleclick() {
    let mut harness = Harness::new(mouse_app);
    harness.dom.in_runtime(|| MOUSE_LOG.set(String::new()));
    let (x, y) = text_position(&harness, "alpha");

    harness.mouse_down(x, y, MouseButtonKind::Left);
    harness.mouse_down(x, y, MouseButtonKind::Left);

    assert_eq!(mouse_log(&harness), "down;down;double;");
}

#[test]
fn hover_raises_enter_and_leave() {
    let mut harness = Harness::new(mouse_app);
    harness.dom.in_runtime(|| MOUSE_LOG.set(String::new()));
    let (ax, ay) = text_position(&harness, "alpha");
    let (bx, by) = text_position(&harness, "beta");

    harness.mouse_move(ax, ay);
    assert_eq!(mouse_log(&harness), "over;enter;");

    harness.mouse_move(bx, by);
    assert_eq!(mouse_log(&harness), "over;enter;out;leave;");
}
