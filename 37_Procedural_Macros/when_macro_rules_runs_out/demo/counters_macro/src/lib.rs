//! `counters!(requests, errors)`: a struct with one `u64` per name, and an
//! `incr_<name>` method for each.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{Ident, Token, parse_macro_input};

#[proc_macro]
pub fn counters(input: TokenStream) -> TokenStream {
    let names = parse_macro_input!(input with Punctuated::<Ident, Token![,]>::parse_terminated);
    let fields: Vec<&Ident> = names.iter().collect();
    // The identifier `macro_rules!` cannot write. `format_ident!` gives it the
    // span of `name`, the name as the caller typed it.
    let methods: Vec<Ident> = fields.iter().map(|name| format_ident!("incr_{}", name)).collect();
    quote! {
        pub struct Counters {
            #(pub #fields: u64,)*
        }

        impl Counters {
            pub fn new() -> Self {
                Self { #(#fields: 0,)* }
            }

            #(pub fn #methods(&mut self) {
                self.#fields += 1;
            })*
        }
    }
    .into()
}
