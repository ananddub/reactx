//! Scrolling transcript plus its item rows.
//!
//! The composer and status bar live in `view/chrome.rs` so the 80 ms status
//! ticker and the streaming transcript do not force the composer to rerender
//! while you type.

use reactx::prelude::*;

use crate::script::Item;

mod chrome;
mod live;

pub use chrome::{Composer, Rule, StatusBar};
pub use live::LiveStatus;

/// Transcript pinned to the bottom until you scroll up. The mouse wheel and
/// `Up`/`Down` move one row, `PageUp`/`PageDown` one screen, `End` re-attaches
/// to the stream. A new turn starts back at the bottom.
#[component]
pub fn Transcript(
    items: Signal<Vec<Item>>,
    expanded: Signal<bool>,
    height: u16,
    generation: Signal<u64>,
) -> Element {
    let list = items();
    let is_expanded = expanded();
    let total = row_count(&list, is_expanded);
    let limit = total.saturating_sub(height as usize);
    let mut follow = use_signal(|| true);
    let mut top = use_signal(|| 0usize);
    let page = (height as usize).saturating_sub(2).max(1);
    use_effect(move || {
        generation();
        follow.set(true);
        top.set(0);
    });
    use_input(move |_, key| match key.key {
        TuiKey::Up => scroll_by(follow, top, limit, -1),
        TuiKey::Down => scroll_by(follow, top, limit, 1),
        TuiKey::PageUp => scroll_by(follow, top, limit, -(page as i32)),
        TuiKey::PageDown => scroll_by(follow, top, limit, page as i32),
        TuiKey::Home => scroll_by(follow, top, limit, -(limit as i32)),
        TuiKey::End => scroll_by(follow, top, limit, limit as i32),
        _ => {}
    });
    let offset = if follow() { limit } else { top().min(limit) };
    let gap = (height as usize).saturating_sub(total);
    let class = format!("flex flex-col h-[{height}] overflow-hidden");
    let style = format!("scroll-y:{offset}");
    jsx! {
        <div
            class={class}
            style={style}
            onwheel=move |event| {
                let delta = event.delta().strip_units().y;
                scroll_by(follow, top, limit, delta as i32 * 2);
            }
        >
            {(gap > 0).then(|| jsx! { <div key="gap" class={format!("h-[{gap}]")} /> })}
            {list.iter().enumerate().map(|(index, item)| row(index, item, is_expanded))}
        </div>
    }
}

fn scroll_by(mut follow: Signal<bool>, mut top: Signal<usize>, limit: usize, delta: i32) {
    let current = if *follow.peek() { limit } else { *top.peek() };
    let next = (current as i32 + delta).clamp(0, limit as i32) as usize;
    if next >= limit {
        follow.set(true);
    } else {
        follow.set(false);
        top.set(next);
    }
}

fn row_count(items: &[Item], expanded: bool) -> usize {
    items
        .iter()
        .map(|item| match item {
            Item::Tool { output, .. } => {
                let shown = if expanded {
                    output.len()
                } else {
                    output.len().min(2)
                };
                1 + shown + usize::from(!expanded && output.len() > shown)
            }
            _ => 1,
        })
        .sum()
}

fn row(index: usize, item: &Item, expanded: bool) -> Element {
    match item {
        Item::User(text) => jsx! {
            <div key="{index}" class="flex flex-row gap-1 px-1">
                <span class="text-green-400 bold">{"❯"}</span>
                <span class="text-slate-100 bold">{text.clone()}</span>
            </div>
        },
        Item::Head(text) => jsx! {
            <div key="{index}" class="flex flex-row gap-1 px-1">
                <span class="text-violet-300">{"✻"}</span>
                <span class="text-slate-300 bold">{text.clone()}</span>
            </div>
        },
        Item::Think(text) => jsx! {
            <div key="{index}" class="flex flex-row gap-1 px-2">
                <span class="text-slate-600">{"•"}</span>
                <span class="text-slate-500">{text.clone()}</span>
            </div>
        },
        Item::Tool { name, args, output } => tool(index, name, args, output, expanded),
        Item::Answer(text) => jsx! {
            <div key="{index}" class="flex flex-row px-2">
                <span class="text-slate-200">{text.clone()}</span>
            </div>
        },
        Item::Hint(text) => jsx! {
            <div key="{index}" class="flex flex-row px-2">
                <span class="text-slate-600 italic">{text.clone()}</span>
            </div>
        },
    }
}

fn tool(index: usize, name: &str, args: &str, output: &[String], expanded: bool) -> Element {
    let shown = if expanded {
        output.len()
    } else {
        output.len().min(2)
    };
    let hidden = output.len() - shown;
    jsx! {
        <>
            <div key="{index}" class="flex flex-row gap-1 px-1">
                <span class="text-slate-600">{"•"}</span>
                <span class="text-teal-300 bold">{name.to_string()}</span>
                <span class="text-slate-400">{args.to_string()}</span>
            </div>
            {output.iter().take(shown).enumerate().map(|(line, text)| jsx! {
                <div key="{index}-{line}" class="flex flex-row px-3">
                    <span class={output_class(text)}>{text.clone()}</span>
                </div>
            })}
            {(!expanded && hidden > 0).then(|| jsx! {
                <div key="{index}-more" class="flex flex-row px-3">
                    <span class="text-slate-600 italic">
                        {format!("… +{hidden} lines (Ctrl+O to expand)")}
                    </span>
                </div>
            })}
        </>
    }
}

fn output_class(line: &str) -> &'static str {
    if line.contains("ok") || line.contains("Published") {
        "text-green-400"
    } else if line.contains("FAILED") || line.contains("error") {
        "text-rose-400"
    } else if line.starts_with("  ") {
        "text-slate-500"
    } else {
        "text-slate-400"
    }
}
