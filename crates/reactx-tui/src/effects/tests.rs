//! Screen effects run after node painting and repaint the screen once done.

use std::thread::sleep;
use std::time::Duration;

use dioxus::prelude::*;
use ratatui::backend::TestBackend;
use ratatui::Terminal;

use super::{fx, screen_effects, Color, Interpolation};
use crate::TuiRenderer;

fn app() -> Element {
    rsx! {
        div { style: "color:#a6e3a1", "hello" }
    }
}

fn fg(terminal: &Terminal<TestBackend>, x: u16, y: u16) -> Color {
    terminal.backend().buffer()[(x, y)].fg
}

#[test]
fn queued_effect_tints_the_frame_and_restores_it_when_done() {
    let mut dom = VirtualDom::new(app);
    let mut renderer = TuiRenderer::new();
    dom.rebuild(&mut renderer);
    let mut terminal = Terminal::new(TestBackend::new(12, 2)).unwrap();
    renderer.paint(&mut terminal).unwrap();
    let normal = fg(&terminal, 0, 0);
    assert!(!renderer.effects_active());

    screen_effects().play(fx::fade_from_fg(Color::Red, (60u32, Interpolation::Linear)));
    assert!(renderer.effects_active(), "queued effect not reported");

    renderer.paint(&mut terminal).unwrap();
    assert_ne!(
        fg(&terminal, 0, 0),
        normal,
        "effect did not touch the frame"
    );
    assert!(renderer.effects_active(), "effect finished too early");

    // A step longer than the 60 ms timer finishes the effect and restores the
    // original colours; the renderer still schedules one clean frame.
    sleep(Duration::from_millis(120));
    renderer.paint(&mut terminal).unwrap();
    assert_eq!(fg(&terminal, 0, 0), normal, "text colour was not restored");
    assert!(renderer.effects_active(), "no follow-up frame scheduled");

    renderer.paint(&mut terminal).unwrap();
    assert!(!renderer.effects_active(), "renderer kept animating");
}
