//! Kata solution: `fn trimmed(s: &str) -> str` is refused. Three signatures
//! that compile, and what each one hands back.
//!
//! Every answer is a size in words or a comparison of addresses, never an
//! address, so the answer key is the same on every run and every target.
//!
//!   rustc --edition 2024 returned_by_value_kata.rs -o /tmp/rbvk && /tmp/rbvk

use std::mem::size_of;

fn words<T>() -> usize {
    size_of::<T>() / size_of::<usize>()
}

/// Borrows: the result is a window onto the caller's text.
fn trimmed_borrowed(s: &str) -> &str {
    s.trim()
}

/// Owns, with room to grow: the text is copied into a new buffer.
fn trimmed_string(s: &str) -> String {
    s.trim().to_string()
}

/// Owns, fixed length: the text is copied, and the capacity word is gone.
fn trimmed_boxed(s: &str) -> Box<str> {
    s.trim().into()
}

/// Does `part` start inside `whole`'s bytes?
fn points_into(part: &str, whole: &str) -> bool {
    whole.as_bytes().as_ptr_range().contains(&part.as_ptr())
}

fn main() {
    let input = String::from("   hello   ");

    let a = trimmed_borrowed(&input);
    let b = trimmed_string(&input);
    let c = trimmed_boxed(&input);

    println!("input {input:?}");
    println!("   -> &str      {} words  {a:?}  points into the input: {}", words::<&str>(), points_into(a, &input));
    println!("   -> String    {} words  {b:?}  points into the input: {}", words::<String>(), points_into(&b, &input));
    println!("   -> Box<str>  {} words  {c:?}  points into the input: {}", words::<Box<str>>(), points_into(&c, &input));

    drop(input);
    println!("after drop(input), b and c still work: {b} {c}");
    // println!("{a}");  // E0505: cannot move out of `input` because it is borrowed
}
