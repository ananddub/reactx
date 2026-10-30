//! Raw input hooks mirroring Ink's `useInput` and `usePaste`.

use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};

use dioxus_core::prelude::{use_drop, use_hook};
use dioxus_core::VirtualDom;
use reactx_core::{TuiKey, TuiKeyEvent};

type InputHandler = Box<dyn FnMut(String, TuiKeyEvent)>;
type PasteHandler = Box<dyn FnMut(String)>;

struct Registration<T> {
    id: u64,
    active: bool,
    handler: T,
}

thread_local! {
    static INPUT: RefCell<Vec<Registration<InputHandler>>> = RefCell::new(Vec::new());
    static PASTE: RefCell<Vec<Registration<PasteHandler>>> = RefCell::new(Vec::new());
    static NEXT: AtomicU64 = const { AtomicU64::new(1) };
}

fn next_id() -> u64 {
    NEXT.with(|next| next.fetch_add(1, Ordering::Relaxed))
}

fn install<T>(
    list: &'static std::thread::LocalKey<RefCell<Vec<Registration<T>>>>,
    id: u64,
    active: bool,
    handler: T,
) {
    list.with(|entries| {
        let mut entries = entries.borrow_mut();
        match entries.iter_mut().find(|entry| entry.id == id) {
            Some(entry) => {
                entry.active = active;
                entry.handler = handler;
            }
            None => entries.push(Registration {
                id,
                active,
                handler,
            }),
        }
    });
}

fn uninstall<T>(list: &'static std::thread::LocalKey<RefCell<Vec<Registration<T>>>>, id: u64) {
    list.with(|entries| entries.borrow_mut().retain(|entry| entry.id != id));
}

/// Register a handler for every key press, wherever focus is.
///
/// The handler receives the printable part of the key (`""` for arrows and
/// modifiers) and the full key event. Handlers run before focused widgets see
/// the key, matching Ink's raw-mode model.
pub fn use_input(handler: impl FnMut(String, TuiKeyEvent) + 'static) {
    use_input_when(true, handler)
}

/// [`use_input`] with an `is_active` switch, like Ink's second argument.
pub fn use_input_when(active: bool, handler: impl FnMut(String, TuiKeyEvent) + 'static) {
    let id = use_hook(next_id);
    use_drop(move || uninstall(&INPUT, id));
    install(&INPUT, id, active, Box::new(handler));
}

/// Register a handler for bracketed-paste events.
pub fn use_paste(handler: impl FnMut(String) + 'static) {
    use_paste_when(true, handler)
}

/// [`use_paste`] with an `is_active` switch.
pub fn use_paste_when(active: bool, handler: impl FnMut(String) + 'static) {
    let id = use_hook(next_id);
    use_drop(move || uninstall(&PASTE, id));
    install(&PASTE, id, active, Box::new(handler));
}

/// Deliver a key press to every active [`use_input`] handler.
pub(crate) fn dispatch(dom: &VirtualDom, key: &TuiKeyEvent) {
    let input = match &key.key {
        TuiKey::Char(ch) if !key.modifiers.ctrl && !key.modifiers.alt => ch.to_string(),
        _ => String::new(),
    };
    dom.in_runtime(|| {
        INPUT.with(|entries| {
            for entry in entries.borrow_mut().iter_mut() {
                if entry.active {
                    (entry.handler)(input.clone(), key.clone());
                }
            }
        });
    });
}

/// Deliver a paste event to every active [`use_paste`] handler.
pub(crate) fn dispatch_paste(dom: &VirtualDom, text: &str) {
    dom.in_runtime(|| {
        PASTE.with(|entries| {
            for entry in entries.borrow_mut().iter_mut() {
                if entry.active {
                    (entry.handler)(text.to_string());
                }
            }
        });
    });
}

/// Forget every registration, used when a run starts or ends.
pub(crate) fn clear() {
    INPUT.with(|entries| entries.borrow_mut().clear());
    PASTE.with(|entries| entries.borrow_mut().clear());
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};

    use dioxus::prelude::*;
    use reactx_core::{KeyModifiers, TuiKey, TuiKeyEvent};

    use super::*;

    thread_local! {
        static HITS: Cell<usize> = const { Cell::new(0) };
        static SEEN: RefCell<String> = const { RefCell::new(String::new()) };
    }

    fn app() -> Element {
        use_input(|input, _key| {
            HITS.with(|hits| hits.set(hits.get() + 1));
            SEEN.with(|seen| seen.borrow_mut().push_str(&input));
        });
        rsx! { div { "x" } }
    }

    fn char_key(ch: char) -> TuiKeyEvent {
        TuiKeyEvent {
            key: TuiKey::Char(ch),
            modifiers: KeyModifiers::NONE,
        }
    }

    #[test]
    fn handlers_receive_printable_characters() {
        clear();
        HITS.with(|hits| hits.set(0));
        SEEN.with(|seen| seen.borrow_mut().clear());
        let mut dom = VirtualDom::new(app);
        let mut renderer = crate::TuiRenderer::new();
        dom.rebuild(&mut renderer);

        dispatch(&dom, &char_key('a'));
        let arrow = TuiKeyEvent {
            key: TuiKey::Left,
            modifiers: KeyModifiers::NONE,
        };
        dispatch(&dom, &arrow);

        assert_eq!(HITS.with(Cell::get), 2);
        assert_eq!(SEEN.with(|seen| seen.borrow().clone()), "a");
        clear();
    }
}
