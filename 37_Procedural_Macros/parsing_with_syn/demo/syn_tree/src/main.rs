//! `syn` is an ordinary library, so an ordinary program can call it: here, to
//! print the tree a derive on a one-field struct works with.

use syn::DeriveInput;

fn main() {
    let from_str: DeriveInput = syn::parse_str("pub struct Point { x: i32 }").unwrap();
    println!("{from_str:#?}");

    // The same tokens laid out differently, parsed from a token stream.
    let tokens: proc_macro2::TokenStream = "pub struct Point {\n    x: i32\n}".parse().unwrap();
    let from_tokens: DeriveInput = syn::parse2(tokens).unwrap();
    println!("same tree: {}", from_str == from_tokens);
}
