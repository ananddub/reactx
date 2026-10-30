//! Router screen swap mid-animation: where focus lands, and what happens to
//! the old screen's hooks and ticker.
//!
//! Run with: `cargo run -p reactx --example router_focus`

#![allow(non_snake_case)]

use std::sync::atomic::{AtomicUsize, Ordering};

use reactx::prelude::*;

const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

static CLEANUPS: AtomicUsize = AtomicUsize::new(0);

fn spinner(frame: u64) -> String {
    SPINNER[(frame / 2) as usize % SPINNER.len()].to_string()
}

fn ScreenA() -> Element {
    let frame = use_animation_frame()();
    use_drop(|| {
        CLEANUPS.fetch_add(1, Ordering::SeqCst);
    });
    jsx! {
        <div style="direction:column;gap:1">
            <span style="color:#89b4fa;bold:true">"Screen A · animating"</span>
            <div style="direction:row;gap:2">
                <span style="color:#f9e2af">{format!("{} frame {frame}", spinner(frame))}</span>
                <span style="color:#6c7086">"use_animation_frame at ~30 fps"</span>
            </div>
            <div style="direction:row;gap:2">
                <button onclick={move |_| navigate("/b")}>"Next screen"</button>
                <button>"Other action"</button>
            </div>
            <span style="color:#6c7086">"Tab moves focus · Enter opens the focused button"</span>
        </div>
    }
}

fn ScreenB() -> Element {
    let tick = use_animation_frame()();
    let mut presses = use_signal(|| 0u32);
    let cleanup = if CLEANUPS.load(Ordering::SeqCst) > 0 {
        "✓ screen A unmounted — use_drop ran, its ticker stopped"
    } else {
        "… waiting for the old screen to unmount"
    };
    jsx! {
        <div style="direction:column;gap:1">
            <span style="color:#a6e3a1;bold:true">"Screen B · swapped mid-animation"</span>
            <div style="direction:row;gap:2">
                <span style="color:#f9e2af">{format!("{} frame {tick}", spinner(tick))}</span>
                <span style="color:#6c7086">"this screen has its own ticker"</span>
            </div>
            <span style="color:#a6e3a1">{cleanup}</span>
            <div style="direction:row;gap:2">
                <button autofocus="true" onclick={move |_| presses += 1}>"Primary action"</button>
                <button onclick={move |_| navigate("/")}>"Back to A"</button>
            </div>
            <span style="color:#6c7086">
                {format!(
                    "autofocus landed here · primary pressed {}× · Tab stays in this screen",
                    presses()
                )}
            </span>
        </div>
    }
}

fn App() -> Element {
    jsx! {
        <div style="direction:column;height:100%;padding:1;gap:1">
            <span style="color:#cba6f7;bold:true">"ReactX · router screen swap mid-animation"</span>
            <div style="direction:column;border:rounded;padding:1;gap:1;flex_grow:1">
                <Router routes={routes! {
                    "/" => ScreenA,
                    "/b" => ScreenB,
                }} />
            </div>
            <span style="color:#6c7086">"Focus follows the swap: the new screen's autofocus, else its first focusable node."</span>
        </div>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
