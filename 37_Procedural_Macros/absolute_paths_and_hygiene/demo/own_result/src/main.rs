use by_name_derive::ByNameAsWritten;
use std::fmt;
use std::str::FromStr;

/// This crate's own `Result`, with the error type every function here returns.
type Result<T> = std::result::Result<T, &'static str>;

#[derive(ByNameAsWritten)]
enum Method {
    Get,
    Post,
}

fn parse(text: &str) -> Result<Method> {
    Method::from_str(text)
}

fn main() {
    println!("{} {}", Method::Get, parse("Post").unwrap());
}
