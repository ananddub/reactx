//! Path pattern matching.

use std::collections::BTreeMap;

use crate::query::{decode_component, split_location, Query};
use crate::route::Route;

/// The result of matching a location against the route table.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RouteMatch {
    /// Index into the route table, or `None` for a 404.
    pub index: Option<usize>,
    pub params: BTreeMap<String, String>,
    pub query: Query,
}

/// Match a single pattern against a path, capturing `:name` segments and a
/// trailing `*` wildcard (exposed as `rest`).
pub fn match_route(pattern: &str, path: &str) -> Option<BTreeMap<String, String>> {
    let pattern_segments = segments(pattern);
    let path_segments = segments(path);
    let mut params = BTreeMap::new();

    for (index, segment) in pattern_segments.iter().enumerate() {
        if *segment == "*" {
            let rest = path_segments
                .get(index..)
                .map(|tail| tail.join("/"))
                .unwrap_or_default();
            params.insert("rest".to_string(), rest);
            return Some(params);
        }
        let path_segment = path_segments.get(index)?;
        if let Some(name) = segment.strip_prefix(':') {
            if !name.is_empty() {
                params.insert(name.to_string(), decode_component(path_segment));
            }
        } else if segment != path_segment {
            return None;
        }
    }

    (pattern_segments.len() == path_segments.len()).then_some(params)
}

fn segments(input: &str) -> Vec<&str> {
    input
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect()
}

pub(crate) fn match_routes(routes: &[Route], location: &str) -> RouteMatch {
    let (path, query) = split_location(location);
    for (index, route) in routes.iter().enumerate() {
        if let Some(params) = match_route(route.path, &path) {
            return RouteMatch {
                index: Some(index),
                params,
                query,
            };
        }
    }
    RouteMatch {
        index: None,
        params: BTreeMap::new(),
        query,
    }
}
