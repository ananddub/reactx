//! Focus behaviour when the router swaps screens while an animation runs.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use ratatui::style::Modifier;
use reactx::prelude::*;

use crate::harness::Harness;

static A_DROPS: AtomicUsize = AtomicUsize::new(0);
static FOCUS_LOG: Store<String> = Store::new("framework_router_focus_log", String::new);

fn ScreenA() -> Element {
    let frame = use_animation_frame()();
    use_drop(|| {
        A_DROPS.fetch_add(1, Ordering::SeqCst);
    });
    jsx! {
        <div style="direction:column;gap:1">
            <span>{format!("screen a frame {frame}")}</span>
            <button
                onclick={move |_| navigate("/b")}
                onblur={move |_| FOCUS_LOG.with_mut(|log| log.push_str("a-blur;"))}
            >{"Next screen"}</button>
            <button onblur={move |_| FOCUS_LOG.with_mut(|log| log.push_str("a2-blur;"))}>
                {"Other"}
            </button>
        </div>
    }
}

fn ScreenB() -> Element {
    // A hook with a different type than screen A's: shared hook slots would panic.
    let answer = use_signal(|| 7u32);
    jsx! {
        <div style="direction:column;gap:1">
            <span>{format!("screen b {}", answer())}</span>
            <button
                autofocus="true"
                onfocus={move |_| FOCUS_LOG.with_mut(|log| log.push_str("b-focus;"))}
            >{"B preferred"}</button>
            <button>{"B second"}</button>
        </div>
    }
}

fn app() -> Element {
    jsx! {
        <Router routes={routes! {
            "/" => ScreenA,
            "/b" => ScreenB,
        }} />
    }
}

fn focused(harness: &Harness) -> String {
    let id = harness
        .renderer
        .focus
        .focused_id()
        .expect("nothing focused");
    harness.node_text(id)
}

fn reversed(harness: &Harness, text: &str) -> bool {
    let id = harness.button(text);
    let rect = harness.renderer.layout[&id];
    let buffer = harness.terminal.backend().buffer();
    (rect.x..rect.x + rect.width).any(|x| buffer[(x, rect.y)].modifier.contains(Modifier::REVERSED))
}

fn log(harness: &Harness) -> String {
    harness.dom.in_runtime(|| FOCUS_LOG.peek())
}

fn clear_log(harness: &Harness) {
    harness.dom.in_runtime(|| FOCUS_LOG.set(String::new()));
}

/// Drive the virtual DOM until `window` elapses, rendering every update.
/// Returns whether the DOM scheduled any work at all.
async fn pump(harness: &mut Harness, window: Duration) -> bool {
    let deadline = Instant::now() + window;
    let mut worked = false;
    let Harness {
        dom,
        renderer,
        terminal,
    } = harness;
    while Instant::now() < deadline {
        tokio::select! {
            _ = dom.wait_for_work() => {
                worked = true;
                dom.render_immediate(renderer);
                renderer.paint(terminal).unwrap();
            }
            _ = tokio::time::sleep(Duration::from_millis(20)) => {}
        }
    }
    worked
}

#[tokio::test]
async fn router_swap_mid_animation_moves_focus_and_stops_the_old_ticker() {
    A_DROPS.store(0, Ordering::SeqCst);
    let mut harness = Harness::new(app);
    clear_log(&harness);

    assert!(
        harness.screen().contains("screen a frame 0"),
        "{}",
        harness.screen()
    );
    assert_eq!(focused(&harness), "Next screen");
    assert!(reversed(&harness, "Next screen"), "focus not painted");

    // The screen is animating while focus cycles inside it.
    assert!(pump(&mut harness, Duration::from_millis(140)).await);
    assert!(
        !harness.screen().contains("frame 0"),
        "ticker did not advance:\n{}",
        harness.screen()
    );
    harness.key(TuiKey::Tab);
    assert_eq!(focused(&harness), "Other");
    assert!(reversed(&harness, "Other"));
    assert!(!reversed(&harness, "Next screen"));
    harness.key(TuiKey::Tab);
    assert_eq!(focused(&harness), "Next screen");
    assert!(reversed(&harness, "Next screen"));

    // Activate the route switch in the middle of the animation.
    clear_log(&harness);
    harness.key(TuiKey::Enter);
    assert!(
        harness.screen().contains("screen b 7"),
        "{}",
        harness.screen()
    );

    assert_eq!(
        A_DROPS.load(Ordering::SeqCst),
        1,
        "screen A never unmounted"
    );
    assert_eq!(focused(&harness), "B preferred", "autofocus ignored");
    assert!(reversed(&harness, "B preferred"));
    assert_eq!(harness.renderer.focus.order().len(), 2, "old ring leaked");
    // The implicit remap was silent: no blur on the old screen, no focus on the new one.
    assert_eq!(log(&harness), "", "route swap fired synthetic focus events");

    // Tab wraps inside the new screen only, and emits the usual focus event.
    harness.key(TuiKey::Tab);
    assert_eq!(focused(&harness), "B second");
    assert!(reversed(&harness, "B second"));
    assert!(!reversed(&harness, "B preferred"));
    harness.key(TuiKey::Tab);
    assert_eq!(focused(&harness), "B preferred");
    assert_eq!(log(&harness), "b-focus;");

    // The unmounted screen's ticker is cancelled, so it stops scheduling work.
    assert!(
        !pump(&mut harness, Duration::from_millis(140)).await,
        "ticker of the unmounted screen kept driving frames"
    );
}
