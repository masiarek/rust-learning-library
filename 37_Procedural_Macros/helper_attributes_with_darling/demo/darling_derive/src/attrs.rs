//! Reading `#[debug(...)]` with darling: declare what the attribute may say
//! as two structs, and derive the code that reads it.

use darling::util::{Flag, Ignored};
use darling::{FromDeriveInput, FromField, ast};
use syn::Ident;

/// What `#[debug(...)]` on the struct asked for.
#[derive(FromDeriveInput)]
#[darling(attributes(debug), supports(struct_named))]
pub struct Container {
    /// `ident` and `data` are filled from the item, not from the attribute.
    pub ident: Ident,
    pub data: ast::Data<Ignored, Field>,
    pub rename: Option<String>,
    #[darling(default = || "***".to_owned())]
    pub mask: String,
}

/// What `#[debug(...)]` on one field asked for.
#[derive(FromField)]
#[darling(attributes(debug))]
pub struct Field {
    pub ident: Option<Ident>,
    pub rename: Option<String>,
    pub skip: Flag,
    pub redact: Flag,
}
