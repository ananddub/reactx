//! Screen-reader detection shared by widgets and the launcher.
//!
//! Widgets read the flag while rendering: when assistive output is on,
//! `aria-label` replaces children and `aria-hidden` removes an element from the
//! output. The launcher seeds it from `LaunchOptions`; outside a run it falls
//! back to `REACTX_SCREEN_READER` / `INK_SCREEN_READER`.

use std::cell::Cell;

thread_local! {
    static ENABLED: Cell<Option<bool>> = const { Cell::new(None) };
}

/// Force screen-reader mode on or off. `None` restores environment detection.
pub fn set_enabled(enabled: Option<bool>) {
    ENABLED.with(|slot| slot.set(enabled));
}

/// Whether assistive output is active.
pub fn is_enabled() -> bool {
    ENABLED.with(Cell::get).unwrap_or_else(env_enabled)
}

fn env_enabled() -> bool {
    ["REACTX_SCREEN_READER", "INK_SCREEN_READER"]
        .iter()
        .any(|key| std::env::var(key).is_ok_and(|value| value == "true"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forced_value_wins_and_resets() {
        set_enabled(Some(true));
        assert!(is_enabled());
        set_enabled(Some(false));
        assert!(!is_enabled());
        set_enabled(None);
    }
}
