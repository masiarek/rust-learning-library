//! Kata solution: the receipt is caught when you keep it, not when you print it.
//!
//!   rustc --edition 2024 the_silent_receipt_kata.rs -o /tmp/tsrk && /tmp/tsrk

fn main() {
    println!("1. The call that printed its own receipt");
    let mut a = ['x', 'c', 'z'];
    println!("   println!(\"{{:?}}\", a.reverse())  prints  {:?}", a.reverse());
    println!("   and a is now {a:?} — it DID reverse. You printed the receipt.");
    println!("   reverse() writes the answer back into the receiver and hands");
    println!("   you (), the unit value. Nothing was lost; nothing was returned.");

    println!();
    println!("2. Three ways to make the same mistake, and who catches each");
    println!("   a.reverse().len()      rustc, immediately:");
    println!("     error[E0599]: no method named `len` found for unit type `()`");
    println!("   let x = a.reverse();   clippy, not rustc:");
    println!("     warning: this let-binding has unit value [let_unit_value]");
    println!("   println!(\"{{:?}}\", a.reverse())   NOBODY.");
    println!("   Verified on rustc 1.98.0: silent under -W warnings, and silent");
    println!("   under clippy::all + pedantic + nursery. The lint is on the LET,");
    println!("   so calling inside the println! steps around the one tool that");
    println!("   would have told you.");

    println!();
    println!("3. Why it prints instead of failing — and why an ARRAY makes it likelier");
    println!("   () implements Debug, so {{:?}} accepts it. It does NOT implement");
    println!("   Display, so println!(\"{{}}\", a.reverse()) is E0277 and safe:");
    println!("     error[E0277]: `()` doesn\'t implement `std::fmt::Display`");
    println!("     note: in format strings you may be able to use `{{:?}}` instead");
    println!("   Read that note again. The one diagnostic standing between you");
    println!("   and the silent version RECOMMENDS the silent version — it is");
    println!("   answering a formatting question, and it is right about that.");
    println!("   An array does not implement Display either, so printing one");
    println!("   REQUIRES {{:?}} anyway. The formatter a beginner is pushed into");
    println!("   from both directions is the one that swallows the unit value.");

    println!();
    println!("4. Nor is #[must_use] the missing guard");
    println!("   must_use fires when a return value is DISCARDED. Here it was not");
    println!("   discarded — it was printed, and the real answer went into the");
    println!("   receiver. There is no value being ignored for a lint to notice.");

    println!();
    println!("5. The three right answers");
    let mut b = ['x', 'c', 'z'];
    b.reverse();
    println!("   mutate, then look:  b.reverse(); b -> {b:?}");

    let c = ['x', 'c', 'z'];
    let backwards: Vec<char> = c.iter().rev().copied().collect();
    println!("   ask for a new one:  c.iter().rev() -> {backwards:?}, c still {c:?}");

    let original = ['x', 'c', 'z'];
    let mut snapshot = original;
    snapshot.reverse();
    println!("   copy, then mutate:  snapshot {snapshot:?}, original {original:?}");
    println!("   That third one is an ARRAY privilege: [char; 3] is Copy, so");
    println!("   `let mut snapshot = original;` duplicates all three elements and");
    println!("   the original keeps its name. A Vec is not Copy — the same line");
    println!("   MOVES it, and you need .clone() to get the snapshot back.");

    println!();
    println!("6. The same split, named twice in std");
    println!("   in place, returns ()      returns a new value");
    println!("   ---------------------     -----------------------------");
    println!("   slice::reverse            Iterator::rev");
    println!("   slice::sort               (collect the sorted iterator)");
    println!("   str::make_ascii_uppercase str::to_ascii_uppercase");
    println!("   Vec::push / clear / dedup Vec::iter().filter().collect()");
    println!("   The naming is the tell: an imperative verb mutates, and the");
    println!("   to_/into_/iter_ forms hand something back.");
}
