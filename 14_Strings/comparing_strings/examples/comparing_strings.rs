//! `==` and `<` on a string compare UTF-8 bytes. That is fast, total and
//! reproducible — and it is not what a human means by "the same word" or
//! "alphabetical order".
//!
//!   rustc --edition 2024 comparing_strings.rs -o /tmp/cs && /tmp/cs

use std::cell::Cell;
use std::cmp::Ordering;

fn main() {
    println!("1. `==` and `<` ask the bytes, and nothing else");
    println!("   \"Zebra\" < \"apple\"  = {}", "Zebra" < "apple"); // true
    println!("   'Z' = {}, 'a' = {}   <- 90 < 97, so the capital wins", 'Z' as u32, 'a' as u32);
    let (a, b) = ("Zebra", "apple");
    println!("   a.cmp(b)                    = {:?}", a.cmp(b)); // Less
    println!("   a.as_bytes().cmp(b.as_bytes()) = {:?}   <- the same call", a.as_bytes().cmp(b.as_bytes()));

    println!("\n2. Lexicographic is not alphabetical");
    let mut names = ["Zawadzki", "zebra", "Adamczyk", "\u{141}ukasiewicz", "Echo", "\u{17C}aba", "\u{e9}clair"];
    names.sort();
    for n in &names {
        println!("   {n}");
    }
    println!("   Three blocks, in this order: capitals, lowercase ASCII, then");
    println!("   everything above U+007F. No alphabet is arranged that way.");

    println!("\n3. ...but it IS code point order, which is worth knowing");
    let mut monotonic = true;
    let mut previous = [0u8; 4];
    let mut previous_len = 0usize;
    for value in 0..=0x10FFFFu32 {
        let Some(c) = char::from_u32(value) else { continue };
        let mut buffer = [0u8; 4];
        let encoded = c.encode_utf8(&mut buffer).len();
        if value > 0 && buffer[..encoded].cmp(&previous[..previous_len]) != Ordering::Greater {
            monotonic = false;
        }
        previous = buffer;
        previous_len = encoded;
    }
    println!("   every scalar value encodes above the one before it: {monotonic}"); // true
    println!("   So sorting bytes and sorting code points give the same order.");
    println!("   That is a property of UTF-8, not of Rust — and it is why the");
    println!("   answer is reproducible everywhere while still being wrong for a reader.");

    println!("\n4. Two ways to ignore case, and what each one misses");
    let rows = [("Content-Type", "content-type"), ("\u{141}\u{d3}D\u{179}", "\u{142}\u{f3}d\u{17a}")];
    println!("   {:<22} {:<22} {:>8} {:>10}", "left", "right", "ascii", "lowercase");
    for (left, right) in rows {
        println!(
            "   {left:<22} {right:<22} {:>8} {:>10}",
            left.eq_ignore_ascii_case(right),
            left.to_lowercase() == right.to_lowercase()
        );
    }
    println!("   eq_ignore_ascii_case folds 26 letters and allocates nothing.");
    println!("   to_lowercase() knows the whole table and allocates two Strings.");

    println!("\n5. Lowercasing is not folding — the German \u{df}");
    let (shouted, written) = ("STRASSE", "stra\u{df}e");
    println!("   {shouted:?}.to_lowercase() = {:?}", shouted.to_lowercase()); // "strasse"
    println!("   {written:?}.to_lowercase() = {:?}", written.to_lowercase()); // "straße"
    println!("   equal after lowercasing?  {}", shouted.to_lowercase() == written.to_lowercase()); // false
    println!("   equal after UPPERcasing?  {}", shouted.to_uppercase() == written.to_uppercase()); // true
    println!("   Case mapping is not symmetric: '\u{df}' uppercases to \"SS\", and");
    println!("   nothing lowercases \"ss\" back to '\u{df}'. Caseless matching wants case");
    println!("   FOLDING, which std does not have.");

    println!("\n6. ...so \"uppercase both sides instead\" is not the fix either");
    println!("   '\u{130}'.to_lowercase() = {:?} ({} chars)", '\u{130}'.to_lowercase().collect::<String>(), '\u{130}'.to_lowercase().count());
    println!("   \"\u{130}\".to_lowercase() == \"i\"  = {}", "\u{130}".to_lowercase() == "i"); // false
    println!("   '\u{131}'.to_uppercase() = {:?}", '\u{131}'.to_uppercase().collect::<String>()); // "I"
    println!("   \"\u{131}\".to_uppercase() == \"I\".to_uppercase() = {}", "\u{131}".to_uppercase() == "I".to_uppercase()); // true
    println!("   Turkish '\u{131}' and 'i' are different letters, and uppercasing merges");
    println!("   them. Case mapping is locale-independent here by design: `str` has");
    println!("   no locale, so it cannot have the Turkish answer or the other one.");

    println!("\n7. Same letter, two spellings");
    let composed = "caf\u{e9}"; // é as U+00E9
    let decomposed = "cafe\u{301}"; // e + COMBINING ACUTE ACCENT
    println!("   {composed} and {decomposed} print the same and are {} bytes vs {}", composed.len(), decomposed.len());
    println!("   composed == decomposed              = {}", composed == decomposed); // false
    println!("   lowercased on both sides            = {}", composed.to_lowercase() == decomposed.to_lowercase()); // false
    println!("   No comparison in std repairs this. It is normalization, and it");
    println!("   belongs to the unicode-normalization crate.");

    println!("\n8. Sorting with a key, and paying for it once");
    let words = ["delta", "Alpha", "charlie", "Bravo", "echo", "Foxtrot", "golf", "Hotel"];
    let calls = Cell::new(0usize);
    let mut once_per_comparison = words;
    once_per_comparison.sort_by_key(|w| {
        calls.set(calls.get() + 1);
        w.to_lowercase()
    });
    println!("   sort_by_key        allocated {} Strings for {} words", calls.get(), words.len());
    let cached = Cell::new(0usize);
    let mut once_per_word = words;
    once_per_word.sort_by_cached_key(|w| {
        cached.set(cached.get() + 1);
        w.to_lowercase()
    });
    println!("   sort_by_cached_key allocated {} Strings for {} words", cached.get(), words.len());
    println!("   both orders: {}", once_per_comparison == once_per_word); // true
    println!("   {once_per_word:?}");
    println!("   Case-insensitive is as far as a key gets you. A key that sorts");
    println!("   \u{141} between L and M is a collation table, and that is a crate.");
}
