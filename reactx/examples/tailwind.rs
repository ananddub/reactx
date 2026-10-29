//! A two-pane dashboard styled entirely with Tailwind-style utility classes.
//!
//! Run with: `cargo run -p reactx --example tailwind`. Resize the terminal to
//! watch the `md:` breakpoint switch the stat row between stacked and
//! side-by-side, hover the cards to see `hover:` variants, and Tab through the
//! sidebar to see `focus:`. The progress bar uses a computed `w-[N%]` class and
//! replays a class-driven `animate-fade` whenever its width changes.
//! 90x24 shows every panel comfortably.

#![allow(non_snake_case)]

use reactx::prelude::*;

const SECTIONS: [&str; 3] = ["Overview", "Metrics", "Logs"];

fn App() -> Element {
    let (width, _) = use_terminal_size();
    let mut section = use_signal(|| 0usize);
    let mut progress = use_signal(|| 35i32);
    let mut online = use_signal(|| true);

    let pct = progress().clamp(0, 100);
    let status_class = if online() {
        "text-emerald-400"
    } else {
        "text-rose-500"
    };
    let status = if online() { "online" } else { "offline" };
    let breakpoint = if width >= 60 { "md" } else { "sm" };
    let bar_class = format!(
        "h-1 bg-emerald-500 w-[{pct}%] animate-fade duration-200 \
         animate-color-emerald-700 animate-key-{pct}"
    );

    jsx! {
        <ThemeProvider>
            <div class="flex flex-col h-full gap-1 p-1 bg-slate-950">
                <div class="flex flex-row items-center gap-2 px-2 bg-slate-800">
                    <span class="text-sky-400 bold">Tailwind playground</span>
                    <span class="grow" />
                    <span class={status_class}>{status}</span>
                    <span class="text-slate-400">{format!("{width} cols / {breakpoint}")}</span>
                </div>

                <div class="flex flex-row gap-1 grow">
                    <div class="flex flex-col w-18 gap-1 p-1 border border-slate-700 rounded bg-slate-900">
                        {SECTIONS.iter().enumerate().map(|(index, name)| {
                            let class = if index == section() {
                                "px-2 bg-sky-600 text-white bold"
                            } else {
                                "px-2 hover:bg-slate-700 focus:bg-sky-700"
                            };
                            rsx! {
                                button {
                                    class: class,
                                    onclick: move |_| section.set(index),
                                    "{name}"
                                }
                            }
                        })}
                        <span class="grow" />
                        <button
                            disabled={true}
                            class="px-2 disabled:dim disabled:bg-rose-900"
                        >
                            Disabled
                        </button>
                    </div>

                    <div class="flex flex-col grow gap-1">
                        <div class="flex flex-col md:flex-row gap-1">
                            <div class="flex flex-col flex-1 px-1 border border-slate-700 rounded bg-slate-900">
                                <span class="text-slate-400">Active section</span>
                                <span class="text-sky-300 bold">{SECTIONS[section()]}</span>
                            </div>
                            <div class="flex flex-col flex-1 px-1 border border-slate-700 rounded bg-slate-900">
                                <span class="text-slate-400">Latency</span>
                                <span class="text-amber-400 bold">42 ms</span>
                            </div>
                            <div class="flex flex-col flex-1 px-1 border border-slate-700 rounded bg-slate-900">
                                <span class="text-slate-400">Uptime</span>
                                <span class="text-emerald-400 bold">99.9%</span>
                            </div>
                        </div>

                        <div class="flex flex-col gap-1 px-1 border border-slate-700 rounded">
                            <div class="flex flex-row items-center gap-2">
                                <span class="w-14 text-slate-300">Progress</span>
                                <div class="grow h-1 bg-slate-800 overflow-hidden">
                                    <div class={bar_class} />
                                </div>
                                <span class="w-4 text-right text-slate-400">{format!("{pct}%")}</span>
                            </div>
                            <div class="flex flex-row gap-1">
                                <button
                                    class="px-2 hover:bg-slate-700"
                                    onclick=move |_| progress.set((progress() - 10).max(0))
                                >
                                    -10
                                </button>
                                <button
                                    class="px-2 hover:bg-slate-700"
                                    onclick=move |_| progress.set((progress() + 10).min(100))
                                >
                                    +10
                                </button>
                                <button
                                    class="px-2 hover:bg-slate-700"
                                    onclick=move |_| online.set(!online())
                                >
                                    Toggle status
                                </button>
                                <Button
                                    variant="ghost"
                                    class="text-center"
                                    on_click=move |_| progress.set(35)
                                >
                                    Reset
                                </Button>
                            </div>
                        </div>

                        <div class="flex flex-row gap-1 grow">
                            <div class="flex flex-col grow items-center justify-center p-1 border border-slate-700 rounded bg-slate-900 hover:bg-slate-800">
                                <span class="text-slate-300">Hover this card</span>
                                <span class="text-slate-500">{"hover:bg-slate-800"}</span>
                            </div>
                            <div class="flex flex-col w-[26] h-[5] items-center justify-center border border-[#f38ba8] rounded text-[#f38ba8]">
                                <span>{"border-[#f38ba8]"}</span>
                                <span>{"w-[26] h-[5]"}</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="px-2 text-slate-500">
                    {"Tab: focus ring · Enter: activate · Ctrl+C: quit"}
                </div>
            </div>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
