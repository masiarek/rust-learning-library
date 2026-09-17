//! `split_for_impl` outside a macro. `syn` and `quote` run on `proc-macro2`,
//! which works in an ordinary program, so the three pieces can be printed.

use quote::quote;
use syn::{DeriveInput, parse_quote};

fn main() {
    // The struct from generics_app, plus a default for each type and const parameter.
    let input: DeriveInput = parse_quote! {
        struct Wrapper<'a, T: Clone = String, const N: usize = 3>
        where
            T: PartialEq,
        {
            label: &'a str,
            items: [T; N],
        }
    };
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    println!("impl_generics  {}", quote!(#impl_generics));
    println!("ty_generics    {}", quote!(#ty_generics));
    println!("where_clause   {}", quote!(#where_clause));
}
