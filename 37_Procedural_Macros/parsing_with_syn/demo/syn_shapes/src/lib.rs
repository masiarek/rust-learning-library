//! Two macros that read a type definition with `syn` and describe its shape.
//!
//! Both start the same way: `parse_macro_input!` turns the tokens into a
//! `DeriveInput`, or hands the parse error back to the compiler. After that the
//! work is a `match` on `Data` and `Fields`, not a walk over tokens.

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Fields, FieldsNamed, parse_macro_input};

/// `#[derive(Shape)]` adds `pub const SHAPE: &str` to the type.
#[proc_macro_derive(Shape)]
pub fn derive_shape(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let shape = describe(&input);
    quote! {
        impl #name {
            pub const SHAPE: &str = #shape;
        }
    }
    .into()
}

/// `shape_of!(struct Foo;)` becomes a string literal. Unlike a derive, it can be
/// handed tokens that are not a type definition at all.
#[proc_macro]
pub fn shape_of(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let shape = describe(&input);
    quote! { #shape }.into()
}

fn describe(input: &DeriveInput) -> String {
    match &input.data {
        Data::Struct(data) => format!("struct with {}", describe_fields(&data.fields)),
        Data::Enum(data) => {
            let variants: Vec<String> = data
                .variants
                .iter()
                .map(|variant| format!("{} with {}", variant.ident, describe_fields(&variant.fields)))
                .collect();
            format!("enum of {}", variants.join("; "))
        }
        Data::Union(data) => format!("union with {}", describe_named(&data.fields)),
    }
}

fn describe_fields(fields: &Fields) -> String {
    match fields {
        Fields::Named(named) => describe_named(named),
        Fields::Unnamed(unnamed) => {
            let types: Vec<String> =
                unnamed.unnamed.iter().map(|field| field.ty.to_token_stream().to_string()).collect();
            format!("unnamed fields ({})", types.join(", "))
        }
        Fields::Unit => "no fields".to_string(),
    }
}

fn describe_named(named: &FieldsNamed) -> String {
    // Every field in `FieldsNamed` has a name, so `ident` is always `Some`.
    let names: Vec<String> = named.named.iter().map(|field| field.ident.as_ref().unwrap().to_string()).collect();
    format!("named fields {}", names.join(", "))
}
