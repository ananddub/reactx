//! Terminal size shared with components.
//!
//! `launch` seeds the size before the first frame and updates it on every
//! resize, so reading [`use_terminal_size`] during render subscribes the
//! component to future size changes.

use dioxus_signals::{GlobalSignal, Signal};

/// Latest terminal size in cells (width, height).
pub static TERMINAL_SIZE: GlobalSignal<(u16, u16)> = Signal::global(|| (80, 24));

/// Read the terminal size for the current window. Calling this inside a
/// component rerenders it when the terminal is resized.
pub fn use_terminal_size() -> (u16, u16) {
    TERMINAL_SIZE()
}

/// Replace the stored size. Must run inside the Dioxus runtime.
pub fn set_terminal_size(dom: &dioxus_core::VirtualDom, width: u16, height: u16) {
    dom.in_runtime(|| {
        TERMINAL_SIZE.with_mut(|size| *size = (width, height));
    });
}
