//! `#[trace]` prints a line when a function is entered and another when it is left.
//!
//! An attribute's output replaces the item, so the macro parses the function,
//! changes one field of it, and returns all of it: the attributes, visibility
//! and signature go back exactly as they were parsed.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn trace(args: TokenStream, item: TokenStream) -> TokenStream {
    // The first stream is whatever sat between the parentheses of `#[trace(…)]`.
    // This macro takes nothing there, so any token at all is an error.
    if !args.is_empty() {
        let args = proc_macro2::TokenStream::from(args);
        let item = proc_macro2::TokenStream::from(item);
        let error = syn::Error::new_spanned(args, "#[trace] takes no arguments");
        let error = error.into_compile_error();
        // The function goes back too, or every call to it is a second error.
        return quote!(#error #item).into();
    }

    let mut function = parse_macro_input!(item as ItemFn);
    let name = function.sig.ident.to_string();
    let body = &function.block;
    // `leave` is printed by a value dropped on the way out, so it runs however
    // the body ends: at its last expression, at a `return`, or at a `?`.
    function.block = parse_quote! {{
        struct Leave;
        impl ::core::ops::Drop for Leave {
            fn drop(&mut self) {
                ::std::println!("leave {}", #name);
            }
        }
        ::std::println!("enter {}", #name);
        let _leave = Leave;
        #body
    }};
    quote!(#function).into()
}

/// The version that looks right: print `leave` after the body. A `return` or a
/// `?` inside the body leaves the function before that line is reached.
#[proc_macro_attribute]
pub fn trace_after_body(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    let name = function.sig.ident.to_string();
    let body = &function.block;
    function.block = parse_quote! {{
        ::std::println!("enter {}", #name);
        let result = #body;
        ::std::println!("leave {}", #name);
        result
    }};
    quote!(#function).into()
}
