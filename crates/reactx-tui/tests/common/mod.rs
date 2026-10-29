//! Shared helpers for headless animation tests.

#![allow(dead_code)]

use dioxus::prelude::*;
use ratatui::backend::TestBackend;
use ratatui::style::Color;
use ratatui::Terminal;
use reactx_core::{KeyModifiers, TuiEvent};
use reactx_tui::events::{handle_event, Flow};
use reactx_tui::TuiRenderer;

pub fn screen(terminal: &Terminal<TestBackend>) -> String {
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

pub fn fg(terminal: &Terminal<TestBackend>, x: u16, y: u16) -> Color {
    terminal.backend().buffer()[(x, y)].fg
}

pub fn setup(app: fn() -> Element) -> (VirtualDom, TuiRenderer, Terminal<TestBackend>) {
    let mut dom = VirtualDom::new(app);
    let mut renderer = TuiRenderer::new();
    dom.rebuild(&mut renderer);
    let mut terminal = Terminal::new(TestBackend::new(30, 6)).unwrap();
    renderer.paint(&mut terminal).unwrap();
    (dom, renderer, terminal)
}

/// Click the first button on screen, settle the DOM and paint one frame.
pub fn click(
    dom: &mut VirtualDom,
    renderer: &mut TuiRenderer,
    terminal: &mut Terminal<TestBackend>,
) {
    let button = renderer
        .tree
        .find_first_tag(renderer.tree.root_id(), "button")
        .expect("button node");
    let rect = renderer.layout[&button];
    let event = TuiEvent::MouseClick {
        x: rect.x,
        y: rect.y,
        modifiers: KeyModifiers::NONE,
    };
    assert_eq!(handle_event(dom, renderer, event), Flow::Continue);
    dom.render_immediate(renderer);
    renderer.paint(terminal).unwrap();
}
