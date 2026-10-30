//! Route table entries and the `routes!` macro.

use dioxus::dioxus_core::{DynamicNode, Template, TemplateNode, VComponent, VNode};
use dioxus::prelude::*;

/// A route target: any component function without props.
pub type RouteFn = fn() -> Element;

/// A template with a single dynamic slot, used to mount a route component.
static ROUTE_TEMPLATE: Template = Template {
    roots: &[TemplateNode::Dynamic { id: 0 }],
    node_paths: &[&[0u8]],
    attr_paths: &[],
};

/// One entry of the route table.
#[derive(Clone)]
pub struct Route {
    /// Path pattern, e.g. `/todo/:id`.
    pub path: &'static str,
    component: VComponent,
}

impl Route {
    /// Build a route from a component function.
    ///
    /// The component is mounted in its own scope, so its hooks are dropped
    /// when navigation leaves the route: `use_drop` runs and animation tickers
    /// stop instead of leaking into the next screen.
    pub fn new<F>(path: &'static str, render: F) -> Self
    where
        F: Fn() -> Element + Clone + 'static,
    {
        Self {
            path,
            component: VComponent::new(render, (), std::any::type_name::<F>()),
        }
    }

    /// An element that mounts the route component in a fresh scope.
    pub(crate) fn element(&self) -> Element {
        Element::Ok(VNode::new(
            None,
            ROUTE_TEMPLATE,
            Box::new([DynamicNode::Component(self.component.clone())]),
            Box::new([]),
        ))
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
