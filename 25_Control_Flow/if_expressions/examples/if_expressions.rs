//! `if` expressions.
//!
//! `if` picks one of two blocks, as in every other language, and then it does
//! one more thing: the block that ran hands back its last expression, so the
//! whole `if` has a VALUE and a TYPE. Each section below is one claim from the
//! lesson, printed. The refusals (a non-`bool` condition, branches of two types,
//! a missing `else`) cannot live here, because they do not compile; the lesson
//! shows rustc's real output for each.
//!
//!   rustc --edition 2024 if_expressions.rs -o /tmp/ifx && /tmp/ifx

use std::any::type_name_of_val;

fn banner(title: &str) {
    println!("\n──── {title}");
}

/// The chain from *Rust in Action* §2.4.6, given something to produce.
fn describe(item: i32) -> &'static str {
    if item == 42 {
        "forty-two"
    } else if item == 132 {
        "one hundred thirty-two"
    } else {
        "neither"
    }
}

/// An `if` as a function's tail expression: no `return`, no placeholder.
fn sign(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}

/// Says out loud when it is evaluated, so the output shows which branch ran.
fn loud(label: &str, value: i32) -> i32 {
    println!("  evaluating the {label} branch");
    value
}

fn main() {
    banner("1. The statement form: no parentheses, braces required");
    let n = 7;
    if n < 10 {
        println!("  {n} is a single digit");
    }

    banner("2. if has a value: the last expression of the branch that ran");
    for n in [3, 10, 99] {
        let size = if n < 10 { "small" } else { "large" };
        println!("  n = {n:>2} -> size = {size:?}");
    }

    banner("3. An else-if chain is still ONE expression");
    for item in [42, 132, 7] {
        let label = if item == 42 {
            "forty-two"
        } else if item == 132 {
            "one hundred thirty-two"
        } else {
            "neither"
        };
        println!("  {item:>3} -> {label:?}   (same as describe: {})", describe(item) == label);
    }

    banner("4. A branch can do work before it hands back its value");
    for cents in [1_999u32, 45] {
        let price = if cents >= 100 {
            let dollars = cents / 100;
            let rest = cents % 100;
            format!("${dollars}.{rest:02}")
        } else {
            format!("{cents} cents")
        };
        println!("  {cents:>4} -> {price}");
    }
    println!("  `dollars` and `rest` never escape the branch; only the tail does.");

    banner("5. Only the branch that is taken is evaluated");
    let ready = true;
    let x = if ready { loud("then", 1) } else { loud("else", 2) };
    println!("  x = {x}; nothing was printed for the else branch");

    banner("6. One type for the whole if: the type both branches agree on");
    let a = if ready { 1 } else { 2 };
    let b: u8 = if ready { 1 } else { 2 };
    let s = if ready { "yes" } else { "no" };
    println!("  let a     = if ready {{ 1 }} else {{ 2 }};         -> {}", type_name_of_val(&a));
    println!("  let b: u8 = if ready {{ 1 }} else {{ 2 }};         -> {}", type_name_of_val(&b));
    println!("  let s     = if ready {{ \"yes\" }} else {{ \"no\" }}; -> {}", type_name_of_val(&s));
    println!("  The annotation on b reaches into BOTH branches: neither literal is an i32.");

    banner("7. No else means the value is (), whichever way it goes");
    for quiet in [true, false] {
        let unit = if quiet {
            println!("  quiet = {quiet}: the block ran");
        };
        println!("  quiet = {quiet}: unit = {unit:?}");
    }

    banner("8. A branch that never finishes fits any type");
    for (total, count) in [(10, 2), (7, 0), (9, 3)] {
        let mean = if count == 0 {
            println!("  {total} / {count}: skipped");
            continue; // type `!`, so it fits where the other branch has an i32
        } else {
            total / count
        };
        println!("  {total} / {count} = {mean}");
    }

    banner("9. if goes anywhere a value goes: a tail, an argument");
    for n in [-5, 0, 6] {
        println!(
            "  {n:>2} is {} and {}",
            sign(n),
            if n % 2 == 0 { "even" } else { "odd" }
        );
    }
}
