//! Screen-reader mode, mirroring Ink's `useIsScreenReaderEnabled`.

/// Whether assistive output is active.
///
/// When true, widgets announce `aria-label` text instead of their children and
/// drop elements marked `aria-hidden`.
pub fn use_is_screen_reader_enabled() -> bool {
    reactx_core::screen::is_enabled()
}
