//! Kata solution: the same program, with a type that is not `Copy`.
//!
//! The Step 1 program works on an `Option<i32>` for one reason: `i32` is `Copy`,
//! so `if let Some(n) = opt` duplicates the value and leaves `opt` alone. Swap in
//! a `String` and the same shape MOVES, so the line after the block is:
//!
//!     error[E0382]: use of partially moved value: `maybe_name`
//!       |
//!     4 |     if let Some(name) = maybe_name {
//!       |                 ---- value partially moved here
//!     9 |     let shout = maybe_name.unwrap_or_default();
//!       |                 ^^^^^^^^^^ value used here after partial move
//!       |
//!       = note: partial move occurs because value has type `String`, which does
//!               not implement the `Copy` trait
//!
//! Four ways to make it compile, and one that is not a fix at all but is often
//! the right answer anyway.
//!
//!   rustc --edition 2024 shadowing_and_unwrap_kata.rs -o /tmp/sauk && /tmp/sauk

fn banner(n: u32, title: &str) {
    println!("\n──── Fix {n}: {title}");
}

fn main() {
    // ─────────────────────────────────────────────────── 1
    banner(1, "Borrow the option: `&maybe_name`");
    let maybe_name: Option<String> = Some("Ada".to_string());

    if let Some(name) = &maybe_name {
        // `name` is a &String — match ergonomics borrowed it for us.
        let name = name.to_uppercase(); // a shadow, and a NEW String
        println!("  shouted   -> {name}");
    }
    println!("  original  -> {maybe_name:?}   still here");

    // ─────────────────────────────────────────────────── 2
    banner(2, "Borrow inside the pattern: `Some(ref name)`");
    let maybe_name: Option<String> = Some("Ada".to_string());

    if let Some(ref name) = maybe_name {
        println!("  borrowed  -> {name}");
    }
    println!("  original  -> {maybe_name:?}   the older spelling of fix 1");

    // ─────────────────────────────────────────────────── 3
    banner(3, "Change the option instead: `.as_ref()` / `.as_deref()`");
    let maybe_name: Option<String> = Some("Ada".to_string());

    let borrowed: Option<&String> = maybe_name.as_ref();
    let as_str: Option<&str> = maybe_name.as_deref();
    println!("  as_ref()  -> {borrowed:?}");
    println!("  as_deref()-> {as_str:?}   Option<&str>, which is what most fns want");
    println!("  length    -> {:?}", maybe_name.as_ref().map(|s| s.len()));
    println!("  original  -> {maybe_name:?}");

    // ─────────────────────────────────────────────────── 4
    banner(4, "Clone it: correct, and the one to justify");
    let maybe_name: Option<String> = Some("Ada".to_string());

    let owned = maybe_name.clone().unwrap_or_default();
    println!("  cloned    -> {owned:?}   a second allocation of the same bytes");
    println!("  original  -> {maybe_name:?}");
    println!("      Reach for this when you need a value that OUTLIVES the option.");
    println!("      Reaching for it to quiet E0382 is how a borrow bug becomes a");
    println!("      performance one — the compiler was asking a question, not");
    println!("      objecting.");

    // ─────────────────────────────────────────────────── 5
    banner(5, "Not a fix: move it on purpose, and put it last");
    let maybe_name: Option<String> = Some("Ada".to_string());

    println!("  before    -> {maybe_name:?}");
    if let Some(name) = maybe_name {
        // This CONSUMES the option. Nothing below reads it, so nothing complains.
        let name = format!("{name} the First");
        println!("  moved     -> {name}");
    }
    // println!("{maybe_name:?}");   // <- uncomment for the E0382 in the header

    println!("\n      Fix 1 is the one to ship: no allocation, and the option is");
    println!("      untouched. But read the error before reaching for any of them —");
    println!("      'use after partial move' is a question about how long you need");
    println!("      the value, and sometimes the honest answer is fix 5: you were");
    println!("      finished with it, and the line order was the only problem.");
}
