//! A `Result` is one of two shapes: `Ok` with the value, or `Err` with the
//! reason. `parse` is the first function most people meet that returns one.
//! The page is 17_Option_and_Result/ok_and_err/README.md.
//!
//!   rustc --edition 2024 ok_and_err.rs -o /tmp/t && /tmp/t

use std::num::ParseFloatError;

/// Reads one field of a CSV row, the way the loader in 28_Testing does,
/// but hands the failure back instead of panicking.
fn read_field(text: &str) -> Result<f32, ParseFloatError> {
    text.trim().parse()
}

fn main() {
    println!("1. What parse hands back");
    for text in ["1", "-1.5", " 1 ", "x", ""] {
        let result = read_field(text);
        println!("   read_field({:<6}) = {result:?}", format!("{text:?}"));
    }

    println!();
    println!("2. match reads both shapes, and must");
    for text in ["-1", "yes"] {
        match read_field(text) {
            Ok(value) => println!("   {text:?}: Ok, the value is {value}"),
            Err(error) => println!("   {text:?}: Err, because {error}"),
        }
    }

    println!();
    println!("3. The error is a value too: Debug for a developer, Display for a user");
    let error = read_field("x").unwrap_err();
    println!("   {{:?}} -> {error:?}");
    println!("   {{}}   -> {error}");

    println!();
    println!("4. Asking without taking the value out");
    let good = read_field("1");
    let bad = read_field("x");
    println!("   good.is_ok() = {}, bad.is_err() = {}", good.is_ok(), bad.is_err());
    println!("   good.ok()    = {:?}, bad.ok() = {:?}   <- Result to Option, error dropped", good.ok(), bad.ok());
}
