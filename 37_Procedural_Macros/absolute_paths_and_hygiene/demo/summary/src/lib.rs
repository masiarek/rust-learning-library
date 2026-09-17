//! `#[derive(Summary)]`: a `summary()` method that lists every field as
//! `[name = value]`.
//!
//! The generated method binds the user's fields by name, then builds its result
//! in a local variable of its own, `text`. The two derives differ in one thing:
//! the span carried by the identifiers the macro makes up, `text` and `summary`.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, parse_macro_input};

/// Both identifiers resolve as if the user had typed them where `#[derive]` is.
#[proc_macro_derive(SummaryCallSite)]
pub fn summary_call_site(input: TokenStream) -> TokenStream {
    expand(input, Span::call_site())
}

/// `text` becomes a local variable only the macro's own tokens can name.
#[proc_macro_derive(Summary)]
pub fn summary(input: TokenStream) -> TokenStream {
    expand(input, Span::mixed_site())
}

fn expand(input: TokenStream, span: Span) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &input.data
    else {
        return syn::Error::new_spanned(name, "Summary needs named fields")
            .into_compile_error()
            .into();
    };
    // The user's field names, carrying the spans they have in the user's source.
    let fields: Vec<&Ident> = fields
        .named
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect();
    let labels = fields.iter().map(|f| f.to_string());
    let text = Ident::new("text", span);
    let summary = Ident::new("summary", span);
    quote! {
        impl #name {
            pub fn #summary(&self) -> ::std::string::String {
                let Self { #(#fields),* } = self;
                let mut #text = ::std::string::String::new();
                #(#text.push_str(&::std::format!("[{} = {}]", #labels, #fields));)*
                #text
            }
        }
    }
    .into()
}
