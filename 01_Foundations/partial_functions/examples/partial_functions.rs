//! Partial functions: `Option` turns "sometimes undefined" into "always answers".
//!
//! A *total* function has an answer for every input. A *partial* one does not —
//! division has no answer when the divisor is zero, `first()` has none for an
//! empty list. Widening the return type to `Option<T>` makes a partial function
//! total, because "no answer" becomes one of the answers.
//!
//!   rustc --edition 2024 partial_functions.rs -o /tmp/pf && /tmp/pf

use std::panic;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 { None } else { Some(dividend / divisor) }
}

fn step1() {
    banner(1, "The classic partial function, made total");

    for (a, b) in [(10.0, 2.0), (10.0, 0.0)] {
        match divide(a, b) {
            Some(v) => println!("  divide({a}, {b}) -> Result: {v}"),
            None => println!("  divide({a}, {b}) -> Cannot divide by zero"),
        }
    }
    println!("      `divide` now has an answer for EVERY pair of f64. That is what");
    println!("      'total' means, and it is the whole trick.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "But be careful WHY you are guarding: floats do not panic");

    println!("  10.0_f64 / 0.0  = {}", 10.0_f64 / 0.0);
    println!("   0.0_f64 / 0.0  = {}", 0.0_f64 / 0.0);
    println!("      No panic, no error — IEEE 754 says inf and NaN, and they propagate");
    println!("      silently through every later calculation. So the Option is not");
    println!("      preventing a crash; it is stopping a WRONG NUMBER from escaping.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn divisor_from(s: &str) -> i32 {
    s.parse().unwrap() // opaque to the compiler, so `n / d` is not const-folded
}

fn step3() {
    banner(3, "Integers are the case that really does panic");

    let n = 10;
    let d = divisor_from("0");

    let prior = panic::take_hook();
    panic::set_hook(Box::new(|_| {})); // keep the demo output clean
    let outcome = panic::catch_unwind(|| n / d);
    panic::set_hook(prior);

    match outcome {
        Ok(v) => println!("  10 / 0 evaluated to {v}"),
        Err(_) => println!("  10 / 0 -> panicked (caught here only to keep this demo running)"),
    }
    println!("  10i32.checked_div(0) -> {:?}", 10i32.checked_div(0));
    println!("      Same expression, two designs: `/` is partial and panics;");
    println!("      `checked_div` is total and hands back None.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "std is full of these — the checked_* family");

    println!("  i32::MAX.checked_add(1)  -> {:?}", i32::MAX.checked_add(1));
    println!("  5u32.checked_sub(10)     -> {:?}", 5u32.checked_sub(10));
    println!("  10i32.checked_rem(0)     -> {:?}", 10i32.checked_rem(0));
    println!("  2i32.checked_pow(40)     -> {:?}", 2i32.checked_pow(40));
    println!("      Every one is an operation that is undefined somewhere in its input");
    println!("      range, offered as a total function returning Option.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "Collections do it too: indexing vs get()");

    let v = vec![10, 20, 30];
    println!("  v.first()  -> {:?}", v.first());
    println!("  v.last()   -> {:?}", v.last());
    println!("  v.get(9)   -> {:?}", v.get(9));

    let empty: Vec<i32> = vec![];
    println!("  empty.first() -> {:?}", empty.first());
    println!("      `v[9]` is the partial version and panics. `.get(9)` is the total one.");
    println!("      Same data, same question — the return type is the entire difference.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn sqrt(x: f64) -> Option<f64> {
    if x < 0.0 { None } else { Some(x.sqrt()) }
}

fn step6() {
    banner(6, "When None is not enough, the answer is Result");

    println!("  sqrt(9.0)  -> {:?}", sqrt(9.0));
    println!("  sqrt(-1.0) -> {:?}", sqrt(-1.0));
    println!("      Here None is fine: there is exactly one reason a real square root");
    println!("      is undefined. Once a function can be undefined for SEVERAL reasons,");
    println!("      None stops being an answer and you owe the caller a Result.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    println!();
}
