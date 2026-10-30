//! Terminal size and focus shared with components.
//!
//! `launch` seeds the size before the first frame and updates it on every
//! resize, so reading [`use_terminal_size`] during render subscribes the
//! component to future size changes. [`use_terminal_focus`] works the same
//! way for terminal focus gained/lost events.

use dioxus_signals::{GlobalSignal, Signal};

/// Latest terminal size in cells (width, height).
pub static TERMINAL_SIZE: GlobalSignal<(u16, u16)> = Signal::global(|| (80, 24));

/// Whether the terminal window currently has OS focus.
pub static TERMINAL_FOCUS: GlobalSignal<bool> = Signal::global(|| true);

/// Read the terminal size for the current window. Calling this inside a
/// component rerenders it when the terminal is resized.
pub fn use_terminal_size() -> (u16, u16) {
    TERMINAL_SIZE()
}

/// Ink-style alias for [`use_terminal_size`].
pub fn use_window_size() -> (u16, u16) {
    use_terminal_size()
}

/// Read whether the terminal has focus. Calling this inside a component
/// rerenders it when the terminal gains or loses OS focus.
pub fn use_terminal_focus() -> bool {
    TERMINAL_FOCUS()
}

/// Replace the stored size. Must run inside the Dioxus runtime.
pub fn set_terminal_size(dom: &dioxus_core::VirtualDom, width: u16, height: u16) {
    dom.in_runtime(|| {
        TERMINAL_SIZE.with_mut(|size| *size = (width, height));
    });
}

/// Replace the stored focus state. Must run inside the Dioxus runtime.
pub fn set_terminal_focus(dom: &dioxus_core::VirtualDom, focused: bool) {
    dom.in_runtime(|| {
        TERMINAL_FOCUS.with_mut(|state| *state = focused);
    });
}
