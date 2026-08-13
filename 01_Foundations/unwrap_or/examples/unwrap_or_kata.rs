//! Kata solution: the default that was built for nothing.
//!
//!   rustc --edition 2024 unwrap_or_kata.rs -o /tmp/uok && /tmp/uok

/// Stands in for a default that costs something — a file read, a query, an
/// allocation. The print is how you can see it happen.
fn default_width() -> usize {
    println!("      …reading the default width from config");
    72
}

fn main() {
    let configured: Option<usize> = Some(100);

    println!("unwrap_or, when the value was there all along:");
    let w = configured.unwrap_or(default_width());
    println!("  width -> {w}");
    println!("      The argument is an ordinary argument, so Rust built it before");
    println!("      the call and then threw it away. Nothing in the answer shows it.");

    println!("\nunwrap_or_else, same value:");
    let w = configured.unwrap_or_else(default_width);
    println!("  width -> {w}");
    println!("      The closure was never called.");

    println!("\nAnd when the value really is missing, both do the same work:");
    let missing: Option<usize> = None;
    println!("  width -> {}", missing.unwrap_or_else(default_width));

    // ── The other half: unwrap_or takes `self`, so it eats the Option.
    let title: Option<String> = Some("Springfield 2026".to_string());

    // `title.unwrap_or(...)` here would move `title`, and the next line would
    // not compile. as_deref borrows instead: Option<String> -> Option<&str>.
    let shown: &str = title.as_deref().unwrap_or("(untitled)");
    println!("\nBorrowing instead of consuming:");
    println!("  shown -> {shown}");
    println!("  title is still here -> {title:?}");
}
