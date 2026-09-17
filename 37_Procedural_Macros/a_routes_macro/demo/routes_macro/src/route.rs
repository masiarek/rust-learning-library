//! What `routes!` reads: `METHOD "path" => fn handler(..) { .. }`, repeated.

use syn::parse::{Parse, ParseStream};
use syn::{Ident, ItemFn, LitStr, Token};

pub struct Routes(pub Vec<Route>);

pub struct Route {
    pub method: Ident,
    pub path: LitStr,
    pub handler: ItemFn,
}

impl Parse for Routes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut routes = Vec::new();
        while !input.is_empty() {
            routes.push(input.parse()?);
        }
        Ok(Routes(routes))
    }
}

impl Parse for Route {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let method = input.parse()?;
        let path = input.parse()?;
        input.parse::<Token![=>]>()?;
        let handler = input.parse()?;
        Ok(Route { method, path, handler })
    }
}

/// The text between two slashes: matched as written, or captured by name.
pub enum Segment {
    Literal(String),
    Param(String),
}

/// Splits `/users/{id}` into `users` and `{id}`. It returns messages rather
/// than `syn::Error`s: the string's contents have no spans of their own.
pub fn segments(path: &str) -> Result<Vec<Segment>, String> {
    let Some(rest) = path.strip_prefix('/') else {
        return Err(format!("a path starts with `/`: {path:?}"));
    };
    let segment = |piece: &str| match piece.strip_prefix('{').and_then(|p| p.strip_suffix('}')) {
        Some(name) if syn::parse_str::<Ident>(name).is_ok() => Ok(Segment::Param(name.to_string())),
        Some(name) => Err(format!("`{name}` is not a parameter name")),
        None if piece.contains(['{', '}']) => Err(format!("unmatched brace in `{piece}`")),
        None => Ok(Segment::Literal(piece.to_string())),
    };
    rest.split('/').map(segment).collect()
}

/// The names of a handler's parameters, with the identifier each was written as.
pub fn parameters(handler: &ItemFn) -> syn::Result<Vec<&Ident>> {
    let names = handler.sig.inputs.iter().map(|input| match input {
        syn::FnArg::Typed(typed) => match &*typed.pat {
            syn::Pat::Ident(name) => Ok(&name.ident),
            other => Err(syn::Error::new_spanned(other, "a handler parameter must be a plain name")),
        },
        syn::FnArg::Receiver(receiver) => {
            Err(syn::Error::new_spanned(receiver, "a handler cannot take `self`"))
        }
    });
    names.collect()
}
