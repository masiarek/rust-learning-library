//! The trait and its derive in one crate, the way you would lay out an
//! ordinary library.

use proc_macro::TokenStream;

pub trait HelloMacro {
    fn hello_macro() -> &'static str;
}

pub fn greeting(name: &str) -> String {
    format!("Hello, Macro! My name is {name}!")
}

#[proc_macro_derive(HelloMacro)]
pub fn derive_hello_macro(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
