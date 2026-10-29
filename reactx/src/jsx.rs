//! Runtime support for `jsx!` interpolation.
//!
//! The macro rewrites `{expression}` children to `jsx_node(expression)`, which
//! accepts text values, signals, elements, options and iterator maps. This is
//! what lets examples use `{if ...}`, `{match ...}` and
//! `{items.iter().map(|item| jsx! { ... })}` inside markup.

use std::fmt::Display;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::{Element, IntoDynNode, ReadOnlySignal, Signal, VNode};

/// Values that can be embedded in `jsx!` children.
pub trait IntoJsxNode {
    fn into_jsx_node(self) -> DynamicNode;
}

/// Embeds a value in `jsx!` children; called by the `jsx!` macro.
#[must_use]
pub fn jsx_node(value: impl IntoJsxNode) -> DynamicNode {
    value.into_jsx_node()
}

/// Elements produced by iterator adapters inside `jsx!`.
pub trait IntoJsxElement {
    fn into_jsx_element(self) -> VNode;
}

impl IntoJsxElement for VNode {
    fn into_jsx_element(self) -> VNode {
        self
    }
}

impl IntoJsxElement for Element {
    fn into_jsx_element(self) -> VNode {
        self.unwrap_or_else(|_| VNode::placeholder())
    }
}

impl IntoJsxElement for &Element {
    fn into_jsx_element(self) -> VNode {
        match self {
            Ok(node) => node.clone(),
            Err(_) => VNode::placeholder(),
        }
    }
}

impl IntoJsxNode for DynamicNode {
    fn into_jsx_node(self) -> DynamicNode {
        self
    }
}

impl IntoJsxNode for VNode {
    fn into_jsx_node(self) -> DynamicNode {
        self.into_dyn_node()
    }
}

impl IntoJsxNode for &VNode {
    fn into_jsx_node(self) -> DynamicNode {
        self.into_dyn_node()
    }
}

impl IntoJsxNode for Element {
    fn into_jsx_node(self) -> DynamicNode {
        self.into_dyn_node()
    }
}

impl IntoJsxNode for &Element {
    fn into_jsx_node(self) -> DynamicNode {
        self.into_dyn_node()
    }
}

impl IntoJsxNode for String {
    fn into_jsx_node(self) -> DynamicNode {
        self.into_dyn_node()
    }
}

impl IntoJsxNode for &str {
    fn into_jsx_node(self) -> DynamicNode {
        self.into_dyn_node()
    }
}

impl IntoJsxNode for &String {
    fn into_jsx_node(self) -> DynamicNode {
        self.as_str().into_dyn_node()
    }
}

impl<T: IntoDynNode> IntoJsxNode for Option<T> {
    fn into_jsx_node(self) -> DynamicNode {
        self.into_dyn_node()
    }
}

impl<T: Display + 'static> IntoJsxNode for Signal<T> {
    fn into_jsx_node(self) -> DynamicNode {
        self.to_string().into_dyn_node()
    }
}

impl<T: Display + 'static> IntoJsxNode for ReadOnlySignal<T> {
    fn into_jsx_node(self) -> DynamicNode {
        self.to_string().into_dyn_node()
    }
}

macro_rules! display_nodes {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IntoJsxNode for $ty {
                fn into_jsx_node(self) -> DynamicNode {
                    self.to_string().into_dyn_node()
                }
            }
        )*
    };
}

display_nodes!(
    bool, char, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
);

impl<I, F, R> IntoJsxNode for std::iter::Map<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> R,
    R: IntoJsxElement,
{
    fn into_jsx_node(self) -> DynamicNode {
        let nodes: Vec<VNode> = self.map(IntoJsxElement::into_jsx_element).collect();
        // Dioxus 0.6 panics when diffing an empty fragment, so render nothing
        // as a placeholder instead.
        if nodes.is_empty() {
            DynamicNode::default()
        } else {
            DynamicNode::Fragment(nodes)
        }
    }
}
