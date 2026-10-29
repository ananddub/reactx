//! Translation from platform event payloads to dioxus-html typed data.

use std::sync::Once;

use dioxus_html::{
    AnimationData, ClipboardData, CompositionData, DragData, FocusData, FormData,
    HtmlEventConverter, ImageData, KeyboardData, MediaData, MountedData, MouseData,
    PlatformEventData, PointerData, ResizeData, ScrollData, SelectionData, ToggleData, TouchData,
    TransitionData, VisibleData, WheelData,
};

use super::data::{TuiFocusEvent, TuiFormEvent, TuiKeyboardEvent, TuiMouseEvent, TuiWheelEvent};
pub(super) use super::keymap::{map_key, map_modifiers};
use super::text::TuiClipboardEvent;

pub(super) fn install_event_converter() {
    static INSTALL: Once = Once::new();
    INSTALL.call_once(|| dioxus_html::set_event_converter(Box::new(TuiEventConverter)));
}

/// Translates the terminal events we dispatch into dioxus-html's typed data.
struct TuiEventConverter;

impl HtmlEventConverter for TuiEventConverter {
    fn convert_animation_data(&self, _event: &PlatformEventData) -> AnimationData {
        unsupported()
    }

    fn convert_clipboard_data(&self, event: &PlatformEventData) -> ClipboardData {
        ClipboardData::new(
            event
                .downcast::<TuiClipboardEvent>()
                .expect("clipboard event data")
                .clone(),
        )
    }

    fn convert_composition_data(&self, _event: &PlatformEventData) -> CompositionData {
        unsupported()
    }

    fn convert_drag_data(&self, _event: &PlatformEventData) -> DragData {
        unsupported()
    }

    fn convert_focus_data(&self, event: &PlatformEventData) -> FocusData {
        FocusData::new(
            event
                .downcast::<TuiFocusEvent>()
                .expect("focus event data")
                .clone(),
        )
    }

    fn convert_form_data(&self, event: &PlatformEventData) -> FormData {
        FormData::new(
            event
                .downcast::<TuiFormEvent>()
                .expect("form event data")
                .clone(),
        )
    }

    fn convert_image_data(&self, _event: &PlatformEventData) -> ImageData {
        unsupported()
    }

    fn convert_keyboard_data(&self, event: &PlatformEventData) -> KeyboardData {
        KeyboardData::new(
            event
                .downcast::<TuiKeyboardEvent>()
                .expect("keyboard event data")
                .clone(),
        )
    }

    fn convert_media_data(&self, _event: &PlatformEventData) -> MediaData {
        unsupported()
    }

    fn convert_mounted_data(&self, _event: &PlatformEventData) -> MountedData {
        unsupported()
    }

    fn convert_mouse_data(&self, event: &PlatformEventData) -> MouseData {
        MouseData::new(
            event
                .downcast::<TuiMouseEvent>()
                .expect("mouse event data")
                .clone(),
        )
    }

    fn convert_pointer_data(&self, _event: &PlatformEventData) -> PointerData {
        unsupported()
    }

    fn convert_resize_data(&self, _event: &PlatformEventData) -> ResizeData {
        unsupported()
    }

    fn convert_scroll_data(&self, _event: &PlatformEventData) -> ScrollData {
        unsupported()
    }

    fn convert_selection_data(&self, _event: &PlatformEventData) -> SelectionData {
        unsupported()
    }

    fn convert_toggle_data(&self, _event: &PlatformEventData) -> ToggleData {
        unsupported()
    }

    fn convert_touch_data(&self, _event: &PlatformEventData) -> TouchData {
        unsupported()
    }

    fn convert_transition_data(&self, _event: &PlatformEventData) -> TransitionData {
        unsupported()
    }

    fn convert_visible_data(&self, _event: &PlatformEventData) -> VisibleData {
        unsupported()
    }

    fn convert_wheel_data(&self, event: &PlatformEventData) -> WheelData {
        WheelData::new(
            event
                .downcast::<TuiWheelEvent>()
                .expect("wheel event data")
                .clone(),
        )
    }
}

fn unsupported<T>() -> T {
    panic!("reactx-tui does not raise this terminal event kind")
}
