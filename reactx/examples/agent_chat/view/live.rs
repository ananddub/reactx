//! In-place animated status line under the transcript.

use std::time::{Duration, Instant};

use reactx::prelude::*;

use crate::script::Item;

use super::chrome::SPINNER;

const SHIMMER: [&str; 5] = [
    "text-violet-300",
    "text-indigo-300",
    "text-sky-300",
    "text-teal-300",
    "text-emerald-300",
];

/// "Thinking… / Generating…" line that morphs in place every tick instead of
/// appending new rows, so streaming stays cheap.
#[component]
pub fn LiveStatus(items: Signal<Vec<Item>>) -> Element {
    let tick = use_ticker(Duration::from_millis(80))();
    let start = use_hook(Instant::now);
    let list = items();
    let (word, active) = phase(list.last());
    let spin = SPINNER[tick as usize % SPINNER.len()];
    let dots = ".".repeat(1 + (tick as usize / 2) % 3);
    let spin_color = if active {
        SHIMMER[(tick as usize / 2) % SHIMMER.len()]
    } else {
        "text-slate-600"
    };
    let elapsed = format_elapsed(start.elapsed());
    jsx! {
        <div class="flex flex-row justify-between px-1">
            <div class="flex flex-row gap-1">
                <span class={spin_color}>{spin}</span>
                <span class={if active { "text-slate-400" } else { "text-slate-500" }}>
                    {format!("{word}{dots}")}
                </span>
                <span class="text-slate-600">{format!("(esc to cancel, {elapsed})")}</span>
            </div>
            <span class="text-slate-600">{"? for shortcuts"}</span>
        </div>
    }
}

fn phase(last: Option<&Item>) -> (&'static str, bool) {
    match last {
        Some(Item::Tool { .. }) => ("Running", true),
        Some(Item::Answer(_)) => ("Generating", true),
        Some(Item::Hint(_)) => ("Waiting for input", false),
        _ => ("Thinking", true),
    }
}

fn format_elapsed(elapsed: Duration) -> String {
    let secs = elapsed.as_secs();
    if secs >= 60 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{secs}s")
    }
}
