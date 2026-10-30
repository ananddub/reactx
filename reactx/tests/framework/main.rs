//! Headless coverage for the framework-level interaction features: scroll
//! containers with clipping, caret-based text editing, and overlays with a
//! focus trap.

#![allow(non_snake_case)]

mod caret;
mod change;
mod clipboard;
mod events;
mod harness;
mod headless;
mod modal;
mod mouse;
mod rapid;
mod scroll;
mod selection;
mod size;
mod string;
mod terminal_focus;
mod textarea;
