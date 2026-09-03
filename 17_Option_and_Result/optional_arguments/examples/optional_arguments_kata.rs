//! Kata solution: four ways to make one argument optional.
//!
//!   rustc --edition 2024 optional_arguments_kata.rs -o /tmp/oak && /tmp/oak

/// 1. Two functions. No cleverness, and the common call stays short.
fn banner(title: &str) -> String {
    banner_with_width(title, 24)
}

fn banner_with_width(title: &str, width: usize) -> String {
    format!("{:*^width$}", format!(" {title} "))
}

/// 2. An Option parameter: one function, and every caller says what it means.
fn banner_opt(title: &str, width: Option<usize>) -> String {
    banner_with_width(title, width.unwrap_or(24))
}

/// 3. `Option<&T>`, never `&Option<T>`. This signature accepts a caller who has
///    an owned String, a &str, or nothing — the other one only accepts a caller
///    who already happens to be holding an Option.
fn footer(note: Option<&str>) -> String {
    match note {
        Some(n) => format!("— {n}"),
        None => "—".to_string(),
    }
}

/// 4. `impl Into<Option<T>>`: the call site drops the `Some`. Convenient, and
///    it costs a reader one indirection to work out what may be passed.
fn banner_into(title: &str, width: impl Into<Option<usize>>) -> String {
    banner_with_width(title, width.into().unwrap_or(24))
}

fn main() {
    println!("1. Two functions:");
    println!("   {}", banner("Results"));
    println!("   {}", banner_with_width("Results", 40));

    println!("\n2. An Option parameter — the call site states the absence:");
    println!("   {}", banner_opt("Results", None));
    println!("   {}", banner_opt("Results", Some(40)));

    println!("\n3. Option<&T> takes callers the other shape cannot:");
    let owned = String::from("461 rows");
    println!("   {}", footer(Some(&owned)));
    println!("   {}", footer(Some("unofficial")));
    println!("   {}", footer(None));
    println!("      A `&Option<String>` parameter would have rejected the middle");
    println!("      line, and forced the caller to build an Option to say nothing.");

    println!("\n4. impl Into<Option<usize>> — no Some at the call site:");
    println!("   {}", banner_into("Results", 40));
    println!("   {}", banner_into("Results", None));
}
