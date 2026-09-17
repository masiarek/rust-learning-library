//! `filter!(status <> "cancelled" and total >= 5000)`: a query language that is
//! not Rust, turned into a closure for `Iterator::filter`.

mod grammar;

use grammar::{Condition, Filter, Op};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn filter(input: TokenStream) -> TokenStream {
    let filter = parse_macro_input!(input as Filter);
    let tests = filter.conditions.iter().map(|condition| match condition {
        Condition::Compare { field, op, value } => {
            let op = match op {
                Op::Eq => quote!(==),
                Op::NotEq => quote!(!=),
                Op::Le => quote!(<=),
                Op::Ge => quote!(>=),
                Op::Lt => quote!(<),
                Op::Gt => quote!(>),
            };
            quote!(row.#field #op #value)
        }
        Condition::In { field, values } => {
            let values = values.iter();
            quote!([#(#values),*].contains(&row.#field))
        }
    });
    quote!(|row| #(#tests)&&*).into()
}
