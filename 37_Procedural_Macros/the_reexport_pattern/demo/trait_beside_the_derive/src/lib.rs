//! One crate for the trait and its derive: the layout the split exists to avoid.

use proc_macro::TokenStream;

pub trait Describe {
    fn describe(&self) -> String;
}

#[proc_macro_derive(Describe)]
pub fn derive_describe(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
