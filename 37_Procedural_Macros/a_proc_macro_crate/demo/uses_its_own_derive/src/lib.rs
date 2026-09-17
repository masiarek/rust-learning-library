//! A derive, used in the crate that defines it.

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn derive_hello_macro(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[derive(HelloMacro)]
struct Pancakes;
