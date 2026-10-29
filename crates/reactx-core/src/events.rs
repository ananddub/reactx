/// Framework-level key representation, independent of the terminal backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiKey {
    Char(char),
    Enter,
    Tab,
    BackTab,
    Backspace,
    Delete,
    Escape,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    F(u8),
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

impl KeyModifiers {
    pub const NONE: Self = Self {
        shift: false,
        ctrl: false,
        alt: false,
    };

    pub fn is_empty(self) -> bool {
        !self.shift && !self.ctrl && !self.alt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiKeyEvent {
    pub key: TuiKey,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonKind {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiEvent {
    Key(TuiKeyEvent),
    MouseClick {
        x: u16,
        y: u16,
    },
    MouseDown {
        x: u16,
        y: u16,
        button: MouseButtonKind,
    },
    MouseUp {
        x: u16,
        y: u16,
        button: MouseButtonKind,
    },
    /// Pointer movement with no button held.
    MouseMove {
        x: u16,
        y: u16,
    },
    /// Pointer movement while a button is held.
    MouseDrag {
        x: u16,
        y: u16,
        button: MouseButtonKind,
    },
    /// Mouse wheel movement in cells at the given terminal position.
    Scroll {
        x: u16,
        y: u16,
        delta_x: i32,
        delta_y: i32,
    },
    Resize {
        width: u16,
        height: u16,
    },
    FocusGained,
    FocusLost,
    Paste(String),
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusEvent {
    Focus,
    Blur,
}
