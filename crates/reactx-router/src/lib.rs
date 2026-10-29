//! In-memory routing for ReactX.
//!
//! ```ignore
//! # use reactx::prelude::*;
//! fn App() -> Element {
//!     rsx! {
//!         Router {
//!             routes: routes! {
//!                 "/" => Home,
//!                 "/todo/:id" => Todo,
//!             },
//!         }
//!     }
//! }
//! # fn Home() -> Element { rsx! { "home" } }
//! # fn Todo() -> Element { rsx! { "todo" } }
//! ```
//!
//! `ignore` because the example needs the `reactx` facade crate, which depends
//! on this crate.

mod components;
mod context;
mod matcher;
mod query;
mod route;

pub use components::{Link, Router};
pub use context::{navigate, use_location, use_param, use_query, use_router, RouterContext};
pub use matcher::{match_route, RouteMatch};
pub use query::{split_location, Query};
pub use route::{Route, RouteFn};

#[cfg(test)]
mod tests;
