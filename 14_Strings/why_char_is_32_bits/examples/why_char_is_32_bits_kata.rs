//! Kata solution: rebuild `char::from_u32` from the definition, count the exact
//! holes in the range, then price the fixed width against UTF-8.
//!
//!   rustc --edition 2024 why_char_is_32_bits_kata.rs -o /tmp/wc32k && /tmp/wc32k

use std::mem::size_of;

/// A Unicode *scalar value*: a code point at or below U+10FFFF that is not one
/// half of a UTF-16 surrogate pair. That sentence is the whole definition of
/// what a `char` may hold.
fn is_scalar(n: u32) -> bool {
    n <= 0x10FFFF && !(0xD800..=0xDFFF).contains(&n)
}

fn main() {
    println!("Round 1 — agree with the standard library, then count the holes");
    let probe = 0..=0x11_FFFFu32; // a little past the top, to catch an off-by-one
    let disagreements = probe.clone().filter(|&n| is_scalar(n) != char::from_u32(n).is_some()).count();
    println!("   probed {} values, disagreements with char::from_u32: {disagreements}", probe.clone().count());

    let accepted = probe.clone().filter(|&n| is_scalar(n)).count();
    let surrogates = (0..=0x10FFFFu32).filter(|&n| !is_scalar(n)).count();
    println!("   accepted            {accepted}");
    println!("   0x110000 - accepted {}", 0x110000 - accepted);
    println!("   refused inside the range (the surrogates) {surrogates} = 0x{surrogates:X}");
    println!("   The hole is exactly D800..=DFFF, which UTF-16 reserved to encode");
    println!("   everything above U+FFFF. UTF-8 needs no such reservation and pays");
    println!("   for the hole anyway, because the hole is in Unicode, not in UTF-16.");

    println!("\nRound 2 — the price of a fixed width");
    println!("   {:>5}  {:>5}  {:>6}  {:>5}   text", "chars", "UTF-8", "[char]", "ratio");
    for s in ["plain ascii", "zażółć gęślą jaźń", "日本語のテキスト", "🦀🦀🦀"] {
        let n = s.chars().count();
        let utf8 = s.len();
        let fixed = n * size_of::<char>();
        println!("   {n:>5}  {utf8:>5}  {fixed:>6}  {:>4.1}x   {s:?}", fixed as f64 / utf8 as f64);
    }
    println!("   The ratio is worst for the text that never needed the range: ASCII");
    println!("   pays 4x, and the emoji that genuinely needs 21 bits pays 1.0x. A");
    println!("   fixed width taxes the common case to make the rare case uniform.");

    println!("\nRound 3 — where the 11 spare bits went");
    println!("   size_of::<char>()                 = {}", size_of::<char>());
    println!("   size_of::<Option<char>>()         = {}", size_of::<Option<char>>());
    println!("   size_of::<Option<Option<char>>>() = {}", size_of::<Option<Option<char>>>());
    println!("   size_of::<Result<char, ()>>()     = {}", size_of::<Result<char, ()>>());
    println!("   size_of::<u32>()                  = {}", size_of::<u32>());
    println!("   size_of::<Option<u32>>()          = {}", size_of::<Option<u32>>());
    let niches = (u32::MAX as u64 + 1) - accepted as u64;
    println!("   Every invalid bit pattern is a niche the compiler can spend on a");
    println!("   discriminant. char has {niches} of them; u32 has none, so the");
    println!("   same Option doubles its size.");
}
