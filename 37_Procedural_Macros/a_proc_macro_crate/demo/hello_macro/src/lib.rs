//! The smallest useful derive: `#[derive(HelloMacro)]` implements a trait for
//! the type it is on. No `syn`, no `quote`, no dependencies at all; the
//! `proc_macro` crate comes with the compiler.

use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(HelloMacro)]
pub fn derive_hello_macro(item: TokenStream) -> TokenStream {
    let name = type_name(item);
    format!(
        r#"impl HelloMacro for {name} {{
            fn hello_macro() -> &'static str {{ "Hello, Macro! My name is {name}!" }}
        }}"#
    )
    .parse()
    .unwrap()
}

/// The identifier after `struct`, `enum` or `union`. A private function is
/// fine: only the macros themselves may be `pub`.
fn type_name(item: TokenStream) -> String {
    let is_keyword = |token: &TokenTree| {
        matches!(token, TokenTree::Ident(i) if ["struct", "enum", "union"].contains(&i.to_string().as_str()))
    };
    let mut tokens = item.into_iter().skip_while(|token| !is_keyword(token));
    match (tokens.next(), tokens.next()) {
        (Some(_keyword), Some(TokenTree::Ident(name))) => name.to_string(),
        _ => panic!("HelloMacro goes on a struct, an enum or a union"),
    }
}
