//! Identifier sanitizing and spans.

use proc_macro2::Span;
use rstml::node::{NodeName, NodeNameFragment};
use syn::Ident;

pub(crate) fn sanitize(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut underscore = false;
    for ch in name.chars() {
        if ch == '_' || ch.is_alphanumeric() {
            out.push(ch);
            underscore = ch == '_';
        } else if matches!(ch, '-' | '.' | ':') && !underscore && !out.is_empty() {
            out.push('_');
            underscore = true;
        }
    }
    out
}

pub(crate) fn make_ident(name: &str, span: Span) -> syn::Result<Ident> {
    let cleaned = sanitize(name);
    let mut chars = cleaned.chars();
    let valid = matches!(chars.next(), Some(first) if first == '_' || first.is_alphabetic())
        && chars.all(|ch| ch == '_' || ch.is_alphanumeric());

    if !valid || cleaned == "_" {
        return Err(syn::Error::new(
            span,
            format!("`{name}` cannot be translated to a Rust identifier"),
        ));
    }
    if matches!(cleaned.as_str(), "self" | "Self" | "super" | "crate") {
        return Err(syn::Error::new(
            span,
            format!("`{name}` is reserved and cannot be used as an attribute name"),
        ));
    }
    if is_keyword(&cleaned) {
        Ok(Ident::new_raw(&cleaned, span))
    } else {
        Ok(Ident::new(&cleaned, span))
    }
}

fn is_keyword(name: &str) -> bool {
    matches!(
        name,
        "abstract"
            | "as"
            | "async"
            | "await"
            | "become"
            | "box"
            | "break"
            | "const"
            | "continue"
            | "do"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "final"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "macro"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "override"
            | "priv"
            | "pub"
            | "ref"
            | "return"
            | "static"
            | "struct"
            | "trait"
            | "true"
            | "try"
            | "type"
            | "typeof"
            | "unsafe"
            | "unsized"
            | "use"
            | "virtual"
            | "where"
            | "while"
            | "yield"
    )
}

pub(crate) fn name_span(name: &NodeName) -> Span {
    match name {
        NodeName::Path(path) => path
            .path
            .segments
            .first()
            .map_or_else(Span::call_site, |segment| segment.ident.span()),
        NodeName::Punctuated(fragments) => {
            fragments
                .first()
                .map_or_else(Span::call_site, |fragment| match fragment {
                    NodeNameFragment::Ident(ident) => ident.span(),
                    NodeNameFragment::Literal(literal) => literal.span(),
                    NodeNameFragment::Empty => Span::call_site(),
                })
        }
        NodeName::Block(_) => Span::call_site(),
    }
}
