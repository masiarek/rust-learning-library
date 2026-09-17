//! `mask` from `darling_derive`, declared without `#[darling(default)]`.

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[derive(FromDeriveInput)]
#[darling(attributes(debug))]
struct Container {
    ident: syn::Ident,
    mask: String, // no #[darling(default)]
}

/// Adds `const MASK`, so there is something to generate from the attribute.
#[proc_macro_derive(MaskRequired, attributes(debug))]
pub fn derive_mask_required(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match Container::from_derive_input(&input) {
        Ok(Container { ident, mask }) => {
            quote!(impl #ident { pub const MASK: &str = #mask; }).into()
        }
        Err(errors) => errors.write_errors().into(),
    }
}
