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
            <Container border="rounded" border-color="#89b4fa" padding="1" gap={Some(1)}>
                <Text color="#a6e3a1" bold>Count: {count}</Text>
                <div style="direction:row;gap:1">
                    <Button on_click=move |_| count += 1>+1</Button>
                    <Button variant="danger" on_click=move |_| count -= 1>-1</Button>
                </div>
                <Text dim>Tab to focus, Enter or Space to activate, Ctrl+C to quit.</Text>
            </Container>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
