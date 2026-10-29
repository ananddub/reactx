//! A lazily initialized global store.
//!
//! ```no_run
//! use reactx::prelude::*;
//!
//! static COUNT: Store<i32> = Store::new("count", || 0);
//!
//! fn Counter() -> Element {
//!     let count = COUNT.with(|count| *count);
//!     jsx! {
//!         <button
//!             class="px-2 border rounded"
//!             onclick=move |_| COUNT.with_mut(|count| *count += 1)
//!         >
//!             +1 ({count})
//!         </button>
//!     }
//! }
//! ```

use std::marker::PhantomData;

use dioxus::prelude::*;

/// A store created once and shared by the whole app. Reads inside a component
/// subscribe it to later writes.
pub struct Store<T: 'static> {
    constructor: fn() -> T,
    name: &'static str,
    _marker: PhantomData<fn() -> T>,
}

impl<T: 'static> Clone for Store<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Copy for Store<T> {}

impl<T: 'static> Store<T> {
    /// Define a store. `name` must be unique for the type `T` across the app.
    pub const fn new(name: &'static str, constructor: fn() -> T) -> Self {
        Self {
            constructor,
            name,
            _marker: PhantomData,
        }
    }

    fn signal(&self) -> GlobalSignal<T> {
        GlobalSignal::with_name(self.constructor, self.name)
    }
}

impl<T: Clone + 'static> Store<T> {
    /// Read the value, subscribing the current component to changes.
    pub fn with<R>(&self, read: impl FnOnce(&T) -> R) -> R {
        self.signal().with(read)
    }

    /// Mutate the value, rerendering every subscribed component.
    pub fn with_mut<R>(&self, write: impl FnOnce(&mut T) -> R) -> R {
        self.signal().with_mut(write)
    }

    /// Replace the value, rerendering every subscribed component.
    pub fn set(&self, value: T) {
        let mut signal = self.signal();
        signal.set(value);
    }

    /// Snapshot the value without subscribing.
    pub fn peek(&self) -> T {
        let signal = self.signal();
        let guard = signal.peek();
        (*guard).clone()
    }
}
