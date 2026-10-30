use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::matcher::match_routes;
use crate::*;

#[test]
fn matches_static_and_dynamic_routes() {
    assert_eq!(match_route("/", "/"), Some(BTreeMap::new()));
    assert_eq!(match_route("/about", "/about"), Some(BTreeMap::new()));
    assert_eq!(match_route("/about", "/about/team"), None);

    let params = match_route("/todo/:id", "/todo/42").unwrap();
    assert_eq!(params.get("id").map(String::as_str), Some("42"));
}

#[test]
fn wildcard_captures_rest() {
    let params = match_route("/files/*", "/files/a/b.txt").unwrap();
    assert_eq!(params.get("rest").map(String::as_str), Some("a/b.txt"));
}

#[test]
fn parses_query_strings() {
    let query = Query::parse("tab=2&name=hello+world&encoded=%23tag");
    assert_eq!(query.get("tab"), Some("2"));
    assert_eq!(query.get("name"), Some("hello world"));
    assert_eq!(query.get("encoded"), Some("#tag"));
    assert_eq!(query.get_as::<u32>("tab"), Some(2));
}

#[test]
fn splits_location() {
    let (path, query) = split_location("/todo/7?filter=done");
    assert_eq!(path, "/todo/7");
    assert_eq!(query.get("filter"), Some("done"));
}

#[test]
fn first_route_wins() {
    let routes = vec![
        Route::new("/", || rsx! { "home" }),
        Route::new("/:page", || rsx! { "page" }),
    ];
    let matched = match_routes(&routes, "/anything");
    assert_eq!(matched.index, Some(1));
}

#[test]
fn decodes_percent_escapes_leniently() {
    let query = Query::parse("bad=%2x&tail=%&ok=a%20b");
    assert_eq!(query.get("bad"), Some("%2x"));
    assert_eq!(query.get("tail"), Some("%"));
    assert_eq!(query.get("ok"), Some("a b"));

    let params = match_route("/todo/:id", "/todo/a%20b").unwrap();
    assert_eq!(params.get("id").map(String::as_str), Some("a b"));
}

#[test]
fn static_routes_beat_dynamic_ones() {
    let routes = vec![
        Route::new("/:page", || rsx! { "page" }),
        Route::new("/about", || rsx! { "about" }),
        Route::new("/files/*", || rsx! { "files" }),
    ];
    assert_eq!(match_routes(&routes, "/about/").index, Some(1));
    let matched = match_routes(&routes, "/files/a/b/");
    assert_eq!(matched.index, Some(2));
    assert_eq!(matched.params.get("rest").map(String::as_str), Some("a/b"));
}

#[test]
fn unusual_param_names_are_preserved() {
    let params = match_route("/todo/:item-id", "/todo/7").unwrap();
    assert_eq!(params.get("item-id").map(String::as_str), Some("7"));
}

#[test]
fn wildcard_matches_empty_rest() {
    for path in ["/files", "/files/"] {
        let params = match_route("/files/*", path).unwrap();
        assert_eq!(params.get("rest").map(String::as_str), Some(""));
    }
}
