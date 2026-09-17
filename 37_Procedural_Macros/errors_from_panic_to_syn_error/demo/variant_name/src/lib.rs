//! `#[derive(VariantName)]`: `name()` returns a variant's name, for an enum
//! whose variants carry no data.
//!
//! A variant that does carry data is the user's mistake. The three derives
//! report it three ways, and are otherwise the same.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Variant, parse_macro_input};

/// Way 1: panic.
#[proc_macro_derive(VariantNamePanics)]
pub fn variant_name_panics(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(data) = &input.data else {
        panic!("VariantName is for enums");
    };
    if let Some(variant) = data.variants.iter().find(|v| has_data(v)) {
        panic!("{}", mistake(variant));
    }
    generate(&input.ident, data.variants.iter()).into()
}

/// Way 2: return a `compile_error!` instead of the impl.
#[proc_macro_derive(VariantNameCompileError)]
pub fn variant_name_compile_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(data) = &input.data else {
        return quote!(::core::compile_error!("VariantName is for enums");).into();
    };
    if let Some(variant) = data.variants.iter().find(|v| has_data(v)) {
        let message = mistake(variant);
        return quote!(::core::compile_error!(#message);).into();
    }
    generate(&input.ident, data.variants.iter()).into()
}

/// Way 3: a `syn::Error` for every mistake, each spanned on the wrong tokens.
#[proc_macro_derive(VariantName)]
pub fn variant_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Returning `syn::Result` is what lets every check below use `?`.
fn expand(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "VariantName is for enums",
        ));
    };
    check_variants(data.variants.iter())?;
    Ok(generate(&input.ident, data.variants.iter()))
}

/// One error for every variant that carries data, combined into one `Err`.
fn check_variants<'a>(variants: impl Iterator<Item = &'a Variant>) -> syn::Result<()> {
    let combined = variants
        .filter(|v| has_data(v))
        .map(|v| syn::Error::new_spanned(&v.fields, mistake(v)))
        .reduce(|mut all, next| {
            all.combine(next);
            all
        });
    match combined {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

fn has_data(variant: &Variant) -> bool {
    !matches!(variant.fields, Fields::Unit)
}

fn mistake(variant: &Variant) -> String {
    let name = &variant.ident;
    format!("`{name}` carries data; VariantName needs variants without fields")
}

fn generate<'a>(name: &Ident, variants: impl Iterator<Item = &'a Variant>) -> TokenStream2 {
    let variants: Vec<&Ident> = variants.map(|v| &v.ident).collect();
    quote! {
        impl #name {
            pub fn name(&self) -> &'static str {
                match self {
                    #(Self::#variants => ::core::stringify!(#variants),)*
                }
            }
        }
    }
}
