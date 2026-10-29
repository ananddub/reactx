//! HTML/JSX syntax for ReactX.
//!
//! `jsx!` parses genuine `<Tag attr="value">children</Tag>` markup with
//! [`rstml`] and re-emits the equivalent `rsx!` invocation. Components, hooks
//! and the terminal renderer are untouched: only the surface syntax differs.
//!
//! Supported:
//! - nested elements and components, including fragments `<>...</>`
//! - `attr="text"`, `attr={expression}`, `on_click=move |_| ...` and boolean
//!   shorthand (`disabled`)
//! - text children and `{expression}` interpolation, where the expression is
//!   anything implementing `reactx::IntoJsxNode` (strings, numbers, signals,
//!   elements, options and iterator maps)
//! - hyphenated attribute names such as `border-color`
//!
//! Not supported (fails with a compile error): dynamic tag names, spread-style
//! `{...}` attributes and closure-binding attributes like `on_click(event)`.

use proc_macro::TokenStream;

mod emit;
mod names;
mod text;

#[cfg(test)]
mod tests;

/// Translate JSX markup into `rsx!`.
#[proc_macro]
pub fn jsx(input: TokenStream) -> TokenStream {
    match emit::expand(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
