//! The `Router` component and navigation links.

use std::rc::Rc;

use dioxus::prelude::*;
use reactx_ui::use_theme;

use crate::context::{navigate, RouterContext};
use crate::matcher::match_routes;
use crate::route::Route;

/// The router itself: provides [`RouterContext`] and renders the matched route.
#[component]
pub fn Router(
    routes: Vec<Route>,
    #[props(default)] initial: String,
    #[props(default)] fallback: Option<Element>,
) -> Element {
    let start = if initial.is_empty() {
        "/".to_string()
    } else {
        initial
    };

    let table = use_hook(|| Rc::new(routes));
    let location = use_signal(|| start.clone());
    let matched = {
        let table = table.clone();
        use_memo(move || match_routes(&table, &location()))
    };
    let history = use_signal(|| vec![start]);
    let cursor = use_signal(|| 0usize);

    use_context_provider(|| RouterContext::new(location, matched, history, cursor));

    let current = matched();
    let content = match current.index {
        Some(index) => table[index].element(),
        None => fallback.unwrap_or_else(|| {
            rsx! {
                span { style: "color:#f38ba8", "404 — no route matched" }
            }
        }),
    };

    rsx! { {content} }
}

/// A clickable link that navigates within the router.
#[component]
pub fn Link(
    to: String,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    let theme = use_theme();
    let style = format!("color:{};underline:true", theme.accent.to_css());
    let target = to.clone();

    rsx! {
        span {
            class: class,
            style: style,
            onclick: move |_| navigate(target.clone()),
            {children}
        }
    }
}
