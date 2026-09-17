//! `#[derive(CustomDebug)]`: a `Debug` impl that the struct can shape with
//! `#[debug(...)]` attributes. This file declares the macro and writes the
//! `impl`; reading the attributes is `attrs.rs`'s job, and darling's.

mod attrs;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// `attributes(debug)` is still the compiler's business: darling reads the
/// attribute, but only this declaration lets the user write it.
#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive_custom_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match attrs::Container::from_derive_input(&input) {
        Ok(container) => expand(&container).into(),
        Err(errors) => errors.write_errors().into(),
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
    let fields = container.data.as_struct().expect("supports(struct_named)");
    let shown = fields.iter().filter(|field| !field.skip.is_present());
    let calls = shown.map(|field| {
        let member = field.ident.as_ref().expect("named fields have names");
        let label = field.rename.clone().unwrap_or_else(|| member.to_string());
        if field.redact.is_present() {
            quote!(.field(#label, &::core::format_args!("{}", #mask)))
        } else {
            quote!(.field(#label, &self.#member))
        }
    });
    // A skipped field still exists, so say so: `finish_non_exhaustive` prints `..`.
    let finish = if fields.iter().any(|field| field.skip.is_present()) {
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
