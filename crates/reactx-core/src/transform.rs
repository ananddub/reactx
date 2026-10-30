//! Output transforms registered by the `<Transform>` component.
//!
//! Components and the render loop share one thread, so a thread-local map
//! avoids routing Dioxus callbacks through the node tree. Each registration is
//! replaced on every render, so the latest closure (and its captured state) is
//! the one used by the next frame.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use dioxus_core::prelude::Callback;

/// A text transform: rendered text in, transformed text out.
pub type TransformFn = Rc<dyn Fn(&str) -> String>;

thread_local! {
    static REGISTRY: RefCell<HashMap<String, TransformFn>> = RefCell::new(HashMap::new());
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Allocate an id for a newly mounted `<Transform>`.
pub fn next_id() -> String {
    format!("transform-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed))
}

/// Register (or replace) the transform stored under `id`.
pub fn register(id: &str, callback: Callback<String, String>) {
    let function: TransformFn = Rc::new(move |text: &str| callback.call(text.to_string()));
    REGISTRY.with(|registry| {
        registry.borrow_mut().insert(id.to_string(), function);
    });
}

/// Look up the transform stored under `id`.
pub fn lookup(id: &str) -> Option<TransformFn> {
    REGISTRY.with(|registry| registry.borrow().get(id).cloned())
}

/// Drop a registration when its `<Transform>` unmounts.
pub fn remove(id: &str) {
    REGISTRY.with(|registry| {
        registry.borrow_mut().remove(id);
    });
}

/// Clear every registration. Called when a new app starts on this thread so
/// ids from a previous run cannot leak into the next one.
pub fn clear() {
    REGISTRY.with(|registry| registry.borrow_mut().clear());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_unique() {
        assert_ne!(next_id(), next_id());
    }

    #[test]
    fn lookups_miss_unknown_ids() {
        assert!(lookup("missing").is_none());
        remove("missing");
        clear();
    }
}
