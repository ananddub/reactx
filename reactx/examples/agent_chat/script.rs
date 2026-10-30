//! Scripted turns and the async driver that plays them.
//!
//! The demo has no model behind it: a tokio task appends transcript items on a
//! timer so the UI visibly thinks, calls a tool and streams an answer. Real
//! events still arrive through `use_input`; submitting a message bumps a
//! generation counter, which cancels the running turn and starts a new one.

mod turns;

use std::time::Duration;

use dioxus::prelude::*;

use turns::{Turn, TURNS, USER_TURN};

#[derive(Clone, PartialEq)]
pub enum Item {
    User(String),
    Head(String),
    Think(String),
    Tool {
        name: String,
        args: String,
        output: Vec<String>,
    },
    Answer(String),
    Hint(String),
}

const MAX_ITEMS: usize = 140;

/// Run turns forever until the generation counter changes. `first` is the
/// message typed by the user, if any; otherwise the canned turns rotate.
pub fn spawn_driver(
    mut items: Signal<Vec<Item>>,
    generation: Signal<u64>,
    mut next: Signal<usize>,
    my_gen: u64,
    first: Option<String>,
) {
    spawn(async move {
        let mut first = first;
        loop {
            if *generation.peek() != my_gen {
                return;
            }
            let (prompt, turn) = match first.take() {
                Some(text) => (text, &USER_TURN),
                None => {
                    let index = *next.peek();
                    next += 1;
                    let turn = &TURNS[index % TURNS.len()];
                    (turn.prompt.to_string(), turn)
                }
            };
            if !play(&mut items, generation, my_gen, prompt, turn).await {
                return;
            }
        }
    });
}

async fn play(
    items: &mut Signal<Vec<Item>>,
    generation: Signal<u64>,
    my_gen: u64,
    prompt: String,
    turn: &Turn,
) -> bool {
    push(items, Item::User(prompt));
    if !pause(generation, my_gen, 650).await {
        return false;
    }
    push(items, Item::Head("Thinking".to_string()));
    for line in turn.reasoning {
        if !pause(generation, my_gen, 550).await {
            return false;
        }
        push(items, Item::Think((*line).to_string()));
    }
    if !pause(generation, my_gen, 350).await {
        return false;
    }
    push(
        items,
        Item::Tool {
            name: turn.tool.0.to_string(),
            args: turn.tool.1.to_string(),
            output: Vec::new(),
        },
    );
    for line in turn.output {
        if !pause(generation, my_gen, 140).await {
            return false;
        }
        if let Some(Item::Tool { output, .. }) = items.write().last_mut() {
            output.push((*line).to_string());
        }
    }
    for line in turn.answer {
        if !pause(generation, my_gen, 340).await {
            return false;
        }
        push(items, Item::Answer((*line).to_string()));
    }
    if !pause(generation, my_gen, 1100).await {
        return false;
    }
    push(
        items,
        Item::Hint("Ready · type a message and press Enter".to_string()),
    );
    pause(generation, my_gen, 2400).await
}

fn push(items: &mut Signal<Vec<Item>>, item: Item) {
    let mut list = items.write();
    list.push(item);
    let overflow = list.len().saturating_sub(MAX_ITEMS);
    if overflow > 0 {
        list.drain(..overflow);
    }
}

/// Sleep in slices so a generation change cancels the turn quickly.
async fn pause(generation: Signal<u64>, my_gen: u64, ms: u64) -> bool {
    let mut slices = (ms / 50).max(1);
    while slices > 0 {
        tokio::time::sleep(Duration::from_millis(50)).await;
        slices -= 1;
        if *generation.peek() != my_gen {
            return false;
        }
    }
    true
}
