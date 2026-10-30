//! Raw mode and terminal feature lifecycle.

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

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

static PANIC_HOOK: Once = Once::new();
static CLAIMED: AtomicBool = AtomicBool::new(false);
static PUSHED: AtomicBool = AtomicBool::new(false);
static ALTERNATE: AtomicBool = AtomicBool::new(false);

/// Restore the terminal to a usable state. Safe to call at any time; cleanup
/// runs exactly once, so the panic hook and the terminal guard cannot double
/// restore.
pub fn restore_terminal() {
    restore();
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

fn restore() {
    if !CLAIMED.swap(false, Ordering::SeqCst) {
        return;
    }
    let pushed = PUSHED.swap(false, Ordering::SeqCst);
    let alternate = ALTERNATE.swap(false, Ordering::SeqCst);
    let _ = disable_raw_mode();
    if pushed {
        let _ = execute!(io::stdout(), PopKeyboardEnhancementFlags);
    }
    let _ = execute!(
        io::stdout(),
        DisableMouseCapture,
        DisableBracketedPaste,
        DisableFocusChange
    );
    if alternate {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
    let _ = execute!(io::stdout(), Show);
}

/// Claims the terminal for the lifetime of the run loop. Inline mode leaves
/// the normal screen in place so earlier output stays in the scrollback.
pub(crate) struct TerminalGuard;

impl TerminalGuard {
    pub(crate) fn enter(mode: Mode, kitty_keyboard: bool) -> io::Result<Self> {
        enable_raw_mode()?;
        CLAIMED.store(true, Ordering::SeqCst);
        if let Err(err) = claim(mode, kitty_keyboard) {
            restore();
            return Err(err);
        }
        Ok(Self)
    }
}

fn claim(mode: Mode, kitty_keyboard: bool) -> io::Result<()> {
    let mut out = io::stdout();
    if matches!(mode, Mode::Fullscreen) {
        execute!(out, EnterAlternateScreen)?;
        ALTERNATE.store(true, Ordering::SeqCst);
    }
    execute!(
        out,
        EnableMouseCapture,
        EnableBracketedPaste,
        EnableFocusChange,
        Show
    )?;
    // The kitty protocol keeps one keyboard stack per screen, so the push has
    // to happen while the screen that will pop it is active: after entering
    // the alternate screen, and popped before leaving it. Pushing on the main
    // screen instead left the shell's screen with the flags still enabled.
    if kitty_keyboard {
        execute!(
            out,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                    | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
            )
        )?;
        PUSHED.store(true, Ordering::SeqCst);
    }
    Ok(())
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        restore();
    }
}
