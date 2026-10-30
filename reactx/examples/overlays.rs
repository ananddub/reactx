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
            <Box
                class="flex flex-col border border-yellow-200 rounded p-1 gap-1 h-full"
                title="Overlays"
            >
                <span class="bold">A modal stays above the page and owns the keyboard.</span>
                <span class="dim">
                    {"Tab cycles inside the dialog · Escape or a backdrop click closes it · Ctrl+C quits"}
                </span>
                <div class="flex flex-row gap-2">
                    <Button
                        variant="danger"
                        disabled={open()}
                        on_click=move |_| open.set(true)
                    >
                        {"Delete project…"}
                    </Button>
                    <span class="dim">{format!("deleted {} time(s)", deletions())}</span>
                </div>
                {(!toast().is_empty()).then(|| jsx! {
                    <div
                        class="absolute right-1 bottom-1 z-[200] border rounded px-1 bg-slate-700 text-green-300"
                        style="focusable:false"
                        onclick=move |_| toast.set(String::new())
                    >
                        {format!("{} · click to dismiss", toast())}
                    </div>
                })}
                <Modal open={open()} title="Confirm delete" on_close=move |_| open.set(false)>
                    <span>{"Delete 'staging.toml'? This cannot be undone."}</span>
                    <div class="flex flex-row gap-2 justify-end">
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
            </Box>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
