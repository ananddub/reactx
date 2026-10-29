//! A modal overlay with a focus trap, Escape handling and a layered toast,
//! written with HTML-like `jsx!` syntax.
//!
//! Run with: `cargo run -p reactx --example overlays`
//!
//! While the dialog is open the focus ring only contains its buttons and
//! pointer events outside the trap are ignored. `Escape`, the backdrop and the
//! Cancel button all close it; the toast paints above the modal because it
//! declares a higher `layer`.

#![allow(non_snake_case)]

use reactx::prelude::*;

fn App() -> Element {
    let mut open = use_signal(|| false);
    let mut toast = use_signal(String::new);
    let mut deletions = use_signal(|| 0u32);

    jsx! {
        <ThemeProvider>
            <Container
                border="rounded"
                border-color="#f9e2af"
                padding="1"
                gap={Some(1)}
                height="100%"
                title="Overlays"
            >
                <Text bold>A modal stays above the page and owns the keyboard.</Text>
                <Text dim>
                    {"Tab cycles inside the dialog · Escape or a backdrop click closes it · Ctrl+C quits"}
                </Text>
                <div style="direction:row;gap:2">
                    <Button
                        variant="danger"
                        disabled={open()}
                        on_click=move |_| open.set(true)
                    >
                        {"Delete project…"}
                    </Button>
                    <Text dim>{format!("deleted {} time(s)", deletions())}</Text>
                </div>
                {(!toast().is_empty()).then(|| jsx! {
                    <div
                        style="position:absolute;right:1;bottom:1;layer:200;border:rounded;padding:0 1;bg:#313244;color:#a6e3a1;focusable:false"
                        onclick=move |_| toast.set(String::new())
                    >
                        {format!("{} · click to dismiss", toast())}
                    </div>
                })}
                <Modal open={open()} title="Confirm delete" on_close=move |_| open.set(false)>
                    <Text>{"Delete 'staging.toml'? This cannot be undone."}</Text>
                    <div style="direction:row;gap:2;justify:end">
                        <Button
                            variant="danger"
                            on_click=move |_| {
                                open.set(false);
                                deletions += 1;
                                toast.set("Deleted staging.toml".to_string());
                            }
                        >
                            Delete
                        </Button>
                        <Button variant="ghost" on_click=move |_| open.set(false)>Cancel</Button>
                    </div>
                </Modal>
            </Container>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
