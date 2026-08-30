//! Kata solution: two rulers over one string — bytes and chars, and the two
//! indices that look alike until a crab turns up.
//!
//!   rustc --edition 2024 two_rulers_kata.rs -o /tmp/trk && /tmp/trk

/// Both answers to "how long is it", returned together so neither can be
/// mistaken for the other.
fn lengths(s: &str) -> (usize, usize) {
    (s.len(), s.chars().count())
}

/// Every second character, starting with the first.
fn alternate(s: &str) -> String {
    s.chars().step_by(2).collect()
}

/// How many raw bytes are outside ASCII — i.e. part of a multibyte character.
fn non_ascii_bytes(s: &str) -> usize {
    s.bytes().filter(|&b| b > 127).count()
}

fn main() {
    let s = "Hello 🦀!";

    println!("1. Two lengths, one string");
    let (bytes, chars) = lengths(s);
    println!("   {s:?}");
    println!("   len()          {bytes:>2}   <- bytes, what the heap holds");
    println!("   chars().count() {chars:>1}   <- Unicode scalars, what you meant");
    println!("   The gap is one crab: 🦀 is 4 bytes and 1 char.");

    println!("\n2. Every second character");
    println!("   alternate({s:?})      = {:?}", alternate(s));
    println!("   alternate(\"abcdefgh\")     = {:?}", alternate("abcdefgh"));
    println!("   step_by(2) counts *characters*, so the crab is never cut in half.");

    println!("\n3. Bytes above 127");
    for t in [s, "plain ascii", "café", "こんにちは"] {
        println!("   {:>2} of {:>2} bytes are non-ASCII   {t:?}", non_ascii_bytes(t), t.len());
    }
    println!("   A byte > 127 is never a whole character on its own: it is a lead");
    println!("   byte or a continuation byte of a 2-, 3- or 4-byte sequence.");

    println!("\n4. chars().enumerate() — the counter runs 0,1,2,…");
    for (i, c) in s.chars().enumerate() {
        println!("   {i}  {c:?}");
    }

    println!("\n5. char_indices() — the index is a BYTE offset");
    for (i, c) in s.char_indices() {
        println!("   {i:>2}  {c:?}  ({} byte{})", c.len_utf8(), if c.len_utf8() == 1 { "" } else { "s" });
    }

    println!("\nThe two side by side:");
    println!("   {:>9}  {:>12}  char", "enumerate", "char_indices");
    for ((n, c), (b, _)) in s.chars().enumerate().zip(s.char_indices()) {
        println!("   {n:>9}  {b:>12}  {c:?}");
    }
    println!("   They agree until the first multibyte character and never again.");
    println!("   Only the char_indices number can be handed to &s[..i]; the");
    println!("   enumerate number is a position in a sequence, not an offset.");
}
