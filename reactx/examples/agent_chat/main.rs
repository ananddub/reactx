//! A self-playing agent chat, styled like a coding CLI.
//!
//! Run with: `cargo run -p reactx --example agent_chat`
//!
//! The transcript streams on its own: thinking bullets, a tool call, output
//! and an answer appear on a timer, while the status bar spins and context
//! usage drifts. Type a message and press Enter to start a fresh turn; Ctrl+O
//! expands collapsed tool output; Ctrl+C quits. Nothing here talks to a model —
//! the point is to show the renderer staying busy.

#![allow(non_snake_case)]

mod script;
mod view;

use reactx::prelude::*;

const MODES: [&str; 4] = ["YOLO", "Plan", "Auto", "Don't Ask"];

fn App() -> Element {
    let mut items = use_signal(|| {
        vec![
            script::Item::Head("ReactX agent · demo session".to_string()),
            script::Item::Answer(
                "A scripted chat demo — thinking, tool calls and the composer all run on live signals."
                    .to_string(),
            ),
            script::Item::Hint(
                "Ctrl+O expand · Up/Down or wheel scroll · Shift+Tab mode · Esc cancel · Ctrl+C quits"
                    .to_string(),
            ),
        ]
    });
    let mut input = use_signal(String::new);
    let mut generation = use_signal(|| 0u64);
    let next_turn = use_signal(|| 0usize);
    let mut expanded = use_signal(|| false);
    let mut mode = use_signal(|| 0usize);

    use_hook(move || script::spawn_driver(items, generation, next_turn, 0, None));
    use_input(move |_, key| {
        let shift_tab =
            key.key == TuiKey::BackTab || (key.key == TuiKey::Tab && key.modifiers.shift);
        if shift_tab {
            let next = (*mode.peek() + 1) % MODES.len();
            mode.set(next);
        } else if key.modifiers.ctrl && key.key == TuiKey::Char('o') {
            let next = !*expanded.peek();
            expanded.set(next);
        } else if key.key == TuiKey::Escape {
            let next = *generation.peek() + 1;
            generation.set(next);
            items.write().push(script::Item::Hint(
                "Cancelled · Enter a message to start a new turn".to_string(),
            ));
        }
    });
    let current_mode = mode();
    let next_mode = MODES[(current_mode + 1) % MODES.len()];

    let (_, terminal_height) = use_terminal_size();
    let rows = terminal_height.max(7).saturating_sub(6);

    jsx! {
        <ThemeProvider>
            <div class="flex flex-col h-full">
                <view::Transcript
                    items={items}
                    expanded={expanded}
                    height={rows}
                    generation={generation}
                />
                <view::LiveStatus items={items} />
                <div class="flex flex-row justify-between px-1">
                    <div class="flex flex-row gap-1">
                        {MODES.iter().enumerate().map(|(index, name)| {
                            let class = if index == current_mode {
                                "text-yellow-300 bold"
                            } else {
                                "text-slate-600"
                            };
                            jsx! { <span key="{index}" class={class}>{*name}</span> }
                        })}
                    </div>
                    <div class="flex flex-row gap-1">
                        <span class="text-slate-500">{format!("Shift+Tab to {next_mode}")}</span>
                        <span class="text-slate-500">{"· 53 skills"}</span>
                    </div>
                </div>
                <view::Rule />
                <view::Composer
                    input={input}
                    on_submit=move |text: String| {
                        let text = text.trim().to_string();
                        if text.is_empty() {
                            return;
                        }
                        input.set(String::new());
                        let next = *generation.peek() + 1;
                        generation.set(next);
                        items.write().clear();
                        script::spawn_driver(items, generation, next_turn, next, Some(text));
                    }
                />
                <view::Rule />
                <view::StatusBar />
            </div>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
