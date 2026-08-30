//! Kata solution: the `Vec` that could not outlive its string.
//!
//!   rustc --edition 2024 collect_into_a_vec_kata.rs -o /tmp/civk && /tmp/civk
//!
//! The version that does not compile, kept as a comment so the file still does:
//!
//!     fn tidy(raw: &str) -> Vec<&str> {
//!         let lower = raw.to_lowercase();             // a NEW String, owned here
//!         lower.split(':').map(str::trim).collect()   // E0515
//!     }
//!
//! error[E0515]: cannot return value referencing local variable `lower`
//!   returns a value referencing data owned by the current function

use std::borrow::Cow;

/// Fix 1 — own the pieces. One allocation per piece, and the caller is free.
fn tidy_owned(raw: &str) -> Vec<String> {
    raw.to_lowercase()
        .split(':')
        .map(|p| p.trim().to_string())
        .collect()
}

/// Fix 2 — make the caller own the lowercase copy, so the borrow has something to point at.
fn tidy_borrowed(lower: &str) -> Vec<&str> {
    lower.split(':').map(str::trim).collect()
}

/// Fix 3 — do only the transformations that hand back a piece of the input.
fn tidy_trimmed(raw: &str) -> Vec<&str> {
    raw.split(':').map(str::trim).collect()
}

/// Fix 4 — borrow when you can, own when you must.
fn tidy_cow(raw: &str) -> Vec<Cow<'_, str>> {
    raw.split(':')
        .map(str::trim)
        .map(|p| {
            if p.bytes().any(|b| b.is_ascii_uppercase()) {
                Cow::Owned(p.to_lowercase())
            } else {
                Cow::Borrowed(p)
            }
        })
        .collect()
}

fn main() {
    let raw = "Cara : ada : Ben";

    println!("The four fixes, on {raw:?}");
    println!();

    let owned = tidy_owned(raw);
    println!("1. Vec<String>      {owned:?}");
    println!("   Allocates the lowercase copy, one String per piece, and the Vec.");
    println!("   Works for any transformation, and the result borrows nothing, so");
    println!("   it outlives everything it was made from.");
    println!();

    let lower = raw.to_lowercase();
    let borrowed = tidy_borrowed(&lower);
    println!("2. Vec<&str>, caller owns the lowercase copy");
    println!("   {borrowed:?}");
    println!("   Allocates one String at the call site, plus the Vec. The body of");
    println!("   the function did not change at all: what changed is WHERE the");
    println!("   lowercase copy lives, and now it outlives the call.");
    println!();

    let trimmed = tidy_trimmed(raw);
    println!("3. Vec<&str>, no new text at all");
    println!("   {trimmed:?}");
    println!("   Allocates the Vec and nothing else — no character was copied.");
    println!("   `trim` hands back a slice OF its argument, which is the only kind");
    println!("   of transformation a borrowing pipeline can do: `to_lowercase`,");
    println!("   `to_uppercase` and `replace` each build a new String, and that");
    println!("   String has to belong to somebody.");
    println!();

    let mixed = tidy_cow(raw);
    let borrowed_pieces = mixed.iter().filter(|c| matches!(c, Cow::Borrowed(_))).count();
    println!("4. Vec<Cow<str>>    {mixed:?}");
    println!("   {borrowed_pieces} of the {} pieces borrowed. Only a piece that really held a", mixed.len());
    println!("   capital letter paid for a String — which is what `Cow` is for when");
    println!("   most of the input is already the thing you wanted.");
    println!();

    println!("The rule underneath all four:");
    println!("  a Vec<&str> is a Vec of borrows, so it is only ever as long-lived");
    println!("  as the text it points into. Ask where that text lives, and the");
    println!("  choice between &str and String makes itself.");
}
