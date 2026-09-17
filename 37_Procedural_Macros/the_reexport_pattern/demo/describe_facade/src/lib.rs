//! The crate users depend on. It holds the trait, re-exports the derive under
//! the same name, and keeps a hidden module for the code the derive generates.

/// A one-line description of a value, field by field.
pub trait Describe {
    fn describe(&self) -> String;
}

/// The derive macro. It lives in `describe_facade_derive`, because a
/// proc-macro crate can export nothing but macros; this line makes it
/// `describe_facade::Describe` too.
pub use describe_facade_derive::Describe;

/// Used by code that `#[derive(Describe)]` generates, which runs in the user's
/// crate and so can only call what is public. Not part of the API.
#[doc(hidden)]
pub mod __private {
    use std::fmt::Debug;

    pub fn describe_fields(name: &str, fields: &[(&str, &dyn Debug)]) -> String {
        let fields: Vec<String> = fields.iter().map(|(field, value)| format!("{field} = {value:?}")).collect();
        format!("{name}: {}", fields.join(", "))
    }
}
