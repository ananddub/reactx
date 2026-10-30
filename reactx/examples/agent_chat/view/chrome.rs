//! Horizontal rule, composer and status bar around the transcript.

use std::time::Duration;

use reactx::prelude::*;

pub(super) const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Full-width horizontal rule that follows terminal resizes.
#[component]
pub fn Rule() -> Element {
    let (width, _) = use_terminal_size();
    let line = "─".repeat(width.saturating_sub(2) as usize);
    jsx! {
        <div class="px-1">
            <span class="text-slate-700">{line}</span>
        </div>
    }
}

/// Single-line prompt. Enter submits the current value.
#[component]
pub fn Composer(input: Signal<String>, on_submit: EventHandler<String>) -> Element {
    let value = input();
    let mut field = input;
    jsx! {
        <div class="flex flex-row gap-1 px-1">
            <span class="text-rose-400 bold">{"*"}</span>
            <input
                class="grow text-slate-100"
                style="border:none;padding:0"
                value={value}
                placeholder="Message ReactX agent… (Enter to send)"
                oninput=move |event: FormEvent| field.set(event.value())
                onkeydown=move |event: KeyboardEvent| {
                    if event.key() == Key::Enter {
                        let text = field.peek().to_string();
                        on_submit.call(text);
                    }
                }
            />
        </div>
    }
}

/// Spinner, model info, context usage, working directory and diff stats.
#[component]
pub fn StatusBar() -> Element {
    let tick = use_ticker(Duration::from_millis(80))();
    let spinner = SPINNER[tick as usize % SPINNER.len()];
    let ctx = 17 + (tick / 45 % 6) as usize;
    let cells = 5usize;
    let filled = (ctx * cells / 100).max(1);
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(cells - filled));
    let cwd = use_hook(|| {
        std::env::current_dir()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| "~".to_string())
    });
    jsx! {
        <div class="flex flex-row justify-between px-1">
            <div class="flex flex-row gap-1">
                <span class="text-slate-400">
                    {format!("{spinner} DeepSeek-Flash Model · 200K context · ctx")}
                </span>
                <span class="text-slate-500">{bar}</span>
                <span class="text-slate-400">{format!("{ctx}% · {cwd}")}</span>
            </div>
            <div class="flex flex-row gap-1">
                <span class="text-green-400">{"+29898"}</span>
                <span class="text-rose-400">{"−4254"}</span>
            </div>
        </div>
    }
}
