//! `routes!`: a table of `METHOD "path" => fn handler(..) { .. }` lines,
//! checked while compiling and turned into one `route` function.

mod check;
mod route;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use route::{Routes, Segment, parameters, segments};
use syn::parse_macro_input;

#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    let Routes(routes) = parse_macro_input!(input as Routes);
    if let Err(errors) = check::check(&routes) {
        return errors.into_compile_error().into();
    }
    let arms = routes.iter().map(|route| {
        let method = route.method.to_string();
        let patterns = segments(&route.path.value()).unwrap().into_iter().map(|segment| match segment {
            Segment::Literal(text) => quote!(#text),
            Segment::Param(name) => {
                let name = format_ident!("{}", name);
                quote!(#name)
            }
        });
        let handler = &route.handler.sig.ident;
        let name = handler.to_string();
        let arguments = parameters(&route.handler).unwrap();
        quote! {
            (#method, [#(#patterns),*]) => Some((#name, #handler(#(#arguments),*))),
        }
    });
    let handlers = routes.iter().map(|route| &route.handler);
    quote! {
        #(#handlers)*

        /// The handler for `method` and `path`, and what it returned.
        pub fn route(method: &str, path: &str) -> Option<(&'static str, String)> {
            let segments: Vec<&str> = path.split('/').skip(1).collect();
            match (method, segments.as_slice()) {
                #(#arms)*
                _ => None,
            }
        }
    }
    .into()
}
