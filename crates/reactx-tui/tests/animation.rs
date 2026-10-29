//! Node animation attributes and the tween hook, driven headlessly.

use std::thread::sleep;
use std::time::{Duration, Instant};

use dioxus::prelude::*;
use ratatui::backend::TestBackend;
use ratatui::style::Color;
use ratatui::Terminal;
use reactx_core::{KeyModifiers, TuiEvent};
use reactx_tui::events::{handle_event, Flow};
use reactx_tui::{use_tween, TuiRenderer};

fn screen(terminal: &Terminal<TestBackend>) -> String {
    let buffer = terminal.backend().buffer();
    let mut out = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            out.push_str(buffer[(x, y)].symbol());
        }
        out.push('\n');
    }
    out
}

fn fg(terminal: &Terminal<TestBackend>, x: u16, y: u16) -> Color {
    terminal.backend().buffer()[(x, y)].fg
}

fn setup(app: fn() -> Element) -> (VirtualDom, TuiRenderer, Terminal<TestBackend>) {
    let mut dom = VirtualDom::new(app);
    let mut renderer = TuiRenderer::new();
    dom.rebuild(&mut renderer);
    let mut terminal = Terminal::new(TestBackend::new(30, 6)).unwrap();
    renderer.paint(&mut terminal).unwrap();
    (dom, renderer, terminal)
}

fn app() -> Element {
    rsx! {
        div {
            style: "width:12;height:2;animate:fade;animate-duration:80;animate-easing:linear;animate-color:red",
            span { "hello" }
        }
        div { style: "width:8;height:2", span { "side" } }
    }
}

#[test]
fn node_animation_is_scoped_and_restores_the_screen() {
    let (_dom, mut renderer, mut terminal) = setup(app);
    let normal = screen(&terminal);
    let outside = fg(&terminal, 25, 4);
    assert!(renderer.effects_active(), "mount animation did not start");

    sleep(Duration::from_millis(30));
    renderer.paint(&mut terminal).unwrap();
    assert_ne!(fg(&terminal, 0, 0), outside, "node was not tinted");
    assert_eq!(
        fg(&terminal, 25, 4),
        outside,
        "effect leaked outside its rect"
    );
    assert!(renderer.effects_active(), "effect finished too early");

    sleep(Duration::from_millis(150));
    renderer.paint(&mut terminal).unwrap();
    renderer.paint(&mut terminal).unwrap();
    assert_eq!(fg(&terminal, 0, 0), outside, "cell was not restored");
    assert_eq!(screen(&terminal), normal, "screen changed after the effect");
    assert!(!renderer.effects_active(), "renderer kept animating");
}

fn retrigger_app() -> Element {
    let mut key = use_signal(|| 0u64);
    rsx! {
        div {
            style: "width:12;height:2;animate:fade;animate-duration:60;animate-color:red;animate-key:{key}",
            span { "flash" }
        }
        button { style: "width:10", onclick: move |_| key += 1, "again" }
    }
}

#[test]
fn animate_key_restarts_the_animation() {
    let (mut dom, mut renderer, mut terminal) = setup(retrigger_app);
    sleep(Duration::from_millis(120));
    renderer.paint(&mut terminal).unwrap();
    renderer.paint(&mut terminal).unwrap();
    assert!(!renderer.effects_active(), "first run did not finish");

    let outside = fg(&terminal, 25, 4);
    let button = renderer
        .tree
        .find_first_tag(renderer.tree.root_id(), "button")
        .expect("button node");
    let rect = renderer.layout[&button];
    let click = TuiEvent::MouseClick {
        x: rect.x,
        y: rect.y,
        modifiers: KeyModifiers::NONE,
    };
    assert_eq!(handle_event(&dom, &mut renderer, click), Flow::Continue);
    dom.render_immediate(&mut renderer);
    renderer.paint(&mut terminal).unwrap();

    assert!(renderer.effects_active(), "animate-key did not replay");
    assert_ne!(
        fg(&terminal, 0, 0),
        outside,
        "replayed effect did not tint the node"
    );
}

fn tween_app() -> Element {
    let mut target = use_signal(|| 0.0f64);
    let value = use_tween(target(), Duration::from_millis(80));
    rsx! {
        div { style: "width:16;direction:column",
            span { "{value:.0}" }
            button { onclick: move |_| target.set(50.0), "go" }
        }
    }
}

#[tokio::test]
async fn tween_hook_animates_to_the_target() {
    let (mut dom, mut renderer, mut terminal) = setup(tween_app);
    assert!(screen(&terminal).contains("0"), "{}", screen(&terminal));

    let button = renderer
        .tree
        .find_first_tag(renderer.tree.root_id(), "button")
        .expect("button node");
    let rect = renderer.layout[&button];
    let click = TuiEvent::MouseClick {
        x: rect.x,
        y: rect.y,
        modifiers: KeyModifiers::NONE,
    };
    assert_eq!(handle_event(&dom, &mut renderer, click), Flow::Continue);

    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline {
        tokio::select! {
            _ = dom.wait_for_work() => {
                dom.render_immediate(&mut renderer);
                renderer.paint(&mut terminal).unwrap();
            }
            _ = tokio::time::sleep(Duration::from_millis(20)) => {}
        }
        if screen(&terminal).contains("50") {
            return;
        }
    }
    panic!(
        "tween never reached the target; screen:\n{}",
        screen(&terminal)
    );
}
