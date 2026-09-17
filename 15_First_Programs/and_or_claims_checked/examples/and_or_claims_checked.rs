//! A book chapter introduces Rust through two boolean functions, `and` and
//! `or`, before it trains a perceptron on their truth tables. Each numbered
//! block below checks one sentence of that introduction on rustc 1.98.0.
//! The page is 15_First_Programs/and_or_claims_checked/README.md.
//!
//!   rustc --edition 2024 and_or_claims_checked.rs -o /tmp/aoc && /tmp/aoc

use std::any::type_name_of_val;
use std::ops::{BitAnd, BitOr, Not};

fn and(a: bool, b: bool) -> bool {
    a && b
}

fn or(a: bool, b: bool) -> bool {
    a || b
}

/// Prints `name`, then hands `value` back, so a caller can see which operands
/// were evaluated and in what order.
fn noisy(name: &str, value: bool) -> bool {
    println!("     evaluated {name}");
    value
}

fn nothing_to_return() {}

fn main() {
    println!("1. The two functions reproduce both truth tables");
    println!("   A      B      and(A, B)  or(A, B)");
    for a in [true, false] {
        for b in [true, false] {
            println!("   {a:<6} {b:<6} {:<10} {}", and(a, b), or(a, b));
        }
    }

    println!();
    println!("2. and() is not the same as &&: a function gets both arguments already evaluated");
    println!("   noisy(left) && noisy(right):");
    let direct = noisy("left", false) && noisy("right", true);
    println!("     = {direct}");
    println!("   and(noisy(left), noisy(right)):");
    let wrapped = and(noisy("left", false), noisy("right", true));
    println!("     = {wrapped}");

    println!();
    println!("3. Operators as trait method calls: true of & | !, not of && ||");
    println!("   true & false = {:<5}  true.bitand(false) = {}", true & false, true.bitand(false));
    println!("   true | false = {:<5}  true.bitor(false)  = {}", true | false, true.bitor(false));
    println!("   !true        = {:<5}  true.not()         = {}", !true, true.not());
    println!("   std::ops has no trait for && or || (use std::ops::And is E0432)");

    println!();
    println!("4. An immutable binding may be assigned once, not only at the let");
    let a;
    a = true;
    println!("   let a; then a = true; compiles without mut: a = {a}");

    println!();
    println!("5. Shadowing hides a name; the old value is untouched");
    let flag = true;
    let first = &flag;
    let flag = false;
    println!("   first still reads {first}, the new flag is {flag}");

    println!();
    println!("6. A type can come from a later line, not only from the value");
    let mut answers = Vec::new();
    println!("   before any push: {}", type_name_of_val(&answers));
    answers.push(true);
    println!("   after answers.push(true): {answers:?}");

    println!();
    println!("7. A function with no -> returns (), and so does main");
    let returned = nothing_to_return();
    println!("   nothing_to_return() = {returned:?}");

    println!();
    println!("8. The ! makes println a macro; the brackets can be any of three kinds");
    println!("   parentheses: {} AND {} = {}", true, false, and(true, false));
    println!["   brackets:    {} OR {} = {}", true, false, or(true, false)];
    println! {"   braces:      {} AND {} = {}", false, false, and(false, false)}
}
