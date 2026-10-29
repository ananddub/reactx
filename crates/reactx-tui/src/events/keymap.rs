//! Terminal key and modifier translation.

use dioxus_html::input_data::keyboard_types::{Code, Key, Modifiers};
use reactx_core::TuiKey;

pub(super) fn map_modifiers(modifiers: reactx_core::KeyModifiers) -> Modifiers {
    let mut out = Modifiers::empty();
    if modifiers.shift {
        out |= Modifiers::SHIFT;
    }
    if modifiers.ctrl {
        out |= Modifiers::CONTROL;
    }
    if modifiers.alt {
        out |= Modifiers::ALT;
    }
    out
}

pub(super) fn map_key(key: &TuiKey) -> (Key, Code) {
    match key {
        TuiKey::Char(ch) => (Key::Character(ch.to_string()), char_code(*ch)),
        TuiKey::Enter => (Key::Enter, Code::Enter),
        TuiKey::Tab => (Key::Tab, Code::Tab),
        TuiKey::BackTab => (Key::Tab, Code::Tab),
        TuiKey::Backspace => (Key::Backspace, Code::Backspace),
        TuiKey::Delete => (Key::Delete, Code::Delete),
        TuiKey::Escape => (Key::Escape, Code::Escape),
        TuiKey::Left => (Key::ArrowLeft, Code::ArrowLeft),
        TuiKey::Right => (Key::ArrowRight, Code::ArrowRight),
        TuiKey::Up => (Key::ArrowUp, Code::ArrowUp),
        TuiKey::Down => (Key::ArrowDown, Code::ArrowDown),
        TuiKey::Home => (Key::Home, Code::Home),
        TuiKey::End => (Key::End, Code::End),
        TuiKey::PageUp => (Key::PageUp, Code::PageUp),
        TuiKey::PageDown => (Key::PageDown, Code::PageDown),
        TuiKey::Insert => (Key::Insert, Code::Insert),
        TuiKey::F(n) => (function_key(*n), function_code(*n)),
        TuiKey::Other(_) => (Key::Unidentified, Code::Unidentified),
    }
}

fn function_key(n: u8) -> Key {
    match n {
        1 => Key::F1,
        2 => Key::F2,
        3 => Key::F3,
        4 => Key::F4,
        5 => Key::F5,
        6 => Key::F6,
        7 => Key::F7,
        8 => Key::F8,
        9 => Key::F9,
        10 => Key::F10,
        11 => Key::F11,
        12 => Key::F12,
        _ => Key::Unidentified,
    }
}

fn function_code(n: u8) -> Code {
    match n {
        1 => Code::F1,
        2 => Code::F2,
        3 => Code::F3,
        4 => Code::F4,
        5 => Code::F5,
        6 => Code::F6,
        7 => Code::F7,
        8 => Code::F8,
        9 => Code::F9,
        10 => Code::F10,
        11 => Code::F11,
        12 => Code::F12,
        _ => Code::Unidentified,
    }
}

fn char_code(ch: char) -> Code {
    match ch {
        'a'..='z' => char_code(ch.to_ascii_uppercase()),
        'A'..='Z' => {
            let offset = ch as u8 - b'A';
            const LETTERS: [Code; 26] = [
                Code::KeyA,
                Code::KeyB,
                Code::KeyC,
                Code::KeyD,
                Code::KeyE,
                Code::KeyF,
                Code::KeyG,
                Code::KeyH,
                Code::KeyI,
                Code::KeyJ,
                Code::KeyK,
                Code::KeyL,
                Code::KeyM,
                Code::KeyN,
                Code::KeyO,
                Code::KeyP,
                Code::KeyQ,
                Code::KeyR,
                Code::KeyS,
                Code::KeyT,
                Code::KeyU,
                Code::KeyV,
                Code::KeyW,
                Code::KeyX,
                Code::KeyY,
                Code::KeyZ,
            ];
            LETTERS[offset as usize]
        }
        '0'..='9' => {
            let offset = ch as u8 - b'0';
            const DIGITS: [Code; 10] = [
                Code::Digit0,
                Code::Digit1,
                Code::Digit2,
                Code::Digit3,
                Code::Digit4,
                Code::Digit5,
                Code::Digit6,
                Code::Digit7,
                Code::Digit8,
                Code::Digit9,
            ];
            DIGITS[offset as usize]
        }
        ' ' => Code::Space,
        _ => Code::Unidentified,
    }
}
