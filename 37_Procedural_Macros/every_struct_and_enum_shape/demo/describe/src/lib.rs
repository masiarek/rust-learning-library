//! `#[derive(Describe)]`: a `describe()` method for every shape a struct or an
//! enum can take.
//!
//! The method returns the shape, written the way Rust writes it, and each
//! field's name and value. Getting there means handling all three kinds of
//! `Fields` twice: reached through `self` in a struct, bound by a pattern in an
//! enum.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, Index, Variant, parse_macro_input};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let body = match &input.data {
        Data::Struct(data) => describe_struct(name, &data.fields),
        Data::Enum(data) => {
            let arms = data.variants.iter().map(|v| describe_variant(name, v));
            quote!(match self { #(#arms)* })
        }
        // A union does not record which field holds a value, so there is
        // nothing safe to read. Refuse it, pointing at the `union` keyword.
        Data::Union(data) => {
            let message = "Describe cannot read a union: nothing records which field is set";
            return syn::Error::new_spanned(data.union_token, message)
                .into_compile_error()
                .into();
        }
    };
    quote! {
        impl #name {
            pub fn describe(&self) -> (&'static str, ::std::vec::Vec<(&'static str, &dyn ::core::fmt::Debug)>) {
                #body
            }
        }
    }
    .into()
}

/// A struct's fields are reached through `self`: `self.x` when a field has a
/// name, `self.0` when it only has a position.
fn describe_struct(name: &Ident, fields: &Fields) -> TokenStream2 {
    let label = label(name.to_string(), fields);
    let names = field_names(fields);
    let members = fields
        .iter()
        .enumerate()
        .map(|(i, field)| match &field.ident {
            Some(ident) => quote!(#ident),
            None => {
                let index = Index::from(i); // prints as `0`; a bare usize prints as `0usize`
                quote!(#index)
            }
        });
    quote! {
        (#label, ::std::vec::Vec::from([
            #((#names, &self.#members as &dyn ::core::fmt::Debug)),*
        ]))
    }
}

/// An enum's fields are only reachable through a pattern, one `match` arm per
/// variant, and the pattern's shape has to match the variant's.
fn describe_variant(enum_name: &Ident, variant: &Variant) -> TokenStream2 {
    let variant_name = &variant.ident;
    let label = label(format!("{enum_name}::{variant_name}"), &variant.fields);
    let names = field_names(&variant.fields);
    let bindings: Vec<Ident> = (0..variant.fields.len())
        .map(|i| format_ident!("field_{i}"))
        .collect();
    let pattern = match &variant.fields {
        Fields::Named(fields) => {
            let idents = fields.named.iter().map(|f| &f.ident);
            quote!(Self::#variant_name { #(#idents: #bindings),* })
        }
        Fields::Unnamed(_) => quote!(Self::#variant_name(#(#bindings),*)),
        Fields::Unit => quote!(Self::#variant_name),
    };
    quote! {
        #pattern => (#label, ::std::vec::Vec::from([
            #((#names, #bindings as &dyn ::core::fmt::Debug)),*
        ])),
    }
}

/// `Point { .. }`, `Meters(..)` or `Origin`: the shape, in Rust's own syntax.
fn label(name: String, fields: &Fields) -> String {
    match fields {
        Fields::Named(_) => format!("{name} {{ .. }}"),
        Fields::Unnamed(_) => format!("{name}(..)"),
        Fields::Unit => name,
    }
}

/// `x` for a named field, `0` for the first unnamed one.
fn field_names(fields: &Fields) -> impl Iterator<Item = String> + '_ {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| match &field.ident {
            Some(ident) => ident.to_string(),
            None => i.to_string(),
        })
}
