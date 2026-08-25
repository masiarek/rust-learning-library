//! What a program can tell you about how it was built.
//!
//!   rustc --edition 2024 rustc_without_cargo.rs -o /tmp/rwc && /tmp/rwc
//!
//! Every fact below is decided at COMPILE time, by that command line rather
//! than by anything in this file. Compile the same source with different
//! flags and the program says different things about itself.

use std::collections::HashMap;

fn banner(n: u8, title: &str) {
    println!("\n──── Step {n}: {title}");
}

/// Takes its input as an argument, so the arithmetic happens at run time.
/// Written `+`, an overflow here would be a compile error instead — the
/// `arithmetic_overflow` lint catches whatever it can const-evaluate.
fn bump(v: u8) -> u8 {
    v.wrapping_add(1)
}

fn main() {
    banner(1, "Who built me?");
    match option_env!("CARGO_PKG_NAME") {
        Some(name) => println!("  Cargo built this: package {name:?}"),
        None => println!("  No Cargo here — CARGO_PKG_NAME was never set."),
    }
    println!("      `option_env!` asks at COMPILE time, and answers None when the");
    println!("      variable is absent. Its cousin `env!` is a hard error instead:");
    println!("      'environment variable `CARGO_PKG_NAME` not defined at compile");
    println!("      time'. A crate that reads its own version that way cannot be");
    println!("      built by rustc alone — setting those is Cargo's job.");

    banner(2, "Which edition?");
    let scores: Vec<u8> = vec![5, 3, 0];
    if let Some(n) = scores.first()
        && *n > 4
    {
        println!("  A let chain ran, so this was built as edition 2024: top = {n}");
    }
    println!("      That `if let … && …` head is the proof. rustc's DEFAULT edition");
    println!("      is 2015, not the current one, and under it this file does not");
    println!("      compile: 'let chains are only allowed in Rust 2024 or later'.");
    println!("      Cargo passes the edition from Cargo.toml every time; by hand");
    println!("      you pass it yourself, which is why every command in this repo");
    println!("      reads `rustc --edition 2024`.");

    banner(3, "Which profile?");
    println!("  cfg!(debug_assertions) = {}", cfg!(debug_assertions));
    let edge: u8 = 255;
    println!("  bump({edge}) = {}", bump(edge));
    println!("  {edge}u8.checked_add(1) = {:?}", edge.checked_add(1));
    println!("      Plain `rustc` is the debug profile: overflow checks ON. Add -O");
    println!("      and cfg!(debug_assertions) turns false — and a plain `v + 1`");
    println!("      that panicked here ('attempt to add with overflow', exit 101)");
    println!("      wraps quietly to 0 there. The flag you forgot changed the");
    println!("      program's BEHAVIOUR, not just its speed. `cargo run` gives you");
    println!("      the first and `cargo run --release` the second, under names");
    println!("      that are harder to forget.");

    banner(4, "What you gave up");
    let mut seats: HashMap<&str, u32> = HashMap::new();
    seats.insert("Ada", 2);
    println!("  std is all here: seats[\"Ada\"] = {:?}", seats.get("Ada"));
    println!("      All of std, and nothing else. `use rand::…` means fetching that");
    println!("      crate, building it, and passing --extern rand=librand.rlib by");
    println!("      hand — for it, and for every dependency of its own. That is the");
    println!("      wall, and clearing it is the one thing Cargo exists to do.");
    println!("      Tests too: #[test] functions are compiled out of this binary");
    println!("      entirely. `rustc --test` builds the harness instead of main.");
}
