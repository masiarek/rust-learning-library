//! Reading `#[debug(...)]` by hand: pick our attributes out of `attrs`, walk
//! each one with `parse_nested_meta`, and refuse any key we do not know.

use syn::{Attribute, Data, DeriveInput, Error, Fields, Ident, LitStr, Result};

/// What `#[debug(...)]` on the struct asked for.
pub struct Container {
    pub ident: Ident,
    pub rename: Option<String>,
    pub mask: String,
    pub fields: Vec<Field>,
}

/// What `#[debug(...)]` on one field asked for.
pub struct Field {
    pub ident: Ident,
    pub rename: Option<String>,
    pub skip: bool,
    pub redact: bool,
}

impl Container {
    pub fn parse(input: &DeriveInput) -> Result<Self> {
        let mut rename = None;
        let mut mask = "***".to_owned();
        for attr in debug_attrs(&input.attrs) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    rename = Some(meta.value()?.parse::<LitStr>()?.value());
                } else if meta.path.is_ident("mask") {
                    mask = meta.value()?.parse::<LitStr>()?.value();
                } else {
                    return Err(meta.error("unknown key: expected `rename` or `mask`"));
                }
                Ok(())
            })?;
        }
        let Data::Struct(data) = &input.data else {
            return Err(Error::new_spanned(
                &input.ident,
                "CustomDebug needs a struct",
            ));
        };
        let Fields::Named(named) = &data.fields else {
            return Err(Error::new_spanned(
                &data.fields,
                "CustomDebug needs named fields",
            ));
        };
        let fields = named
            .named
            .iter()
            .map(Field::parse)
            .collect::<Result<_>>()?;
        let ident = input.ident.clone();
        Ok(Container {
            ident,
            rename,
            mask,
            fields,
        })
    }
}

impl Field {
    fn parse(field: &syn::Field) -> Result<Self> {
        let ident = field.ident.clone().expect("named fields have names");
        let (mut rename, mut skip, mut redact) = (None, false, false);
        for attr in debug_attrs(&field.attrs) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    rename = Some(meta.value()?.parse::<LitStr>()?.value());
                } else if meta.path.is_ident("skip") {
                    skip = true;
                } else if meta.path.is_ident("redact") {
                    redact = true;
                } else {
                    return Err(meta.error("unknown key: expected `rename`, `skip` or `redact`"));
                }
                Ok(())
            })?;
        }
        Ok(Field {
            ident,
            rename,
            skip,
            redact,
        })
    }
}

/// Only `#[debug(...)]`. The same list holds doc comments, `#[allow]`, and the
/// helper attributes of every other derive on the struct.
fn debug_attrs(attrs: &[Attribute]) -> impl Iterator<Item = &Attribute> {
    attrs.iter().filter(|attr| attr.path().is_ident("debug"))
}
