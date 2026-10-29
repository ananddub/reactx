//! Router state and the hooks that expose it.

use dioxus::prelude::*;

use crate::matcher::RouteMatch;
use crate::query::Query;

/// The router state exposed to descendants.
#[derive(Clone, Copy)]
pub struct RouterContext {
    location: Signal<String>,
    matched: Memo<RouteMatch>,
    history: Signal<Vec<String>>,
    cursor: Signal<usize>,
}

impl RouterContext {
    pub(crate) fn new(
        location: Signal<String>,
        matched: Memo<RouteMatch>,
        history: Signal<Vec<String>>,
        cursor: Signal<usize>,
    ) -> Self {
        Self {
            location,
            matched,
            history,
            cursor,
        }
    }

    pub fn location(&self) -> String {
        (self.location)()
    }

    pub fn route(&self) -> RouteMatch {
        (self.matched)()
    }

    pub fn navigate(&self, to: impl Into<String>) {
        let to = to.into();
        let mut location = self.location;
        let mut history = self.history;
        let mut cursor = self.cursor;
        let position = (cursor)();
        {
            let mut entries = history.write();
            entries.truncate(position + 1);
            entries.push(to.clone());
        }
        cursor.set(position + 1);
        location.set(to);
    }

    pub fn back(&self) {
        let mut cursor = self.cursor;
        let position = (cursor)();
        if position == 0 {
            return;
        }
        let target = {
            let history = self.history.read();
            history[position - 1].clone()
        };
        cursor.set(position - 1);
        let mut location = self.location;
        location.set(target);
    }

    pub fn forward(&self) {
        let mut cursor = self.cursor;
        let position = (cursor)();
        let target = {
            let history = self.history.read();
            history.get(position + 1).cloned()
        };
        if let Some(target) = target {
            cursor.set(position + 1);
            let mut location = self.location;
            location.set(target);
        }
    }

    pub fn can_go_back(&self) -> bool {
        (self.cursor)() > 0
    }

    pub fn can_go_forward(&self) -> bool {
        let history = self.history.read();
        (self.cursor)() + 1 < history.len()
    }
}

/// Access the nearest router; panics outside a [`Router`].
pub fn use_router() -> RouterContext {
    use_context::<RouterContext>()
}

/// The current location, including the query string.
pub fn use_location() -> String {
    use_router().location()
}

/// A captured `:param` from the matched route.
pub fn use_param(name: &str) -> Option<String> {
    use_router().route().params.get(name).cloned()
}

/// The current query string.
pub fn use_query() -> Query {
    use_router().route().query
}

/// Navigate to `to`. Safe to call inside event handlers.
pub fn navigate(to: impl Into<String>) {
    if let Some(router) = try_consume_context::<RouterContext>() {
        router.navigate(to);
    }
}
