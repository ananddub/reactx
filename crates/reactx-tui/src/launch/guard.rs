//! Raw mode and terminal feature lifecycle.

use std::io;

use crossterm::cursor::Show;
use crossterm::event::{
    DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
    EnableFocusChange, EnableMouseCapture, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use super::Mode;

use std::sync::Once;

static PANIC_HOOK: Once = Once::new();

/// Restore the terminal to a usable state. Safe to call at any time.
pub fn restore_terminal() {
    restore(true);
}

/// Install a panic hook that restores the terminal before the default hook
/// prints the panic message. Calling it more than once is a no-op.
pub fn install_panic_hook() {
    PANIC_HOOK.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            restore_terminal();
            previous(info);
        }));
    });
}

fn restore(alternate_screen: bool) {
    let _ = disable_raw_mode();
    if alternate_screen {
        let _ = execute!(
            io::stdout(),
            PopKeyboardEnhancementFlags,
            DisableMouseCapture,
            DisableBracketedPaste,
            DisableFocusChange,
            LeaveAlternateScreen,
            Show
        );
    } else {
        let _ = execute!(
            io::stdout(),
            PopKeyboardEnhancementFlags,
            DisableMouseCapture,
            DisableBracketedPaste,
            DisableFocusChange,
            Show
        );
    }
}

/// Claims the terminal for the lifetime of the run loop. Inline mode leaves
/// the normal screen in place so earlier output stays in the scrollback.
pub(crate) struct TerminalGuard {
    alternate_screen: bool,
}

impl TerminalGuard {
    pub(crate) fn enter(mode: Mode, kitty_keyboard: bool) -> io::Result<Self> {
        enable_raw_mode()?;
        if kitty_keyboard {
            execute!(
                io::stdout(),
                PushKeyboardEnhancementFlags(
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                        | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                )
            )?;
        }
        execute!(
            io::stdout(),
            EnableMouseCapture,
            EnableBracketedPaste,
            EnableFocusChange,
            Show
        )?;
        let alternate_screen = matches!(mode, Mode::Fullscreen);
        if alternate_screen {
            execute!(io::stdout(), EnterAlternateScreen)?;
        }
        Ok(Self { alternate_screen })
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        restore(self.alternate_screen);
    }
}
