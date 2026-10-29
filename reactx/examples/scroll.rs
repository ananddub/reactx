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

    let view_style = format!(
        "direction:column;flex-grow:1;height:100%;overflow:hidden;\
         border:single;title:rows {first}..{last} of {ROWS};padding:0 1;scroll-y:{offset}"
    );

    jsx! {
        <ThemeProvider>
            <Container
                border="rounded"
                border-color="#89b4fa"
                padding="1"
                gap={Some(1)}
                height="100%"
                title="Scroll containers"
            >
                <Text bold>Wheel over the list, or use Up/Down, PgUp/PgDn, Home/End.</Text>
                <div style="direction:row;gap:1;height:{VIEW_HEIGHT}">
                    <div
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
                            <div key="{index}" style="direction:row;gap:1">
                                <span style="color:#6c7086">{format!("{:02}", index + 1)}</span>
                                <span
                                    style={if index % 2 == 0 { "color:#cdd6f4" } else { "color:#a6adc8" }}
                                >
                                    entry {index + 1}{" — scrollable content that never leaves the tree"}
                                </span>
                            </div>
                        })}
                    </div>
                    <Scrollbar position={offset() as usize} content_length={ROWS} />
                </div>
                <div style="direction:row;gap:2">
                    <Button on_click=move |_| offset.set(0)>Top</Button>
                    <Button on_click=move |_| offset.set(MAX_OFFSET)>Bottom</Button>
                    <Text dim>{format!("clip: {}..{}", offset(), offset() + INNER_HEIGHT)}</Text>
                </div>
            </Container>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
