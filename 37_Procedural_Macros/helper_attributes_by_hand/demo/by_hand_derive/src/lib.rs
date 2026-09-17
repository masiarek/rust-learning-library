//! `#[derive(CustomDebug)]`: a `Debug` impl that the struct can shape with
//! `#[debug(...)]` attributes. This file declares the macro and writes the
//! `impl`; reading the attributes is `attrs.rs`'s job.

mod attrs;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// `attributes(debug)` is the declaration: it lets `#[debug(...)]` appear on
/// the struct and on its fields, and hands it to this function untouched.
#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive_custom_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match attrs::Container::parse(&input) {
        Ok(container) => expand(&container).into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// `f.debug_struct(name).field(..)...finish()`, one `.field` per shown field.
/// Generics are ignored here; `generics_lifetimes_and_where` adds them.
fn expand(container: &attrs::Container) -> proc_macro2::TokenStream {
    let ident = &container.ident;
    let name = container
        .rename
        .clone()
        .unwrap_or_else(|| ident.to_string());
    let mask = &container.mask;
    let shown = container.fields.iter().filter(|field| !field.skip);
    let calls = shown.map(|field| {
        let member = &field.ident;
        let label = field.rename.clone().unwrap_or_else(|| member.to_string());
        if field.redact {
            quote!(.field(#label, &::core::format_args!("{}", #mask)))
        } else {
            quote!(.field(#label, &self.#member))
        }
    });
    // A skipped field still exists, so say so: `finish_non_exhaustive` prints `..`.
    let finish = if container.fields.iter().any(|field| field.skip) {
        quote!(finish_non_exhaustive)
    } else {
        quote!(finish)
    };
    quote! {
        impl ::core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(#name) #(#calls)* .#finish()
            }
        }
    }
}
