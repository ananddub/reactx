//! The Dioxus-to-terminal bridge: mutations, flexbox layout, painting and events.

pub mod clipboard;
pub mod events;
pub mod focus;
pub mod layout;
pub mod paint;
pub mod terminal;

mod launch;
mod renderer;
mod widgets;

pub use events::onmousedrag;
pub use focus::FocusManager;
pub use launch::{launch, restore_terminal};
pub use reactx_core::{NodeId, NodeTree, StyleProps, TuiNode};
pub use renderer::{normalize_tag, TuiRenderer};
pub use terminal::{use_terminal_size, TERMINAL_SIZE};

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
