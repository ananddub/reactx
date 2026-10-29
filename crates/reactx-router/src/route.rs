//! Route table entries and the `routes!` macro.

use dioxus::prelude::*;

/// A route target: any component function without props.
pub type RouteFn = fn() -> Element;

/// One entry of the route table.
#[derive(Clone, Copy)]
pub struct Route {
    /// Path pattern, e.g. `/todo/:id`.
    pub path: &'static str,
    pub render: RouteFn,
}

impl Route {
    pub const fn new(path: &'static str, render: RouteFn) -> Self {
        Self { path, render }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Route {}

/// Build a route table: `routes! { "/" => Home, "/todo/:id" => Todo }`.
#[macro_export]
macro_rules! routes {
    ($($path:literal => $component:path),+ $(,)?) => {
        ::std::vec![$($crate::Route::new($path, $component)),+]
    };
}
