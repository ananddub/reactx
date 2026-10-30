//! Query strings and location splitting.

use std::collections::BTreeMap;
use std::str::FromStr;

use percent_encoding::percent_decode_str;

/// Parsed query string.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Query {
    params: BTreeMap<String, String>,
}

impl Query {
    pub fn parse(input: &str) -> Self {
        let params = form_urlencoded::parse(input.as_bytes())
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect();
        Self { params }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(String::as_str)
    }

    pub fn get_as<T: FromStr>(&self, key: &str) -> Option<T> {
        self.get(key)?.parse().ok()
    }

    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.params
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }
}

impl std::ops::Deref for Query {
    type Target = BTreeMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

/// Split a location into a normalized path and a parsed query.
pub fn split_location(location: &str) -> (String, Query) {
    match location.split_once('?') {
        Some((path, query)) => (normalize_path(path), Query::parse(query)),
        None => (normalize_path(location), Query::default()),
    }
}

fn normalize_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

pub(crate) fn decode_component(input: &str) -> String {
    percent_decode_str(input).decode_utf8_lossy().into_owned()
}
