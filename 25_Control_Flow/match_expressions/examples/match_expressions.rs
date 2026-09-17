//! `match` expressions.
//!
//! `match` compares one value against a list of arms, top to bottom, and runs
//! the FIRST arm that fits — exactly one arm, never the next one as well. The
//! list must cover every value the type can hold, and the whole `match` has a
//! value. Each section below is one claim from the lesson, printed. The
//! refusals (a missing case, arms of two types) cannot live here, because they
//! do not compile; the lesson shows rustc's real output for each.
//!
//!   rustc --edition 2024 match_expressions.rs -o /tmp/mx && /tmp/mx

fn banner(title: &str) {
    println!("\n──── {title}");
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Every `u8` is covered, and there is no `_`: the ranges meet end to end.
fn digits(n: u8) -> &'static str {
    match n {
        0..=9 => "one digit",
        10..=99 => "two digits",
        100..=u8::MAX => "three digits",
    }
}

/// Every `i32`, again with no `_`.
fn sign(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0 => "zero",
        1..=i32::MAX => "positive",
    }
}

fn kind(c: char) -> &'static str {
    match c {
        'a'..='z' => "lowercase",
        'A'..='Z' => "uppercase",
        '0'..='9' => "digit",
        _ => "something else",
    }
}

#[derive(Debug, Clone, Copy)]
enum Delivery {
    Standard,
    Express,
    Pickup, // imagine this variant arrived a year after the two functions below
}

/// Written when there were two variants. `_` meant "Express" then.
fn fee_with_catch_all(d: Delivery) -> u32 {
    match d {
        Delivery::Standard => 599,
        _ => 1_299,
    }
}

/// Names every variant, so adding `Pickup` was a compile error until someone
/// decided what it costs.
fn fee_named(d: Delivery) -> u32 {
    match d {
        Delivery::Standard => 599,
        Delivery::Express => 1_299,
        Delivery::Pickup => 0,
    }
}

fn main() {
    banner("1. Arms are tried top to bottom; the first that fits wins");
    for n in [4u8, 7, 12] {
        let band = match n {
            1..=5 => "low",
            3..=9 => "middle", // 3, 4 and 5 fit here too, and never arrive
            _ => "out of range",
        };
        println!("  {n:>2} -> {band}");
    }
    println!("  4 fits both ranges and takes the first. rustc does not warn:");
    println!("  the second arm still has 6..=9 to itself.");

    banner("2. Exactly one arm runs: there is no fall-through, so no break");
    for n in [1u8, 2, 3] {
        print!("  n = {n}: ");
        match n {
            1 => println!("ran the 1 arm"),
            2 | 3 => println!("ran the 2 | 3 arm"),
            _ => println!("ran the _ arm"),
        }
    }

    banner("3. match has a value, and an arm may be a block");
    for n in [0u32, 1, 5] {
        let word = match n {
            0 => "none".to_string(),
            1 => "one".to_string(),
            many => {
                let noun = "items";
                format!("{many} {noun}")
            }
        };
        println!("  {n} -> {word:?}");
    }

    banner("4. A bool: two arms are every case (the book's is_even)");
    for n in [6, 7] {
        let description = match is_even(n) {
            true => "even",
            false => "odd",
        };
        let same = if is_even(n) { "even" } else { "odd" };
        println!("  {n}: match says {description}, if says {same}");
    }

    banner("5. Integers: ranges that meet end to end need no _");
    for n in [7u8, 42, 255] {
        println!("  {n:>3}u8 -> {}", digits(n));
    }
    for n in [i32::MIN, 0, 17] {
        println!("  {n:>11}i32 -> {}", sign(n));
    }

    banner("6. A char: the ranges you care about, and _ for the rest of Unicode");
    for c in ['q', 'Q', '7', 'ż'] {
        println!("  {c:?} -> {}", kind(c));
    }

    banner("7. Listing 2.8 from the book: item is a &i32, the patterns are not");
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };
        if result == "hit!" {
            println!("  {item}: {result}");
        }
    }

    banner("8. A diverging arm fits any type");
    let mut total = 0u32;
    for text in ["12", "seven", "30"] {
        let n: u32 = match text.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("  {text:?}: not a number, skipped");
                continue; // type `!`, so it sits beside a u32 arm
            }
        };
        total += n;
        println!("  {text:?}: added, total = {total}");
    }

    banner("9. _ is exhaustive forever: a new variant goes where _ sends it");
    for d in [Delivery::Standard, Delivery::Express, Delivery::Pickup] {
        println!(
            "  {:<8} catch-all: {:>4} cents   every variant named: {:>4} cents",
            format!("{d:?}"),
            fee_with_catch_all(d),
            fee_named(d)
        );
    }
    println!("  Pickup costs 1299 through the catch-all, and nothing warned.");
}
