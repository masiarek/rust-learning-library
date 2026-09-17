//! `#[derive(FieldNames)]`: a constant listing a struct's field names.
//!
//! The macro is the three-line function at the top. Everything it does is in
//! `expand`, which never names a `proc_macro` type, so a unit test can call it.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

/// The only code that touches `proc_macro`: convert in, expand, convert out.
#[proc_macro_derive(FieldNames)]
pub fn derive_field_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(input.into()).unwrap_or_else(syn::Error::into_compile_error).into()
}

/// The derive's logic, written against `proc_macro2`.
fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(&input.ident, "FieldNames needs a struct"));
    };
    let names = data.fields.iter().enumerate().map(|(i, field)| match &field.ident {
        Some(ident) => ident.to_string(),
        None => i.to_string(), // a tuple struct's fields are named 0, 1, ...
    });
    let name = &input.ident;
    Ok(quote! {
        impl #name {
            pub const FIELD_NAMES: &[&str] = &[#(#names),*];
        }
    })
}

#[cfg(test)]
mod tests {
    use super::expand;
    use quote::quote;

    #[test]
    fn lists_the_fields() {
        let output = expand(quote! { struct Point { x: i32, y: i32 } }).unwrap();
        let expected = quote! {
            impl Point {
                pub const FIELD_NAMES: &[&str] = &["x", "y"];
            }
        };
        assert_eq!(output.to_string(), expected.to_string());
    }

    #[test]
    fn refuses_an_enum() {
        let error = expand(quote! { enum Direction { Up, Down } }).unwrap_err();
        assert_eq!(error.to_string(), "FieldNames needs a struct");
    }

    /// Fails: the expected side is typed the way rustfmt would lay it out.
    #[test]
    #[ignore = "fails on purpose; run with --ignored"]
    fn expected_as_a_typed_string() {
        let output = expand(quote! { struct Point { x: i32, y: i32 } }).unwrap();
        assert_eq!(
            output.to_string(),
            r#"impl Point { pub const FIELD_NAMES: &[&str] = &["x", "y"]; }"#
        );
    }
}
