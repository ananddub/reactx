//! Terminal events to Dioxus synthetic events.
//!
//! [`handle_event`] routes keys, mouse and form input; the submodules split
//! keyboard editing, mouse hit testing, dispatching and the typed payloads
//! that dioxus-html consumes.

use dioxus_core::VirtualDom;
use dioxus_html::{MouseData, PlatformEventData};
use reactx_core::TuiEvent;

use crate::TuiRenderer;

mod convert;
mod data;
mod dispatch;
pub(crate) mod focus;
mod input;
mod keyboard;
mod keymap;
mod mouse;
mod text;

use convert::install_event_converter;
use keyboard::{handle_key, handle_key_up};
use mouse::{
    handle_click, handle_mouse_button, handle_mouse_down, handle_mouse_move, handle_scroll,
};
use text::paste;
pub use text::TuiClipboardEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Continue,
    Quit,
}

/// Build the `onmousedrag` listener attribute.
///
/// dioxus-html has no drag event, so ReactX provides its own constructor; the
/// `jsx!` macro rewrites `onmousedrag` attributes to call this function.
pub fn onmousedrag<Marker>(
    handler: impl dioxus_core::prelude::SuperInto<
        dioxus_core::prelude::EventHandler<dioxus_core::Event<MouseData>>,
        Marker,
    >,
) -> dioxus_core::AttributeValue {
    let handler = handler.super_into();
    dioxus_core::AttributeValue::listener(move |event: dioxus_core::Event<PlatformEventData>| {
        handler.call(event.map(|event: &PlatformEventData| MouseData::from(event)));
    })
}

/// Route one terminal event through the focus manager and into the VirtualDom.
pub fn handle_event(dom: &VirtualDom, renderer: &mut TuiRenderer, event: TuiEvent) -> Flow {
    install_event_converter();
    renderer.sync_focus_baseline();
    match event {
        TuiEvent::Key(key) => handle_key(dom, renderer, key),
        TuiEvent::KeyUp(key) => {
            handle_key_up(dom, renderer, key);
            Flow::Continue
        }
        TuiEvent::MouseClick { x, y, modifiers } => {
            handle_click(dom, renderer, x, y, modifiers);
            Flow::Continue
        }
        TuiEvent::MouseDown {
            x,
            y,
            button,
            modifiers,
        } => {
            handle_mouse_down(dom, renderer, x, y, button, modifiers);
            Flow::Continue
        }
        TuiEvent::MouseUp {
            x,
            y,
            button,
            modifiers,
        } => {
            handle_mouse_button(dom, renderer, x, y, button, modifiers, "mouseup");
            Flow::Continue
        }
        TuiEvent::MouseMove { x, y, modifiers } => {
            handle_mouse_move(dom, renderer, x, y, modifiers);
            Flow::Continue
        }
        TuiEvent::MouseDrag {
            x,
            y,
            button,
            modifiers,
        } => {
            handle_mouse_button(dom, renderer, x, y, button, modifiers, "mousedrag");
            Flow::Continue
        }
        TuiEvent::Scroll {
            x,
            y,
            delta_x,
            delta_y,
            modifiers,
        } => {
            handle_scroll(dom, renderer, x, y, delta_x, delta_y, modifiers);
            Flow::Continue
        }
        TuiEvent::Paste(text) => {
            crate::runtime::input::dispatch_paste(dom, &text);
            paste(dom, renderer, &text);
            Flow::Continue
        }
        TuiEvent::Resize { width, height } => {
            crate::terminal::set_terminal_size(dom, width, height);
            Flow::Continue
        }
        TuiEvent::FocusGained => {
            crate::terminal::set_terminal_focus(dom, true);
            Flow::Continue
        }
        TuiEvent::FocusLost => {
            crate::terminal::set_terminal_focus(dom, false);
            Flow::Continue
        }
        TuiEvent::Quit => Flow::Quit,
    }
}
