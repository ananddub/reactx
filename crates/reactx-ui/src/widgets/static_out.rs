//! Write-once output for the terminal scrollback.

use dioxus::prelude::*;
use reactx_core::Line;

/// Append `items` to the terminal scrollback, once each.
///
/// Only items added since the last render are emitted, so the live view below
/// keeps redrawing while finished output scrolls away above it. Pair this with
/// `launch_inline` from `reactx-tui` so there is a live view to sit under.
///
/// The list should only grow: shrinking it makes already-emitted items
/// eligible again the next time it grows past the old length.
#[component]
pub fn Static(items: Vec<Line<'static>>) -> Element {
    let mut seen = use_signal(|| 0usize);
    let start = *seen.peek();
    if items.len() > start {
        reactx_core::static_out::append_lines(items[start..].iter().cloned());
        seen.set(items.len());
    }

    rsx! {}
}
