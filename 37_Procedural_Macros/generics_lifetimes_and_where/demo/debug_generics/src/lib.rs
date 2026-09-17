//! One `Debug` derive, written four ways. The body of `fmt` is the same in
//! all four; they differ only in the `impl` line and its bounds, which is
//! where a derive on a generic type succeeds or fails.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Generics, parse_macro_input, parse_quote};

/// Right: repeats the generics, and bounds every type parameter by `Debug`.
#[proc_macro_derive(DebugBoundingParams)]
pub fn debug_bounding_params(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    bound_each_type_param(&mut input.generics);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (ident, body) = (&input.ident, body(&input));
    quote!(impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause #body)
        .into()
}

/// Right, differently: repeats the generics, and bounds every field's type.
#[proc_macro_derive(DebugBoundingFields)]
pub fn debug_bounding_fields(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    bound_each_field_type(&mut input);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (ident, body) = (&input.ident, body(&input));
    quote!(impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause #body)
        .into()
}

/// Wrong: `impl Debug for Wrapper`, as if `Wrapper` took no parameters.
#[proc_macro_derive(DebugIgnoringGenerics)]
pub fn debug_ignoring_generics(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (ident, body) = (&input.ident, body(&input));
    quote!(impl ::core::fmt::Debug for #ident #body).into()
}

/// Wrong: `DebugBoundingParams` with `#where_clause` left out of the `impl`.
#[proc_macro_derive(DebugForgettingWhere)]
pub fn debug_forgetting_where(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    bound_each_type_param(&mut input.generics);
    let (impl_generics, ty_generics, _where_clause) = input.generics.split_for_impl();
    let (ident, body) = (&input.ident, body(&input));
    quote!(impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #body).into()
}

/// `T: Debug` for every type parameter `T`. Lifetimes and consts need none.
fn bound_each_type_param(generics: &mut Generics) {
    // Collected first: `type_params` borrows `generics`, `make_where_clause` changes it.
    let params: Vec<_> = generics.type_params().map(|p| p.ident.clone()).collect();
    for param in params {
        let predicate = parse_quote!(#param: ::core::fmt::Debug);
        let where_clause = generics.make_where_clause(); // creates one if there is none
        where_clause.predicates.push(predicate);
    }
}

/// `FieldType: Debug` for every field, whatever the field's type mentions.
fn bound_each_field_type(input: &mut DeriveInput) {
    let types: Vec<_> = fields(input).map(|field| field.ty.clone()).collect();
    for ty in types {
        let predicate = parse_quote!(#ty: ::core::fmt::Debug);
        let where_clause = input.generics.make_where_clause();
        where_clause.predicates.push(predicate);
    }
}

/// `{ fn fmt(..) { f.debug_struct("Name").field("a", &self.a)...finish() } }`
fn body(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.to_string();
    let members: Vec<_> = fields(input).filter_map(|f| f.ident.as_ref()).collect();
    let labels = members.iter().map(|member| member.to_string());
    quote!({
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct(#name) #(.field(#labels, &self.#members))* .finish()
        }
    })
}

/// The named fields. Other shapes are the subject of every_struct_and_enum_shape.
fn fields(input: &DeriveInput) -> impl Iterator<Item = &Field> {
    let Data::Struct(data) = &input.data else {
        panic!("this derive supports structs only");
    };
    let Fields::Named(named) = &data.fields else {
        panic!("this derive supports named fields only");
    };
    named.named.iter()
}
