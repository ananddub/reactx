//! A counter driven by mouse clicks and keyboard activation, written with
//! HTML-like `jsx!` syntax.
//!
//! Run with: `cargo run -p reactx --example counter`

#![allow(non_snake_case)]

use reactx::prelude::*;

fn App() -> Element {
    let mut count = use_signal(|| 0);

    jsx! {
        <ThemeProvider>
            <Box class="flex flex-col border border-blue-300 rounded p-1 gap-1">
                <span class="text-green-300 bold">Count: {count}</span>
                <div class="flex flex-row gap-1">
                    <Button on_click=move |_| count += 1>+1</Button>
                    <Button variant="danger" on_click=move |_| count -= 1>-1</Button>
                </div>
                <span class="dim">Tab to focus, Enter or Space to activate, Ctrl+C to quit.</span>
            </Box>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
