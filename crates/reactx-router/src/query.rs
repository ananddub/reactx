//! Query strings and location splitting.

use std::collections::BTreeMap;
use std::str::FromStr;

/// Parsed query string.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Query {
    params: BTreeMap<String, String>,
}

impl Query {
    pub fn parse(input: &str) -> Self {
        let mut params = BTreeMap::new();
        for pair in input.split('&') {
            if pair.is_empty() {
                continue;
            }
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            params.insert(decode_component(key), decode_component(value));
        }
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
    let mut out = Vec::with_capacity(input.len());
    let mut bytes = input.bytes().peekable();
    while let Some(byte) = bytes.next() {
        match byte {
            b'+' => out.push(b' '),
            b'%' => {
                let high = bytes.next().and_then(hex_value);
                let low = bytes.next().and_then(hex_value);
                match (high, low) {
                    (Some(high), Some(low)) => out.push((high << 4) | low),
                    _ => out.push(b'%'),
                }
            }
            other => out.push(other),
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
