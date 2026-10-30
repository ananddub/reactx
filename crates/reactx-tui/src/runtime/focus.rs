//! Ink-style `useFocus` and `useFocusManager` hooks.
//!
//! Components identify themselves with a user `id`. The run loop mirrors the
//! focus ring into [`FOCUSED_ID`] after each frame, so `use_focus` rerenders
//! when focus moves. Imperative commands are queued and applied by the loop,
//! which owns the renderer.

use std::cell::{Cell, RefCell};

use dioxus_core::prelude::{use_drop, use_hook};
use dioxus_core::VirtualDom;
use dioxus_signals::{GlobalSignal, Readable, Signal};

use crate::events::focus::apply_focus;
use crate::TuiRenderer;

/// User id of the focused element.
pub static FOCUSED_ID: GlobalSignal<Option<String>> = Signal::global(|| None);

#[derive(Debug, Clone, PartialEq, Eq)]
enum FocusCommand {
    Next,
    Previous,
    Focus(String),
    Blur,
}

thread_local! {
    static COMMANDS: RefCell<Vec<FocusCommand>> = const { RefCell::new(Vec::new()) };
    static REGISTERED: RefCell<Vec<(u64, String)>> = const { RefCell::new(Vec::new()) };
    static NEXT_TOKEN: Cell<u64> = const { Cell::new(1) };
    static ENABLED: Cell<bool> = const { Cell::new(true) };
}

fn next_token() -> u64 {
    NEXT_TOKEN.with(|token| {
        let value = token.get();
        token.set(value + 1);
        value
    })
}

/// Whether focus navigation is enabled by [`FocusManagerHandle::disable_focus`].
pub(crate) fn is_enabled() -> bool {
    ENABLED.with(Cell::get)
}

/// Whether `id` was registered through [`use_focus`]. Registered elements join
/// the focus ring even without a `focusable` attribute.
pub(crate) fn is_registered(id: &str) -> bool {
    REGISTERED.with(|entries| entries.borrow().iter().any(|(_, known)| known == id))
}

/// Whether the element with user id `id` currently has focus.
pub fn use_focus(id: &str) -> bool {
    let token = use_hook(next_token);
    let key = id.to_string();
    REGISTERED.with(|entries| {
        let mut entries = entries.borrow_mut();
        match entries.iter_mut().find(|(known, _)| *known == token) {
            Some(entry) => entry.1 = key,
            None => entries.push((token, key)),
        }
    });
    use_drop(move || {
        REGISTERED.with(|entries| entries.borrow_mut().retain(|(known, _)| *known != token))
    });
    FOCUSED_ID().as_deref() == Some(id)
}

/// Imperative focus controls, mirroring Ink's `useFocusManager()`.
#[derive(Clone, Copy, Debug, Default)]
pub struct FocusManagerHandle;

impl FocusManagerHandle {
    /// Move focus to the next focusable element.
    pub fn focus_next(&self) {
        push(FocusCommand::Next);
    }

    /// Move focus to the previous focusable element.
    pub fn focus_previous(&self) {
        push(FocusCommand::Previous);
    }

    /// Focus the element registered under `id`.
    pub fn focus(&self, id: &str) {
        push(FocusCommand::Focus(id.to_string()));
    }

    /// Remove focus from every element.
    pub fn blur(&self) {
        push(FocusCommand::Blur);
    }

    /// Allow Tab and focus commands again.
    pub fn enable_focus(&self) {
        ENABLED.with(|enabled| enabled.set(true));
    }

    /// Freeze focus navigation, like Ink's `disableFocus()`.
    pub fn disable_focus(&self) {
        ENABLED.with(|enabled| enabled.set(false));
    }

    /// The user id of the focused element, if any.
    pub fn active_id(&self) -> Option<String> {
        FOCUSED_ID.peek().clone()
    }
}

/// Access the focus manager for imperative focus control.
pub fn use_focus_manager() -> FocusManagerHandle {
    FocusManagerHandle
}

fn push(command: FocusCommand) {
    COMMANDS.with(|commands| commands.borrow_mut().push(command));
}

/// Apply every queued command to the renderer's focus ring.
pub(crate) fn drain(dom: &VirtualDom, renderer: &mut TuiRenderer) {
    let commands: Vec<FocusCommand> =
        COMMANDS.with(|queue| std::mem::take(&mut *queue.borrow_mut()));
    for command in commands {
        let change = match command {
            FocusCommand::Next if is_enabled() => renderer.focus.focus_next(),
            FocusCommand::Previous if is_enabled() => renderer.focus.focus_prev(),
            FocusCommand::Focus(id) => match renderer.tree.find_by_user_id(&id) {
                Some(node) => renderer.focus.set_focus(Some(node)),
                None => continue,
            },
            FocusCommand::Blur => renderer.focus.clear(),
            _ => continue,
        };
        apply_focus(dom, renderer, change);
    }
}

/// Mirror the focused node's user id into [`FOCUSED_ID`].
pub(crate) fn publish(dom: &VirtualDom, renderer: &TuiRenderer) {
    let id = focused_user_id(renderer);
    let unchanged = dom.in_runtime(|| *FOCUSED_ID.peek() == id);
    if !unchanged {
        dom.in_runtime(|| FOCUSED_ID.with_mut(|slot| *slot = id));
    }
}

fn focused_user_id(renderer: &TuiRenderer) -> Option<String> {
    if !is_enabled() {
        return None;
    }
    renderer
        .focus
        .focused_id()
        .and_then(|node| renderer.tree.get(node))
        .and_then(|node| node.attrs.custom.get("id").cloned())
}

/// Forget every registration and queued command.
pub(crate) fn clear() {
    COMMANDS.with(|commands| commands.borrow_mut().clear());
    REGISTERED.with(|entries| entries.borrow_mut().clear());
    ENABLED.with(|enabled| enabled.set(true));
}
