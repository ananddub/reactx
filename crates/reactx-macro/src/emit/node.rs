//! Emitting elements and their attributes.

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use rstml::node::{
    AttributeValueExpr, KVAttributeValue, KeyedAttribute, KeyedAttributeValue, NodeAttribute,
    NodeBlock, NodeElement, NodeName,
};
use syn::{Block, Expr, LitStr, Stmt};

use super::emit_children;
use crate::names::{make_ident, name_span};

pub(super) fn emit_element(element: &NodeElement<rstml::Infallible>) -> syn::Result<TokenStream2> {
    let name = emit_element_name(element.name())?;
    let mut attributes = TokenStream2::new();
    for attribute in element.attributes() {
        attributes.extend(emit_attribute(attribute)?);
    }
    let children = emit_children(element.children())?;
    Ok(quote! { #name { #attributes #children } })
}

fn emit_element_name(name: &NodeName) -> syn::Result<TokenStream2> {
    match name {
        NodeName::Path(path) => Ok(path.path.to_token_stream()),
        NodeName::Punctuated(_) => {
            let ident = make_ident(&name.to_string(), name_span(name))?;
            Ok(quote! { #ident })
        }
        NodeName::Block(_) => Err(syn::Error::new(
            name_span(name),
            "jsx! does not support dynamic tag names",
        )),
    }
}

fn emit_attribute(attribute: &NodeAttribute) -> syn::Result<TokenStream2> {
    match attribute {
        NodeAttribute::Block(block) => Err(syn::Error::new_spanned(
            block,
            "jsx! does not support spread-style `{...}` attributes",
        )),
        NodeAttribute::Attribute(attribute) => emit_keyed_attribute(attribute),
    }
}

fn emit_keyed_attribute(attribute: &KeyedAttribute) -> syn::Result<TokenStream2> {
    let key = make_ident(&attribute.key.to_string(), name_span(&attribute.key))?;
    match &attribute.possible_value {
        KeyedAttributeValue::None => Ok(quote! { #key: true, }),
        KeyedAttributeValue::Binding(_) => Err(syn::Error::new(
            name_span(&attribute.key),
            "jsx! does not support binding attributes like `on_click(event)`; \
             use `on_click={move |event| ...}` instead",
        )),
        KeyedAttributeValue::Value(value) => {
            let value = attribute_value(value)?;
            // dioxus-html has no drag event; emit a custom listener attribute
            // that resolves to `reactx_tui::events::onmousedrag` via the prelude.
            if key == "onmousedrag" {
                let name = LitStr::new("onmousedrag", name_span(&attribute.key));
                return Ok(quote! { #name: onmousedrag(#value), });
            }
            Ok(quote! { #key: #value, })
        }
    }
}

fn attribute_value(value: &AttributeValueExpr) -> syn::Result<TokenStream2> {
    match &value.value {
        KVAttributeValue::Expr(Expr::Block(block)) => Ok(single_expression(&block.block)),
        KVAttributeValue::Expr(expression) => Ok(quote! { #expression }),
        KVAttributeValue::InvalidBraced(block) => Err(syn::Error::new_spanned(
            block,
            "jsx! attribute value is not a valid Rust expression",
        )),
    }
}

pub(super) fn format_block(block: &NodeBlock) -> syn::Result<TokenStream2> {
    match block {
        NodeBlock::ValidBlock(block) => {
            let value = single_expression(block);
            Ok(quote! { jsx_node(#value) })
        }
        NodeBlock::Invalid(block) => Err(syn::Error::new_spanned(
            block,
            "jsx! interpolation must be a valid Rust expression",
        )),
    }
}

/// A block with a single trailing expression can be inlined; anything else is
/// passed through as a block so `let` bindings keep working.
fn single_expression(block: &Block) -> TokenStream2 {
    match block.stmts.as_slice() {
        [Stmt::Expr(expression, None)] => quote! { #expression },
        _ => quote! { #block },
    }
}
