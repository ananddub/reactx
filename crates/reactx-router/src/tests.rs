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
