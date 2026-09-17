//! `#[retry(times = 3, delay_ms = 100)]` runs a function that returns `Result`
//! again whenever it returns `Err`, up to `times` attempts in all, waiting
//! `delay_ms` milliseconds between one attempt and the next.

use std::num::NonZeroU32;

use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemFn, parse_quote};

/// The arguments. `darling` generates the parsing, the defaults and the errors.
#[derive(FromMeta)]
struct RetryArgs {
    /// Attempts in all, the first one included. Zero is not a `NonZeroU32`.
    times: NonZeroU32,
    /// Milliseconds between attempts, or none when left out.
    #[darling(default)]
    delay_ms: u64,
    /// The function that waits. A program can name one that only records the delay.
    #[darling(default = default_sleep)]
    sleep: syn::Path,
}

fn default_sleep() -> syn::Path {
    parse_quote!(::std::thread::sleep)
}

#[proc_macro_attribute]
pub fn retry(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);
    match expand(args.into(), item.clone()) {
        Ok(function) => function.into(),
        Err(errors) => {
            // Every error darling collected, and the function as it was written.
            let errors = errors.write_errors();
            quote!(#errors #item).into()
        }
    }
}

fn expand(args: TokenStream2, item: TokenStream2) -> darling::Result<TokenStream2> {
    let args = RetryArgs::from_list(&NestedMeta::parse_meta_list(args)?)?;
    let mut function: ItemFn = syn::parse2(item)?;
    if let Some(asyncness) = &function.sig.asyncness {
        let message = "#[retry] cannot wait inside an `async fn`";
        return Err(darling::Error::custom(message).with_span(asyncness));
    }
    let syn::ReturnType::Type(_, output) = &function.sig.output else {
        let message = "#[retry] needs a function that returns a `Result`";
        return Err(darling::Error::custom(message).with_span(&function.sig.ident));
    };

    let RetryArgs { times, delay_ms, sleep } = args;
    let times = times.get();
    let body = &function.block;
    // The body becomes a closure called once per attempt, so a `return` or a
    // `?` inside it ends that attempt, not the whole function.
    function.block = parse_quote! {{
        let mut attempts_left: u32 = #times;
        loop {
            attempts_left -= 1;
            match (|| -> #output #body)() {
                ::core::result::Result::Err(_) if attempts_left > 0 => {
                    #sleep(::core::time::Duration::from_millis(#delay_ms));
                }
                result => return result,
            }
        }
    }};
    Ok(quote!(#function))
}
