//! Kata solution: one name, three lengths — a per-char inventory, and the
//! accent that makes two equal-looking strings unequal.
//!
//!   rustc --edition 2024 meet_the_char_kata.rs -o /tmp/mtck && /tmp/mtck

/// One row per char: byte offset, the char, its UTF-8 width, its code point.
fn inventory(label: &str, s: &str) {
    println!("   {label} = {s:?}");
    for (offset, c) in s.char_indices() {
        println!("      byte {offset}: {c:?}   {} byte(s)   U+{:04X}", c.len_utf8(), c as u32);
    }
    println!("      -> {} bytes, {} chars", s.len(), s.chars().count());
}

fn main() {
    println!("Round 1 — the inventory");
    inventory("city", "Łódź");

    println!("\nRound 2 — the same-looking name, spelled two ways");
    let composed = "Zoé"; // é as one char, U+00E9
    let decomposed = "Zoe\u{301}"; // e + combining acute, two chars
    inventory("composed", composed);
    inventory("decomposed", decomposed);
    println!("   composed == decomposed?  {}", composed == decomposed);

    println!("\nRound 3 — which counts can std give you?");
    println!("   bytes:     .len()            -> {} vs {}", composed.len(), decomposed.len());
    println!("   chars:     .chars().count()  -> {} vs {}", composed.chars().count(), decomposed.chars().count());
    println!("   graphemes: what a reader sees -> 3 vs 3, but std cannot count");
    println!("              these; the unicode-segmentation crate can");

    println!("\nRound 4 — safe slicing needs a boundary");
    let city = "Łódź";
    println!("   city.get(0..1) = {:?}   <- inside 'Ł', refused as None", city.get(0..1));
    println!("   city.get(0..2) = {:?}", city.get(0..2));
    let first = city.chars().next();
    println!("   city.chars().next() = {first:?}   <- the honest way to ask for one char");
}
