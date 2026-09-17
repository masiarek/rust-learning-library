//! Two derives that work on `struct Point { x: i32, y: i32 }` and break on
//! `struct Meters(f64)`. Both return each field's value, and nothing else.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Written with only named fields in mind. `field.ident` is an `Option<Ident>`,
/// and `quote` prints `None` as no tokens at all, so on a tuple struct this
/// generates `&self. as &dyn Debug`.
#[proc_macro_derive(DescribeNamedOnly)]
pub fn derive_named_only(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Struct(data) = &input.data else {
        panic!("structs only");
    };
    let idents = data.fields.iter().map(|field| &field.ident);
    quote! {
        impl #name {
            pub fn describe(&self) -> ::std::vec::Vec<&dyn ::core::fmt::Debug> {
                ::std::vec::Vec::from([#(&self.#idents as &dyn ::core::fmt::Debug),*])
            }
        }
    }
    .into()
}

/// Knows that tuple fields have positions, but interpolates the position as a
/// `usize`, and `quote` prints a `usize` with its type suffix: `0usize`.
#[proc_macro_derive(DescribeUsizeIndex)]
pub fn derive_usize_index(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Struct(data) = &input.data else {
        panic!("structs only");
    };
    let positions = 0..data.fields.len();
    quote! {
        impl #name {
            pub fn describe(&self) -> ::std::vec::Vec<&dyn ::core::fmt::Debug> {
                ::std::vec::Vec::from([#(&self.#positions as &dyn ::core::fmt::Debug),*])
            }
        }
    }
    .into()
}
