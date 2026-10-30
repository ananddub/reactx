//! Path pattern matching, backed by `matchit`.

use std::collections::BTreeMap;

use matchit::Router;

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

#[derive(Clone, Copy)]
struct Endpoint {
    index: usize,
    /// Matched through the prefix of a `/*` route, so `rest` is empty.
    empty_rest: bool,
}

/// Match a single pattern against a path, capturing `:name` segments and a
/// trailing `*` wildcard (exposed as `rest`).
pub fn match_route(pattern: &str, path: &str) -> Option<BTreeMap<String, String>> {
    let (pattern, names) = translate(pattern);
    let mut router = Router::new();
    insert_patterns(
        &mut router,
        &pattern,
        Endpoint {
            index: 0,
            empty_rest: false,
        },
    );
    let path = normalize(path);
    let matched = router.at(&path).ok()?;
    let mut params = collect_params(matched.params.iter(), &names);
    if matched.value.empty_rest {
        params.insert("rest".to_string(), String::new());
    }
    Some(params)
}

pub(crate) fn match_routes(routes: &[Route], location: &str) -> RouteMatch {
    let (path, query) = split_location(location);
    let mut router = Router::new();
    let mut names = Vec::with_capacity(routes.len());
    for (index, route) in routes.iter().enumerate() {
        let (pattern, renames) = translate(route.path);
        names.push(renames);
        insert_patterns(
            &mut router,
            &pattern,
            Endpoint {
                index,
                empty_rest: false,
            },
        );
    }
    match router.at(&normalize(&path)) {
        Ok(matched) => {
            let endpoint = *matched.value;
            let mut params = collect_params(matched.params.iter(), &names[endpoint.index]);
            if endpoint.empty_rest {
                params.insert("rest".to_string(), String::new());
            }
            RouteMatch {
                index: Some(endpoint.index),
                params,
                query,
            }
        }
        Err(_) => RouteMatch {
            index: None,
            params: BTreeMap::new(),
            query,
        },
    }
}

/// Register a translated pattern. A trailing catch-all also gets its prefix
/// registered so `/files` and `/files/` match `/files/*` with an empty `rest`.
fn insert_patterns(router: &mut Router<Endpoint>, pattern: &str, endpoint: Endpoint) {
    let _ = router.insert(pattern.to_string(), endpoint);
    if let Some(prefix) = pattern.strip_suffix("/{*rest}") {
        let prefix = if prefix.is_empty() { "/" } else { prefix };
        let _ = router.insert(
            prefix.to_string(),
            Endpoint {
                empty_rest: true,
                ..endpoint
            },
        );
    }
}

/// Rewrite `:name` / `*` patterns into matchit's `{name}` / `{*rest}` syntax.
/// Returns the rewritten pattern and any parameter renames applied to satisfy
/// matchit's identifier rules.
fn translate(pattern: &str) -> (String, Vec<(String, String)>) {
    let mut out = String::new();
    let mut names = Vec::new();
    for segment in pattern.split('/').filter(|segment| !segment.is_empty()) {
        out.push('/');
        if segment == "*" {
            out.push_str("{*rest}");
        } else if let Some(name) = segment.strip_prefix(':') {
            let original = if name.is_empty() { "param" } else { name };
            let key = sanitize(original);
            if key != original {
                names.push((key.clone(), original.to_string()));
            }
            out.push('{');
            out.push_str(&key);
            out.push('}');
        } else {
            out.push_str(segment);
        }
    }
    if out.is_empty() {
        out.push('/');
    }
    (out, names)
}

fn sanitize(name: &str) -> String {
    let mut out: String = name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect();
    if !out
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_')
    {
        out.insert(0, '_');
    }
    out
}

fn normalize(path: &str) -> String {
    let mut out = String::new();
    for segment in path.split('/').filter(|segment| !segment.is_empty()) {
        out.push('/');
        out.push_str(segment);
    }
    if out.is_empty() {
        out.push('/');
    }
    out
}

fn collect_params<'a>(
    params: impl Iterator<Item = (&'a str, &'a str)>,
    names: &[(String, String)],
) -> BTreeMap<String, String> {
    params
        .map(|(key, value)| {
            let key = names
                .iter()
                .find(|(renamed, _)| renamed == key)
                .map(|(_, original)| original.clone())
                .unwrap_or_else(|| key.to_string());
            (key, decode_component(value))
        })
        .collect()
}
