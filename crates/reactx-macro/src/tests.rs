use crate::emit::expand;
use crate::names::sanitize;
use crate::text::clean_text;
use quote::quote;

#[test]
fn text_spacing_is_repaired() {
    assert_eq!(clean_text("Count :"), "Count:");
    assert_eq!(
        clean_text("a , b ; c . d ! e ? f % g / h"),
        "a, b; c. d! e? f% g/h"
    );
    assert_eq!(clean_text("+ 1"), "+1");
    assert_eq!(clean_text("Ctrl + C"), "Ctrl+C");
    assert_eq!(clean_text("- 5"), "-5");
    assert_eq!(clean_text("Open todo # 1"), "Open todo #1");
    assert_eq!(clean_text("open - meteo.com"), "open-meteo.com");
    assert_eq!(clean_text("/ todo / : id ."), "/todo/:id.");
    assert_eq!(clean_text("use / todo /: id route"), "use/todo/:id route");
    assert_eq!(clean_text("First.Second.Third."), "First. Second. Third.");
    assert_eq!(
        clean_text("Feed : Binance.Ctrl + C to quit."),
        "Feed: Binance. Ctrl+C to quit."
    );
    assert_eq!(clean_text("rock - paper - scissors"), "rock-paper-scissors");
    assert_eq!(clean_text("Line one Line two"), "Line one Line two");
    assert_eq!(clean_text("({ x }) [y] -"), "({x}) [y] -");
    assert_eq!(clean_text(" "), " ");
    assert_eq!(clean_text("   "), " ");
    assert_eq!(clean_text("\n    "), "");
}

#[test]
fn names_are_sanitized() {
    assert_eq!(sanitize("border-color"), "border_color");
    assert_eq!(sanitize("data-id"), "data_id");
    assert_eq!(sanitize("tag::name"), "tag_name");
    assert_eq!(sanitize("on_click"), "on_click");
}

#[test]
fn elements_become_rsx() {
    let tokens = expand(quote! {
        <div class="app"><span>Count: {count}</span></div>
    })
    .unwrap()
    .to_string();
    assert!(tokens.contains("rsx !"));
    assert!(tokens.contains("class : \"app\""));
    assert!(tokens.contains("jsx_node"));
}

#[test]
fn mousedrag_becomes_a_custom_listener() {
    let tokens = expand(quote! { <div onmousedrag={move |_| {}} /> })
        .unwrap()
        .to_string();
    assert!(tokens.contains("\"onmousedrag\" : onmousedrag"), "{tokens}");
}

#[test]
fn unsupported_attributes_are_errors() {
    assert!(expand(quote! { <div {spread} /> }).is_err());
    assert!(expand(quote! { <div on_click(event) /> }).is_err());
    assert!(expand(quote! { <{tag} /> }).is_err());
}
