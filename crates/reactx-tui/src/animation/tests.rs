//! The ticker drives rerenders through the same loop `launch` uses.

use std::time::{Duration, Instant};

use dioxus::prelude::*;
use ratatui::backend::TestBackend;
use ratatui::Terminal;

use super::use_ticker;
use crate::TuiRenderer;

fn app() -> Element {
    let frame = use_ticker(Duration::from_millis(10));
    rsx! {
        div { "frame {frame}" }
    }
}

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

#[tokio::test]
async fn ticker_rerenders_without_input() {
    let mut dom = VirtualDom::new(app);
    let mut renderer = TuiRenderer::new();
    dom.rebuild(&mut renderer);
    let mut terminal = Terminal::new(TestBackend::new(24, 2)).unwrap();
    renderer.paint(&mut terminal).unwrap();
    assert!(
        screen(&terminal).contains("frame 0"),
        "{}",
        screen(&terminal)
    );

    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline {
        tokio::select! {
            _ = dom.wait_for_work() => {
                dom.render_immediate(&mut renderer);
                renderer.paint(&mut terminal).unwrap();
            }
            _ = tokio::time::sleep(Duration::from_millis(25)) => {}
        }
        if !screen(&terminal).contains("frame 0") {
            return;
        }
    }
    panic!("animation never advanced; screen:\n{}", screen(&terminal));
}
