//! Translating parsed JSX nodes into `rsx!` tokens.

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use rstml::node::Node;
use syn::LitStr;

use crate::text::{clean_text, ends_tight, starts_tight};

mod node;

use node::emit_element;
use node::format_block;

pub(crate) fn expand(input: TokenStream2) -> syn::Result<TokenStream2> {
    let nodes = rstml::parse2(input)?;
    let body = emit_children(&nodes)?;
    Ok(quote! { rsx! { #body } })
}

#[derive(Clone)]
enum Piece {
    Text(String),
    Interp(TokenStream2),
    Element(TokenStream2),
}

fn emit_children(nodes: &[Node]) -> syn::Result<TokenStream2> {
    let mut pieces = Vec::new();
    collect_pieces(nodes, &mut pieces)?;

    let mut out = TokenStream2::new();
    for (index, piece) in pieces.iter().enumerate() {
        match piece {
            Piece::Text(text) => {
                let mut text = text.clone();
                let previous_interp = index > 0 && matches!(pieces[index - 1], Piece::Interp(_));
                let next_interp = matches!(pieces.get(index + 1), Some(Piece::Interp(_)));
                if previous_interp
                    && !text.starts_with(|ch: char| ch.is_whitespace())
                    && !starts_tight(&text)
                {
                    text.insert(0, ' ');
                }
                if next_interp
                    && !text.ends_with(|ch: char| ch.is_whitespace())
                    && !ends_tight(&text)
                {
                    text.push(' ');
                }

                let literal = LitStr::new(&text, Span::call_site());
                // `rsx!` drops whitespace-only text children; braces in text
                // would be parsed as format placeholders. Emitting both as an
                // interpolated string keeps the text intact.
                if text.trim().is_empty() || text.contains('{') || text.contains('}') {
                    out.extend(quote! { { #literal } });
                } else {
                    out.extend(quote! { #literal });
                }
            }
            Piece::Interp(tokens) => out.extend(quote! { { #tokens } }),
            Piece::Element(tokens) => out.extend(tokens.clone()),
        }
    }
    Ok(out)
}

fn collect_pieces(nodes: &[Node], out: &mut Vec<Piece>) -> syn::Result<()> {
    for node in nodes {
        match node {
            Node::Element(element) => out.push(Piece::Element(emit_element(element)?)),
            Node::Fragment(fragment) => collect_pieces(fragment.children(), out)?,
            Node::Text(text) => push_text(out, text.value_string()),
            Node::RawText(raw) => push_text(out, clean_text(&raw.to_string_best())),
            Node::Block(block) => out.push(Piece::Interp(format_block(block)?)),
            Node::Comment(_) | Node::Doctype(_) => {}
            Node::Custom(_) => unreachable!("jsx! does not enable rstml custom nodes"),
        }
    }
    Ok(())
}

fn push_text(out: &mut Vec<Piece>, text: String) {
    if text.is_empty() {
        return;
    }
    match out.last_mut() {
        Some(Piece::Text(previous)) => {
            if !previous.ends_with(|ch: char| ch.is_whitespace())
                && !text.starts_with(|ch: char| ch.is_whitespace())
            {
                previous.push(' ');
            }
            previous.push_str(&text);
        }
        _ => out.push(Piece::Text(text)),
    }
}
