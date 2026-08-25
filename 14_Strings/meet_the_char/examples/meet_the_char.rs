//! A char is one Unicode scalar value, four bytes wide. Inside a String the
//! same character is one to four UTF-8 bytes — so "how long" has three answers.
//!
//!   rustc --edition 2024 meet_the_char.rs -o /tmp/mtc && /tmp/mtc

use std::mem::size_of;

fn main() {
    let name = "Zoë";

    println!("1. One string, two counts");
    println!("   {name:?}.len()           = {} bytes", name.len()); // 4
    println!("   {name:?}.chars().count() = {} chars", name.chars().count()); // 3

    println!("\n2. Where each char sits, and what it costs in the string");
    for (offset, c) in name.char_indices() {
        println!("   byte {offset}: {c:?}   {} byte(s) in UTF-8, U+{:04X}", c.len_utf8(), c as u32);
    }

    println!("\n3. A char VALUE is always 4 bytes — decoded, not encoded");
    println!("   size_of::<char>() = {} bytes", size_of::<char>()); // 4
    println!("   'ë' costs 4 bytes as a char, 2 bytes inside a String");

    println!("\n4. Slicing is by byte, and only at char boundaries");
    println!("   name.get(0..1) = {:?}", name.get(0..1)); // Some("Z")
    println!("   name.get(2..4) = {:?}", name.get(2..4)); // Some("ë")
    println!("   name.get(2..3) = {:?}      <- byte 3 is the middle of 'ë'", name.get(2..3)); // None
    println!("   name.is_char_boundary(3) = {}", name.is_char_boundary(3)); // false

    println!("\n5. Case is not one-to-one");
    let upper: String = 'ß'.to_uppercase().collect();
    println!("   'ß'.to_uppercase() = {upper:?}   <- one char in, two out");

    println!("\n6. Two spellings that look identical");
    let composed = "é"; // one char: U+00E9
    let decomposed = "e\u{301}"; // two chars: 'e' + combining acute
    println!("   composed   {composed:?}  {} char(s), {} bytes", composed.chars().count(), composed.len());
    println!("   decomposed {decomposed:?}  {} char(s), {} bytes", decomposed.chars().count(), decomposed.len());
    println!("   composed == decomposed?  {}", composed == decomposed); // false
    println!("   what a READER calls one character is a third counting — the");
    println!("   grapheme — and std stops before it; that one needs a crate");
}
