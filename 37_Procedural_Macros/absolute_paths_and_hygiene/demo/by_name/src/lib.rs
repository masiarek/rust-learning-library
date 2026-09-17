//! `#[derive(ByName)]` for an enum whose variants carry no data: `Display`
//! prints a variant's name, and `FromStr` parses the name back.
//!
//! The three derives generate the same code and differ only in how they spell
//! four names. Generated code lands in the user's module, so every name in it
//! is looked up there, among the user's own names.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Spelled the way the std docs spell `Display` and `FromStr`, which assumes a
/// module that imported `fmt` and `FromStr` and uses std's `Result`.
#[proc_macro_derive(ByNameAsWritten)]
pub fn by_name_as_written(input: TokenStream) -> TokenStream {
    expand(
        input,
        quote!(fmt),
        quote!(FromStr),
        quote!(Result),
        quote!(str),
    )
}

/// Full paths, but relative ones: `core` is looked up in the user's module first.
/// And `str` as everyone writes it.
#[proc_macro_derive(ByNameCore)]
pub fn by_name_core(input: TokenStream) -> TokenStream {
    expand(
        input,
        quote!(core::fmt),
        quote!(core::str::FromStr),
        quote!(core::result::Result),
        quote!(str),
    )
}

/// A leading `::` starts the path at a crate name, which no module can shadow.
#[proc_macro_derive(ByName)]
pub fn by_name(input: TokenStream) -> TokenStream {
    expand(
        input,
        quote!(::core::fmt),
        quote!(::core::str::FromStr),
        quote!(::core::result::Result),
        quote!(::core::primitive::str),
    )
}

fn expand(
    input: TokenStream,
    fmt: TokenStream2,
    from_str: TokenStream2,
    result: TokenStream2,
    str_type: TokenStream2,
) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(name, "ByName is for enums")
            .into_compile_error()
            .into();
    };
    let variants: Vec<_> = data.variants.iter().map(|v| &v.ident).collect();
    let names: Vec<_> = variants.iter().map(|v| v.to_string()).collect();
    quote! {
        impl #fmt::Display for #name {
            fn fmt(&self, f: &mut #fmt::Formatter<'_>) -> #fmt::Result {
                f.write_str(match self {
                    #(Self::#variants => #names,)*
                })
            }
        }

        impl #from_str for #name {
            type Err = &'static #str_type;

            fn from_str(s: &#str_type) -> #result<Self, Self::Err> {
                match s {
                    #(#names => #result::Ok(Self::#variants),)*
                    _ => #result::Err("no variant has that name"),
                }
            }
        }
    }
    .into()
}
