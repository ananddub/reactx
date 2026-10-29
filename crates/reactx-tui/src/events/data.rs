//! Platform payloads and their dioxus-html trait implementations.

use std::any::Any;

use dioxus_html::geometry::{ClientPoint, ElementPoint, PagePoint, ScreenPoint, WheelDelta};
use dioxus_html::input_data::keyboard_types::{Code, Key, Location, Modifiers};
use dioxus_html::input_data::{MouseButton, MouseButtonSet};
use dioxus_html::point_interaction::{
    InteractionElementOffset, InteractionLocation, ModifiersInteraction, PointerInteraction,
};
use dioxus_html::{
    HasFileData, HasFocusData, HasFormData, HasKeyboardData, HasMouseData, HasWheelData,
};
use reactx_core::MouseButtonKind;

#[derive(Clone)]
pub(super) struct TuiFormEvent {
    pub(super) value: String,
}

impl HasFileData for TuiFormEvent {}

impl HasFormData for TuiFormEvent {
    fn value(&self) -> String {
        self.value.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub(super) struct TuiFocusEvent;

impl HasFocusData for TuiFocusEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub(super) struct TuiMouseEvent {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) modifiers: Modifiers,
    pub(super) button: MouseButtonKind,
}

impl InteractionLocation for TuiMouseEvent {
    fn client_coordinates(&self) -> ClientPoint {
        ClientPoint::new(self.x, self.y)
    }

    fn screen_coordinates(&self) -> ScreenPoint {
        ScreenPoint::new(self.x, self.y)
    }

    fn page_coordinates(&self) -> PagePoint {
        PagePoint::new(self.x, self.y)
    }
}

impl InteractionElementOffset for TuiMouseEvent {
    fn element_coordinates(&self) -> ElementPoint {
        ElementPoint::new(self.x, self.y)
    }
}

impl ModifiersInteraction for TuiMouseEvent {
    fn modifiers(&self) -> Modifiers {
        self.modifiers
    }
}

impl PointerInteraction for TuiMouseEvent {
    fn trigger_button(&self) -> Option<MouseButton> {
        Some(match self.button {
            MouseButtonKind::Left => MouseButton::Primary,
            MouseButtonKind::Right => MouseButton::Secondary,
            MouseButtonKind::Middle => MouseButton::Auxiliary,
        })
    }

    fn held_buttons(&self) -> MouseButtonSet {
        MouseButtonSet::default()
    }
}

impl HasMouseData for TuiMouseEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub(super) struct TuiWheelEvent {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) delta_x: f64,
    pub(super) delta_y: f64,
}

impl InteractionLocation for TuiWheelEvent {
    fn client_coordinates(&self) -> ClientPoint {
        ClientPoint::new(self.x, self.y)
    }

    fn screen_coordinates(&self) -> ScreenPoint {
        ScreenPoint::new(self.x, self.y)
    }

    fn page_coordinates(&self) -> PagePoint {
        PagePoint::new(self.x, self.y)
    }
}

impl InteractionElementOffset for TuiWheelEvent {
    fn element_coordinates(&self) -> ElementPoint {
        ElementPoint::new(self.x, self.y)
    }
}

impl ModifiersInteraction for TuiWheelEvent {
    fn modifiers(&self) -> Modifiers {
        Modifiers::empty()
    }
}

impl PointerInteraction for TuiWheelEvent {
    fn trigger_button(&self) -> Option<MouseButton> {
        None
    }

    fn held_buttons(&self) -> MouseButtonSet {
        MouseButtonSet::default()
    }
}

impl HasMouseData for TuiWheelEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HasWheelData for TuiWheelEvent {
    fn delta(&self) -> WheelDelta {
        WheelDelta::lines(self.delta_x, self.delta_y, 0.0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub(super) struct TuiKeyboardEvent {
    pub(super) key: Key,
    pub(super) code: Code,
    pub(super) modifiers: Modifiers,
}

impl ModifiersInteraction for TuiKeyboardEvent {
    fn modifiers(&self) -> Modifiers {
        self.modifiers
    }
}

impl HasKeyboardData for TuiKeyboardEvent {
    fn key(&self) -> Key {
        self.key.clone()
    }

    fn code(&self) -> Code {
        self.code
    }

    fn location(&self) -> Location {
        Location::Standard
    }

    fn is_auto_repeating(&self) -> bool {
        false
    }

    fn is_composing(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
