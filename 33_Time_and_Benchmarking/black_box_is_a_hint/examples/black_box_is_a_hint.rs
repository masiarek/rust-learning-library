//! `black_box` is a hint. To the program it is the identity function; what it
//! does to the optimizer is invisible here, because this library's examples
//! are built without optimization. The page shows that half in assembly.
//!
//!   rustc --edition 2024 black_box_is_a_hint.rs -o /tmp/bbh && /tmp/bbh

use std::hint::black_box;

// `black_box` is a `const fn` since 1.86, and in constant evaluation it does nothing.
const ANSWER: u64 = black_box(6) * 7;

/// The barrier sees only the finished sum.
fn result_only(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        sum += i;
    }
    black_box(sum)
}

/// The barrier sees every value the loop adds.
fn every_iteration(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        sum += black_box(i);
    }
    black_box(sum)
}

fn main() {
    println!("1. To the program, black_box is the identity function");
    println!("   black_box(55)          = {}", black_box(55));
    let v = vec![1, 2, 3];
    let r = black_box(&v);
    println!("   black_box(&v) is &v    = {}", std::ptr::eq(r, &v));
    println!("   const ANSWER           = {ANSWER}");

    println!();
    println!("2. Where it goes changes the machine code, never the answer");
    println!("   result_only(1_000)     = {}", result_only(1_000));
    println!("   every_iteration(1_000) = {}", every_iteration(1_000));
    println!("   same answer            = {}", result_only(1_000) == every_iteration(1_000));
}
