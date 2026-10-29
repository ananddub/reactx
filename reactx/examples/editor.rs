//! A small editor built from controlled inputs: caret movement, deletion,
//! submit, a scrollable draft list and a live derived preview, written with
//! HTML-like `jsx!` syntax.
//!
//! Run with: `cargo run -p reactx --example editor`
//!
//! Drafts land in a fixed-height box with `overflow:hidden` and `scroll-y`:
//! the wheel scrolls it, new drafts follow to the bottom, and clicking a draft
//! reopens it in the Title field (it returns to the list when saved again).
//!
//! The caret is part of the renderer state, not the value: `Left`/`Right`/
//! `Home`/`End` move it, `Backspace`/`Delete` edit around it, and `oninput`
//! only fires when the string actually changes.

#![allow(non_snake_case)]

use reactx::prelude::*;

const DRAFTS_HEIGHT: u16 = 6;
const DRAFTS_ROWS: usize = DRAFTS_HEIGHT as usize - 2;

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;
    for ch in title.chars() {
        if ch.is_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            pending_dash = false;
            slug.extend(ch.to_lowercase());
        } else {
            pending_dash = true;
        }
    }
    slug
}

fn App() -> Element {
    let mut title = use_signal(String::new);
    let mut tags = use_signal(|| "rust, tui".to_string());
    let mut drafts = use_signal(Vec::<String>::new);
    let mut offset = use_signal(|| 0u16);

    let title_value = title();
    let tag_value = tags();
    let draft_list = drafts();
    let slug = slugify(&title_value);
    let chars = title_value.chars().count();
    let tag_count = tag_value
        .split(',')
        .filter(|tag| !tag.trim().is_empty())
        .count();
    let drafts_style = format!("scroll-y:{offset}");

    jsx! {
        <ThemeProvider>
            <Container
                class="flex flex-col border border-green-300 rounded p-1 gap-1 h-full"
                title="Text editing"
            >
                <Input
                    label="Title"
                    value={title_value}
                    placeholder="hello terminal"
                    on_input=move |next| title.set(next)
                    on_submit=move |next: String| {
                        let next = next.trim().to_string();
                        if !next.is_empty() {
                            drafts.write().push(next);
                            title.set(String::new());
                            offset.set(drafts().len().saturating_sub(DRAFTS_ROWS) as u16);
                        }
                    }
                />
                <Input
                    label="Tags (comma separated)"
                    value={tag_value}
                    placeholder="rust, tui, dioxus"
                    on_input=move |next| tags.set(next)
                />
                <div class="flex flex-row gap-2">
                    <span class="dim">slug: {slug}</span>
                    <span class="dim">{chars} chars</span>
                    <span class="dim">{tag_count} tags</span>
                    <span class="dim">{draft_list.len()} drafts</span>
                </div>
                <div class={format!("flex flex-row h-[{DRAFTS_HEIGHT}]")}>
                    <div
                        class="flex flex-col grow h-full overflow-hidden border px-1"
                        style={drafts_style}
                        onwheel=move |event| {
                            let delta = event.delta().strip_units().y as i32 * 2;
                            let max = drafts().len().saturating_sub(DRAFTS_ROWS) as i32;
                            offset.set((offset() as i32 + delta).clamp(0, max) as u16);
                        }
                    >
                        {if draft_list.is_empty() {
                            jsx! { <span class="dim">"Type a title and press Enter."</span> }
                        } else {
                            jsx! {
                                <>
                                    {draft_list.iter().enumerate().map(|(index, draft)| {
                                        let reopen = draft.clone();
                                        let label = draft.clone();
                                        jsx! {
                                            <div
                                                key="{index}"
                                                class="flex flex-row gap-1"
                                                style="focusable:false"
                                                onclick=move |_| {
                                                    if let Some(position) = drafts()
                                                        .iter()
                                                        .position(|entry| entry == &reopen)
                                                    {
                                                        drafts.write().remove(position);
                                                    }
                                                    title.set(reopen.clone());
                                                    let max = drafts().len().saturating_sub(DRAFTS_ROWS) as u16;
                                                    offset.set(offset().min(max));
                                                }
                                            >
                                                <span class="text-slate-500">{"↺"}</span>
                                                <span class="text-green-300">{label}</span>
                                            </div>
                                        }
                                    })}
                                </>
                            }
                        }}
                    </div>
                    {(!draft_list.is_empty()).then(|| jsx! {
                        <Scrollbar position={offset() as usize} content_length={draft_list.len()} />
                    })}
                </div>
                <span class="dim">
                    {"Enter saves · click a draft to reopen it · wheel scrolls the list · Ctrl+C quits"}
                </span>
            </Container>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
