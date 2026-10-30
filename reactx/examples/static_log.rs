//! Lines scroll into the terminal scrollback above a small live viewport.
//!
//! Run with: `cargo run -p reactx --example static_log`

#![allow(non_snake_case)]

use std::time::Duration;

use reactx::prelude::*;

fn App() -> Element {
    let ticks = use_ticker(Duration::from_millis(700));
    let tick = ticks();
    let items: Vec<Line<'static>> = (1..=tick)
        .map(|n| Line::from(format!("log line {n}")))
        .collect();

    jsx! {
        <Box class="flex flex-col border border-blue-300 rounded p-1">
            <Static items={items} />
            <span class="text-green-300 bold">static_log</span>
            <span class="dim">finished lines stay in the scrollback above.</span>
            <span>{format!("lines emitted: {tick}")}</span>
        </Box>
    }
}

#[tokio::main]
async fn main() {
    launch_inline(App, 6).await;
}
