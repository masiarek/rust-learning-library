//! `quote!` in an ordinary program. Each result is a `proc_macro2::TokenStream`,
//! printed with `Display`, which puts a space between most pairs of tokens.

use proc_macro2::{Ident, Literal, Span};
use quote::{format_ident, quote};
use syn::{Expr, Type};

fn main() {
    // `#var` inserts anything that implements `ToTokens`: an identifier, a
    // syntax tree from `syn`, another token stream.
    let name = Ident::new("total", Span::call_site());
    let ty: Type = syn::parse_str("Vec<u32>").unwrap();
    let expr: Expr = syn::parse_str("vec![1, 2, 3]").unwrap();
    println!("1. {}", quote! { let #name: #ty = #expr; });

    // A Rust value becomes a literal, and an integer keeps its type as a suffix.
    let (label, count) = ("points", 3_usize);
    println!("2. {}", quote! { const LABEL: &str = #label; const COUNT: u8 = #count; });
    let count = Literal::usize_unsuffixed(count);
    println!("3. {}", quote! { const COUNT: u8 = #count; });

    // `#(...),*` repeats with a separator, `#(...)*` without one.
    let fields = vec![format_ident!("x"), format_ident!("y")];
    println!("4. {}", quote! { #(#fields),* });
    println!("5. {}", quote! { #(let #fields = 0;)* });

    // Two iterators in one repetition advance together, like `zip`.
    let types: Vec<Type> = vec![syn::parse_str("i32").unwrap(), syn::parse_str("f64").unwrap()];
    println!("6. {}", quote! { struct Point { #(#fields: #types),* } });

    // A value that is not an iterator is inserted again on every pass.
    println!("7. {}", quote! { #(#fields: #name),* });

    // `format_ident!` builds an identifier from pieces, `Ident::new` from a whole string.
    let check = format_ident!("check_{}", fields[0]);
    let same = Ident::new("check_x", Span::call_site());
    println!("8. {check} {}", check == same);
}
