//! Crossterm event translation.

use crossterm::event::{
    Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton,
    MouseEventKind,
};
use reactx_core::{KeyModifiers as TuiModifiers, MouseButtonKind, TuiEvent, TuiKey, TuiKeyEvent};

pub(super) fn translate(event: CrosstermEvent) -> Option<TuiEvent> {
    match event {
        CrosstermEvent::Key(key) => {
            if matches!(key.kind, KeyEventKind::Release) {
                None
            } else {
                Some(TuiEvent::Key(translate_key(key)))
            }
        }
        CrosstermEvent::Mouse(mouse) => match mouse.kind {
            MouseEventKind::Down(button) => Some(TuiEvent::MouseDown {
                x: mouse.column,
                y: mouse.row,
                button: translate_button(button),
            }),
            MouseEventKind::Up(button) => Some(TuiEvent::MouseUp {
                x: mouse.column,
                y: mouse.row,
                button: translate_button(button),
            }),
            MouseEventKind::Moved => Some(TuiEvent::MouseMove {
                x: mouse.column,
                y: mouse.row,
            }),
            MouseEventKind::Drag(button) => Some(TuiEvent::MouseDrag {
                x: mouse.column,
                y: mouse.row,
                button: translate_button(button),
            }),
            MouseEventKind::ScrollUp => Some(TuiEvent::Scroll {
                x: mouse.column,
                y: mouse.row,
                delta_x: 0,
                delta_y: -1,
            }),
            MouseEventKind::ScrollDown => Some(TuiEvent::Scroll {
                x: mouse.column,
                y: mouse.row,
                delta_x: 0,
                delta_y: 1,
            }),
            MouseEventKind::ScrollLeft => Some(TuiEvent::Scroll {
                x: mouse.column,
                y: mouse.row,
                delta_x: -1,
                delta_y: 0,
            }),
            MouseEventKind::ScrollRight => Some(TuiEvent::Scroll {
                x: mouse.column,
                y: mouse.row,
                delta_x: 1,
                delta_y: 0,
            }),
        },
        CrosstermEvent::Resize(width, height) => Some(TuiEvent::Resize { width, height }),
        CrosstermEvent::FocusGained => Some(TuiEvent::FocusGained),
        CrosstermEvent::FocusLost => Some(TuiEvent::FocusLost),
        CrosstermEvent::Paste(text) => Some(TuiEvent::Paste(text)),
    }
}

fn translate_button(button: MouseButton) -> MouseButtonKind {
    match button {
        MouseButton::Left => MouseButtonKind::Left,
        MouseButton::Right => MouseButtonKind::Right,
        MouseButton::Middle => MouseButtonKind::Middle,
    }
}

fn translate_key(key: KeyEvent) -> TuiKeyEvent {
    let tui_key = match key.code {
        KeyCode::Char(ch) => TuiKey::Char(ch),
        KeyCode::Enter => TuiKey::Enter,
        KeyCode::Tab => TuiKey::Tab,
        KeyCode::BackTab => TuiKey::BackTab,
        KeyCode::Backspace => TuiKey::Backspace,
        KeyCode::Delete => TuiKey::Delete,
        KeyCode::Esc => TuiKey::Escape,
        KeyCode::Left => TuiKey::Left,
        KeyCode::Right => TuiKey::Right,
        KeyCode::Up => TuiKey::Up,
        KeyCode::Down => TuiKey::Down,
        KeyCode::Home => TuiKey::Home,
        KeyCode::End => TuiKey::End,
        KeyCode::PageUp => TuiKey::PageUp,
        KeyCode::PageDown => TuiKey::PageDown,
        KeyCode::Insert => TuiKey::Insert,
        KeyCode::F(number) => TuiKey::F(number),
        other => TuiKey::Other(format!("{other:?}")),
    };

    let modifiers = key.modifiers;
    TuiKeyEvent {
        key: tui_key,
        modifiers: TuiModifiers {
            shift: modifiers.contains(KeyModifiers::SHIFT),
            ctrl: modifiers.contains(KeyModifiers::CONTROL),
            alt: modifiers.contains(KeyModifiers::ALT),
        },
    }
}
