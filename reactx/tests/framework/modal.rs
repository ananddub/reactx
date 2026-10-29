use crate::harness::Harness;
use reactx::prelude::*;

static BG_CLICKS: Store<u32> = Store::new("framework_bg_clicks", || 0);

fn modal_app() -> Element {
    let mut open = use_signal(|| true);
    rsx! {
        ThemeProvider {
            div { style: "direction:column;height:100%",
                button {
                    onclick: move |_| {
                        BG_CLICKS.with_mut(|count| *count += 1);
                        open.set(true);
                    },
                    "background action"
                }
                span { "status {open()}" }
                Modal {
                    open: open(),
                    title: "Confirm",
                    on_close: move |_| open.set(false),
                    Text { "delete file?" }
                    div { style: "direction:row;gap:2;justify:end",
                        Button { on_click: move |_| open.set(false), "Accept" }
                        Button { variant: "ghost", on_click: move |_| open.set(false), "Reject" }
                    }
                }
            }
        }
    }
}

fn disabled_background_app() -> Element {
    let mut open = use_signal(|| false);
    let mut opens = use_signal(|| 0);
    rsx! {
        ThemeProvider {
            div { style: "direction:column;height:100%",
                Button {
                    variant: "danger",
                    disabled: open(),
                    on_click: move |_| {
                        opens += 1;
                        open.set(true);
                    },
                    "open dialog"
                }
                span { "opens {opens()}" }
                Modal {
                    open: open(),
                    title: "Confirm",
                    on_close: move |_| open.set(false),
                    Text { "delete file?" }
                    div { style: "direction:row;gap:2;justify:end",
                        Button { on_click: move |_| open.set(false), "Accept" }
                    }
                }
            }
        }
    }
}

#[test]
fn focus_activation_survives_a_closed_trap() {
    let mut harness = Harness::new(disabled_background_app);
    let background = harness.button("open dialog");
    assert_eq!(harness.renderer.focus.focused_id(), Some(background));

    harness.key(TuiKey::Enter);
    assert!(
        harness.screen().contains("delete file?"),
        "{}",
        harness.screen()
    );
    harness.key(TuiKey::Escape);
    assert!(
        !harness.screen().contains("delete file?"),
        "{}",
        harness.screen()
    );
    assert_eq!(harness.renderer.focus.focused_id(), Some(background));

    harness.key(TuiKey::Enter);
    assert!(
        harness.screen().contains("delete file?"),
        "Enter should reopen from the restored focus:\n{}",
        harness.screen()
    );
    harness.key(TuiKey::Escape);
    assert!(
        harness.screen().contains("opens 2"),
        "the restored focus must activate the page button again:\n{}",
        harness.screen()
    );
}

#[test]
fn modal_traps_focus_blocks_the_page_and_closes() {
    let mut harness = Harness::new(modal_app);
    let screen = harness.screen();
    assert!(screen.contains("delete file?"), "{screen}");
    assert!(
        !screen.contains("background action"),
        "the overlay should paint over the page:\n{screen}"
    );

    let accept = harness.button("Accept");
    let reject = harness.button("Reject");
    let close = harness.button("Close");
    let background = harness.button("background action");
    assert_eq!(harness.renderer.focus.focused_id(), Some(accept));

    harness.key(TuiKey::Tab);
    assert_eq!(harness.renderer.focus.focused_id(), Some(reject));
    harness.key(TuiKey::Tab);
    assert_eq!(harness.renderer.focus.focused_id(), Some(close));
    harness.key(TuiKey::Tab);
    assert_eq!(harness.renderer.focus.focused_id(), Some(accept));

    let card = harness.node_where(|node| {
        node.attrs.border == Some(BorderKind::Rounded)
            && node.attrs.title.as_deref() == Some("Confirm")
    });
    let card_rect = harness.renderer.layout[&card];
    harness.click(card_rect.x + 1, card_rect.y);
    assert!(
        harness.screen().contains("delete file?"),
        "card clicks must not close the modal"
    );

    let bg_rect = harness.renderer.layout[&background];
    harness.click(bg_rect.x + bg_rect.width / 2, bg_rect.y);
    assert_eq!(
        harness.dom.in_runtime(|| BG_CLICKS.with(|count| *count)),
        0,
        "pointer events outside the trap must be ignored"
    );
    assert!(
        harness.screen().contains("status false"),
        "a backdrop click closes the modal:\n{}",
        harness.screen()
    );

    let bg_rect = harness.renderer.layout[&background];
    harness.click(bg_rect.x + bg_rect.width / 2, bg_rect.y);
    assert_eq!(harness.dom.in_runtime(|| BG_CLICKS.with(|count| *count)), 1);
    assert!(harness.screen().contains("delete file?"));

    harness.key(TuiKey::Escape);
    assert!(
        harness.screen().contains("status false"),
        "{}",
        harness.screen()
    );
    assert_eq!(
        harness.renderer.focus.focused_id(),
        Some(background),
        "focus returns to the page when the trap closes"
    );
}
