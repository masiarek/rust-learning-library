//! `#[logged("info")]` prints a level and the function's name when the function
//! is entered. It comes in two versions, which differ only in what they return
//! when something is wrong: with the argument, or with the item.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemFn, LitStr, parse_quote};

/// Returns the error, and nothing else.
#[proc_macro_attribute]
pub fn logged_error_only(args: TokenStream, item: TokenStream) -> TokenStream {
    let level = match level(args.into()) {
        Ok(level) => level,
        Err(error) => return error.into_compile_error().into(),
    };
    let function = syn::parse_macro_input!(item as ItemFn);
    add_log_line(&level, function).into()
}

/// Returns the error, followed by the item exactly as it arrived.
#[proc_macro_attribute]
pub fn logged(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);
    let result = level(args.into()).and_then(|level| {
        let function: ItemFn = syn::parse2(item.clone())?;
        Ok(add_log_line(&level, function))
    });
    match result {
        Ok(output) => output.into(),
        Err(error) => {
            let error = error.into_compile_error();
            quote!(#error #item).into()
        }
    }
}

/// The argument: one string literal, naming one of three levels.
fn level(args: TokenStream2) -> syn::Result<LitStr> {
    let level: LitStr = syn::parse2(args)?;
    match level.value().as_str() {
        "debug" | "info" | "warn" => Ok(level),
        _ => Err(syn::Error::new_spanned(level, r#"expected "debug", "info" or "warn""#)),
    }
}

fn add_log_line(level: &LitStr, mut function: ItemFn) -> TokenStream2 {
    let name = function.sig.ident.to_string();
    let body = &function.block;
    function.block = parse_quote! {{
        ::std::println!("[{}] {}", #level, #name);
        #body
    }};
    quote!(#function)
}
