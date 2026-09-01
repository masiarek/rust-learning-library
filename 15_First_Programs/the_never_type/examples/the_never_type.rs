//! The never type `!` — the type of an expression that does not finish.
//!
//! `()` is the type with exactly one value. `!` is the type with NONE. No value
//! of it can ever exist, which is not a curiosity: it is what lets `panic!()`
//! sit in a `match` arm next to a `u32`, and it is the difference between an
//! arm that diverges and an arm that merely returns nothing.
//!
//!   rustc --edition 2024 the_never_type.rs -o /tmp/nt && /tmp/nt

use std::convert::Infallible;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

/// A function that never returns. `!` is the only return type that promises it.
fn give_up(reason: &str) -> ! {
    panic!("{reason}");
}

/// The stable stand-in for `Result<T, !>`: an error type with no variants.
fn parse_score(raw: u8) -> Result<u8, Infallible> {
    Ok(raw.min(5))
}

fn main() {
    // ───────────────────────────────────────────────────────────── 1
    banner(1, "Four stable ways to produce a `!`");
    println!("  panic!(..)          the value never arrives");
    println!("  loop {{}}             with no `break`, control never leaves");
    println!("  return / break / continue    control goes somewhere else");
    println!("  std::process::exit(..)       the process is gone");
    println!("      Each is an EXPRESSION whose type is `!`. Not 'returns");
    println!("      nothing' — that is `()`, and it is a value you have.");

    // ───────────────────────────────────────────────────────────── 2
    banner(2, "`!` fits into every type; `()` fits into none");
    let n: u32 = 3;
    let a: u32 = match n {
        0 => give_up("a zero score is not a score"), // type `!`
        k => k,                                      // type u32
    };
    println!("  match n {{ 0 => give_up(..), k => k }}  =>  {a}");
    println!("      The two arms have types `!` and `u32`, and the match");
    println!("      still type-checks as u32 — `!` coerces into anything.");
    println!("      Swap the diverging arm for `println!(\"zero\")` and it stops:");
    println!("        error[E0308]: mismatched types");
    println!("           |     0 => println!(\"zero\"),");
    println!("           |          ^^^^^^^^^^^^^^^^ expected `u32`, found `()`");
    println!("      That error is the whole lesson. `()` is a value of the wrong");
    println!("      type. `!` is no value at all, so there is nothing to mismatch.");

    // ───────────────────────────────────────────────────────────── 3
    banner(3, "A semicolon does NOT turn `!` into `()`");
    println!("  fn f() -> String {{ panic!(); }}    compiles");
    println!("  fn g() -> String {{ loop {{}} }}      compiles");
    println!("      The rule you already know — a trailing semicolon makes a");
    println!("      block evaluate to `()` — has an exception, and this is it.");
    println!("      A block whose control flow never reaches the end has type");
    println!("      `!`, punctuation or not. So the missing-return error you");
    println!("      expect here never appears.");

    // ───────────────────────────────────────────────────────────── 4
    banner(4, "You can RETURN `!`, but you cannot write it anywhere else");
    println!("  fn f() -> ! {{ .. }}          stable since 1.0");
    println!("  fn f() -> Result<(), !> {{}}  NOT stable:");
    println!("        error[E0658]: the `!` type is experimental");
    println!("        note: see issue #35121 for more information");
    println!("  fn f(x: !) {{}}               the same error");
    println!("      So `!` is a promise a signature may make about its own");
    println!("      control flow, and not yet a type you may store, nest or");
    println!("      pass. Every `!` you meet in real code is after an `->`.");

    // ───────────────────────────────────────────────────────────── 5
    banner(5, "`Infallible` is the stable stand-in");
    let score = parse_score(9);
    let value = match score {
        Ok(v) => v,
        Err(e) => match e {}, // zero variants: zero arms is exhaustive
    };
    println!("  fn parse_score(u8) -> Result<u8, Infallible>");
    println!("  parse_score(9) = {value}");
    println!("      `Infallible` is `enum Infallible {{}}` — an ordinary enum with");
    println!("      no variants, so it has no values either, and `match e {{}}`");
    println!("      is EXHAUSTIVE with zero arms. That is how you unwrap a");
    println!("      cannot-fail Result without an `unwrap` that could panic.");
    println!("      It is what `Result<T, !>` would mean, spelled in stable Rust.");

    // ───────────────────────────────────────────────────────────── 6
    banner(6, "Where `!` is already working for you");
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| give_up("caught on purpose"));
    std::panic::set_hook(hook);
    println!("  give_up(..) -> !   ran, and unwound: caught = {}", caught.is_err());
    println!("  unreachable!()  todo!()  unimplemented!()   all have type `!`,");
    println!("      which is why any of them may stand in for a value of any");
    println!("      type while you are still writing the function around it.");
    println!("  let Some(x) = opt else {{ return; }};");
    println!("      `let else` requires the else block to diverge — and");
    println!("      'diverge' is exactly 'has type `!`'.");
}
