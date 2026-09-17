//! `#[derive(Validate)]` writes a `validate` method that calls `check_<field>`
//! for every field, a function the user writes. It is built twice: with
//! `quote!`, and by formatting a string and parsing it. Both write the same
//! code; only the spans differ.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, Fields, parse_macro_input};

#[proc_macro_derive(Validate)]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match named_fields(&input) {
        Ok(fields) => fields,
        Err(error) => return error.into_compile_error().into(),
    };
    let name = &input.ident;
    let idents: Vec<_> = fields.iter().map(|field| field.ident.as_ref().unwrap()).collect();
    // `format_ident!` gives each new identifier the span of the field's name.
    let checks = idents.iter().map(|ident| format_ident!("check_{}", ident));
    quote! {
        impl #name {
            pub fn validate(&self) -> ::core::result::Result<(), ::std::string::String> {
                #( Self::#checks(&self.#idents)?; )*
                ::core::result::Result::Ok(())
            }
        }
    }
    .into()
}

/// The same method, written as text and parsed. Every token parsed from a
/// string gets the same span: the place the macro was called.
#[proc_macro_derive(ValidateFromString)]
pub fn derive_validate_from_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match named_fields(&input) {
        Ok(fields) => fields,
        Err(error) => return error.into_compile_error().into(),
    };
    let mut calls = String::new();
    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        calls += &format!("Self::check_{ident}(&self.{ident})?; ");
    }
    format!(
        "impl {} {{ pub fn validate(&self) -> ::core::result::Result<(), ::std::string::String> {{ {calls}::core::result::Result::Ok(()) }} }}",
        input.ident
    )
    .parse()
    .unwrap()
}

fn named_fields(input: &DeriveInput) -> syn::Result<Vec<&Field>> {
    match &input.data {
        Data::Struct(data) if matches!(data.fields, Fields::Named(_)) => Ok(data.fields.iter().collect()),
        _ => Err(syn::Error::new_spanned(&input.ident, "Validate needs a struct with named fields")),
    }
}
