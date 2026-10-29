//! In-memory routing with `routes!`, `Link`, `use_param` and `navigate`,
//! written with HTML-like `jsx!` syntax.
//!
//! Run with: `cargo run -p reactx --example router`

#![allow(non_snake_case)]

use reactx::prelude::*;

fn Home() -> Element {
    jsx! {
        <ThemeProvider>
            <Container border="rounded" border-color="#89b4fa" padding="1" gap={Some(1)}>
                <Text bold>Home</Text>
                <Link to="/todo/1">Open todo #1</Link>
                <Link to="/todo/2">Open todo #2</Link>
                <Text dim>Click a link or use the two todo routes.</Text>
            </Container>
        </ThemeProvider>
    }
}

fn TodoPage() -> Element {
    let id = use_param("id").unwrap_or_default();
    jsx! {
        <ThemeProvider>
            <Container border="rounded" border-color="#a6e3a1" padding="1" gap={Some(1)}>
                <Text color="#a6e3a1" bold>Todo #{id}</Text>
                <Text>{"This page was matched by /todo/:id."}</Text>
                <Button on_click=move |_| navigate("/")>Back home</Button>
            </Container>
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
