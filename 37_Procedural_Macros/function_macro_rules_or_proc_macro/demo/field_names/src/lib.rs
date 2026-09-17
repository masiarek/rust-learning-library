//! `#[derive(FieldNames)]`: a list of a struct's field names, and a getter for
//! each field. The getter's name, `get_x`, is built from the field's, which is
//! the step `macro_rules!` cannot take on stable Rust.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Fields, parse_macro_input};

#[proc_macro_derive(FieldNames)]
pub fn derive_field_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(data) = &input.data else {
        return Error::new_spanned(&input.ident, "FieldNames needs a struct")
            .to_compile_error()
            .into();
    };
    let Fields::Named(fields) = &data.fields else {
        return Error::new_spanned(&input.ident, "FieldNames needs named fields")
            .to_compile_error()
            .into();
    };

    let name = &input.ident;
    let fields: Vec<_> = fields.named.iter().map(|f| (f.ident.as_ref().unwrap(), &f.ty)).collect();
    let names = fields.iter().map(|(field, _)| field.to_string());
    let getters = fields.iter().map(|(field, ty)| {
        let getter = format_ident!("get_{field}");
        quote! { pub fn #getter(&self) -> &#ty { &self.#field } }
    });

    quote! {
        impl #name {
            pub const FIELD_NAMES: &[&str] = &[#(#names),*];
            #(#getters)*
        }
    }
    .into()
}
