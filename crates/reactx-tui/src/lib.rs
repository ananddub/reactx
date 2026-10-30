//! The Dioxus-to-terminal bridge: mutations, flexbox layout, painting and events.

pub mod animation;
pub mod clipboard;
pub mod effects;
pub mod events;
pub mod focus;
pub mod layout;
pub mod paint;
pub mod runtime;
pub mod terminal;

mod app;
mod config;
mod launch;
mod renderer;
mod string;
mod widgets;

pub use animation::{
    use_animation_frame, use_color_tween, use_spring, use_ticker, use_tween, Spring, Tween,
};
pub use app::{use_app, AppHandle};
pub use config::{LaunchOptions, RenderCallback, RenderMetrics};
pub use events::{onmousedrag, TuiClipboardEvent};
pub use focus::FocusManager;
pub use launch::{
    install_panic_hook, launch, launch_inline, launch_inline_with, launch_with, restore_terminal,
};
pub use reactx_core::{NodeId, NodeTree, StyleProps, TuiNode};
pub use renderer::{normalize_tag, TuiRenderer};
pub use runtime::{
    measure_element, use_box_metrics, use_cursor, use_focus, use_focus_manager, use_input,
    use_input_when, use_is_screen_reader_enabled, use_paste, use_paste_when, use_stderr, use_stdin,
    use_stdout, BoxMetrics, CursorHandle, FocusManagerHandle, StderrHandle, StdinHandle,
    StdoutHandle,
};
pub use string::render_to_string;
pub use terminal::{
    use_terminal_focus, use_terminal_size, use_window_size, TERMINAL_FOCUS, TERMINAL_SIZE,
};

use std::io;

use ratatui::backend::Backend;
use ratatui::Terminal;

/// A convenience helper for tests and embedders that want to drive a VirtualDom manually.
pub fn rebuild<T: Backend>(
    dom: &mut dioxus_core::VirtualDom,
    renderer: &mut TuiRenderer,
    terminal: &mut Terminal<T>,
) -> io::Result<()> {
    dom.rebuild(renderer);
    renderer.paint(terminal)
}
