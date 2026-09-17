//! `#[derive(Describe)]`. Every path in the generated code starts with
//! `::describe_facade`, because that is the one crate a user depends on.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(name, "Describe needs a struct").into_compile_error().into();
    };
    let Fields::Named(fields) = &data.fields else {
        return syn::Error::new_spanned(name, "Describe needs named fields").into_compile_error().into();
    };
    let idents: Vec<_> = fields.named.iter().map(|field| field.ident.as_ref().unwrap()).collect();
    let labels = idents.iter().map(|ident| ident.to_string());
    let type_name = name.to_string();
    quote! {
        impl ::describe_facade::Describe for #name {
            fn describe(&self) -> ::std::string::String {
                ::describe_facade::__private::describe_fields(#type_name, &[#((#labels, &self.#idents)),*])
            }
        }
    }
    .into()
}
