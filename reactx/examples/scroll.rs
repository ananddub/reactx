//! A scroll container driven by the mouse wheel and arrow keys, written with
//! HTML-like `jsx!` syntax.
//!
//! Run with: `cargo run -p reactx --example scroll`
//!
//! `scroll-y` is a plain style value the renderer turns into a signed paint
//! offset; `overflow:hidden` on the same node clips everything outside its
//! border. Content is never re-sliced by the app, so widgets like tables keep
//! their geometry while they scroll.

#![allow(non_snake_case)]

use reactx::prelude::*;

const ROWS: usize = 60;
const VIEW_HEIGHT: u16 = 12;
const INNER_HEIGHT: u16 = VIEW_HEIGHT - 2;
const MAX_OFFSET: u16 = ROWS as u16 - INNER_HEIGHT;

fn App() -> Element {
    let mut offset = use_signal(|| 0u16);

    let clamp = move |value: i32| value.clamp(0, MAX_OFFSET as i32) as u16;
    let first = offset() as usize + 1;
    let last = (offset() + INNER_HEIGHT).min(ROWS as u16) as usize;

    let view_style = format!("title:rows {first}..{last} of {ROWS};scroll-y:{offset}");

    jsx! {
        <ThemeProvider>
            <Container
                class="flex flex-col border border-blue-300 rounded p-1 gap-1 h-full"
                title="Scroll containers"
            >
                <span class="bold">Wheel over the list, or use Up/Down, PgUp/PgDn, Home/End.</span>
                <div class={format!("flex flex-row gap-1 h-[{VIEW_HEIGHT}]")}>
                    <div
                        class="flex flex-col grow h-full overflow-hidden border px-1"
                        style={view_style}
                        onwheel=move |event| {
                            let dy = event.delta().strip_units().y;
                            offset.set(clamp(offset() as i32 + dy as i32 * 2));
                        }
                        onkeydown=move |event| {
                            let step = match event.key() {
                                Key::ArrowDown => 1,
                                Key::ArrowUp => -1,
                                Key::PageDown => 8,
                                Key::PageUp => -8,
                                Key::Home => return offset.set(0),
                                Key::End => return offset.set(MAX_OFFSET),
                                _ => return,
                            };
                            offset.set(clamp(offset() as i32 + step));
                        }
                    >
                        {(0..ROWS).map(|index| jsx! {
                            <div key="{index}" class="flex flex-row gap-1">
                                <span class="text-slate-500">{format!("{:02}", index + 1)}</span>
                                <span
                                    class={if index % 2 == 0 { "text-slate-200" } else { "text-slate-400" }}
                                >
                                    entry {index + 1}{" — scrollable content that never leaves the tree"}
                                </span>
                            </div>
                        })}
                    </div>
                    <Scrollbar position={offset() as usize} content_length={ROWS} />
                </div>
                <div class="flex flex-row gap-2">
                    <Button on_click=move |_| offset.set(0)>Top</Button>
                    <Button on_click=move |_| offset.set(MAX_OFFSET)>Bottom</Button>
                    <span class="dim">{format!("clip: {}..{}", offset(), offset() + INNER_HEIGHT)}</span>
                </div>
            </Container>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
