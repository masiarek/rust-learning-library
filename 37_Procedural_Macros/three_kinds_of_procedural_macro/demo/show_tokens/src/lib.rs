//! One procedural macro of each kind, and two that misbehave on purpose.
//!
//! None of them uses `syn` or `quote`. Each turns the tokens it was handed into
//! a string and returns code that carries the string, so the program that uses
//! them can print exactly what a macro receives.

use proc_macro::{TokenStream, TokenTree};

/// A derive receives the item it is attached to, and returns items to add after it.
#[proc_macro_derive(ShowTokens)]
pub fn derive_show_tokens(item: TokenStream) -> TokenStream {
    let name = name_after_keyword(&item);
    let received = item.to_string();
    format!("impl {name} {{ pub const RECEIVED: &str = {received:?}; }}")
        .parse()
        .unwrap()
}

/// A function-like macro receives what sits between its delimiters, and returns
/// what the call becomes.
#[proc_macro]
pub fn show_tokens(input: TokenStream) -> TokenStream {
    format!("{:?}", input.to_string()).parse().unwrap()
}

/// An attribute receives its own arguments and the item, and returns the item's
/// replacement. This one puts the item back and adds two constants beside it.
#[proc_macro_attribute]
pub fn show_attr(args: TokenStream, item: TokenStream) -> TokenStream {
    let name = name_after_keyword(&item).to_uppercase();
    let (args, received) = (args.to_string(), item.to_string());
    format!("{item} pub const {name}_ARGS: &str = {args:?}; pub const {name}_ITEM: &str = {received:?};")
        .parse()
        .unwrap()
}

/// A derive that hands its input straight back. The compiler keeps the original
/// item too, so the program ends up with two.
#[proc_macro_derive(Reemit)]
pub fn derive_reemit(item: TokenStream) -> TokenStream {
    item
}

/// An attribute that returns nothing, so the item is replaced by nothing.
#[proc_macro_attribute]
pub fn swallow(_args: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// The identifier after `struct`, `enum` or `fn`.
fn name_after_keyword(item: &TokenStream) -> String {
    let mut tokens = item.clone().into_iter();
    while let Some(token) = tokens.next() {
        if let TokenTree::Ident(keyword) = token
            && matches!(keyword.to_string().as_str(), "struct" | "enum" | "fn")
            && let Some(TokenTree::Ident(name)) = tokens.next()
        {
            return name.to_string();
        }
    }
    panic!("expected a struct, an enum or a fn")
}
