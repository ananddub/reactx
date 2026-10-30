//! Ink-style `useCursor`: place or hide the terminal cursor from a component.

use std::cell::Cell;

/// A component-provided cursor position that wins over the widget caret.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum CursorOverride {
    /// Let the renderer place the caret (focused text input).
    #[default]
    Auto,
    /// Keep the cursor hidden.
    Hidden,
    /// Pin the cursor to a cell.
    At(u16, u16),
}

thread_local! {
    static OVERRIDE: Cell<CursorOverride> = const { Cell::new(CursorOverride::Auto) };
}

/// The active override, read by the renderer after recomputing layout.
pub(crate) fn current() -> CursorOverride {
    OVERRIDE.with(Cell::get)
}

pub(crate) fn clear() {
    OVERRIDE.with(|slot| slot.set(CursorOverride::Auto));
}

/// Handle for moving the terminal cursor, returned by [`use_cursor`].
#[derive(Clone, Copy, Debug, Default)]
pub struct CursorHandle;

impl CursorHandle {
    /// Pin the cursor to `(x, y)` in terminal cells.
    pub fn set(&self, x: u16, y: u16) {
        OVERRIDE.with(|slot| slot.set(CursorOverride::At(x, y)));
    }

    /// Hide the cursor.
    pub fn hide(&self) {
        OVERRIDE.with(|slot| slot.set(CursorOverride::Hidden));
    }

    /// Give control back to the focused widget's caret.
    pub fn reset(&self) {
        OVERRIDE.with(|slot| slot.set(CursorOverride::Auto));
    }
}

/// Move or hide the terminal cursor from any component, like Ink's `useCursor`.
pub fn use_cursor() -> CursorHandle {
    CursorHandle
}
