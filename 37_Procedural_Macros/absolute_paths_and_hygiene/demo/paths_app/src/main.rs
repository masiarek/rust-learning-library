//! All three hazards at once, under the fixed derive: no `use std::fmt`, a
//! `Result` of its own, and a module named `core`.

use by_name_derive::ByName;

/// The application's core logic, in a module named for it.
mod core {
    pub fn is_safe(method: &super::Method) -> bool {
        matches!(method, super::Method::Get)
    }
}

/// This crate's own `Result`, with the error type every function here returns.
type Result<T> = std::result::Result<T, &'static str>;

#[derive(ByName)]
enum Method {
    Get,
    Post,
    Delete,
}

fn parse(text: &str) -> Result<Method> {
    text.parse()
}

fn main() {
    for text in ["Get", "Delete", "PATCH"] {
        match parse(text) {
            Ok(method) => println!("{text:6} -> {method}, safe: {}", core::is_safe(&method)),
            Err(e) => println!("{text:6} -> error: {e}"),
        }
    }
}
