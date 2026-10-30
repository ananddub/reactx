//! Rendering an app to a string without a terminal.

use dioxus_core::{Element, VirtualDom};
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::Terminal;

use crate::TuiRenderer;

/// Render `app` once at `width` × `height` cells and return the screen text.
///
/// Trailing spaces and trailing blank lines are trimmed, so the result is
/// suitable for assertions, logs or snapshot files.
pub fn render_to_string(app: fn() -> Element, width: u16, height: u16) -> String {
    let (width, height) = (width.max(1), height.max(1));
    let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("test terminal");
    let mut dom = VirtualDom::new(app);
    let mut renderer = TuiRenderer::new();
    dom.rebuild(&mut renderer);
    crate::terminal::set_terminal_size(&dom, width, height);
    dom.render_immediate(&mut renderer);
    renderer.paint(&mut terminal).expect("headless paint");
    buffer_text(terminal.backend().buffer())
}

pub(crate) fn buffer_text(buf: &Buffer) -> String {
    let mut lines: Vec<String> = Vec::new();
    for y in 0..buf.area.height {
        let mut line = String::new();
        for x in 0..buf.area.width {
            line.push_str(buf[(x, y)].symbol());
        }
        lines.push(line.trim_end().to_string());
    }
    while lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }
    lines.join("\n")
}
