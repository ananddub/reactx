//! In-memory routing with `routes!`, `Link`, `use_param` and `navigate`,
//! written with HTML-like `jsx!` syntax.
//!
//! Run with: `cargo run -p reactx --example router`

#![allow(non_snake_case)]

use reactx::prelude::*;

fn Home() -> Element {
    jsx! {
        <ThemeProvider>
            <Box class="flex flex-col border border-blue-300 rounded p-1 gap-1">
                <span class="bold">Home</span>
                <Link to="/todo/1" class="bold">Open todo #1</Link>
                <Link to="/todo/2" class="bold">Open todo #2</Link>
                <span class="dim">Click a link or use the two todo routes.</span>
            </Box>
        </ThemeProvider>
    }
}

fn TodoPage() -> Element {
    let id = use_param("id").unwrap_or_default();
    jsx! {
        <ThemeProvider>
            <Box class="flex flex-col border border-green-300 rounded p-1 gap-1">
                <span class="text-green-300 bold">Todo #{id}</span>
                <span>{"This page was matched by /todo/:id."}</span>
                <Button on_click=move |_| navigate("/")>Back home</Button>
            </Box>
        </ThemeProvider>
    }
}

fn App() -> Element {
    jsx! {
        <Router routes={routes! {
            "/" => Home,
            "/todo/:id" => TodoPage,
        }} />
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
