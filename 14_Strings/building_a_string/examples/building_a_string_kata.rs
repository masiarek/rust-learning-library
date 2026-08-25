//! Kata solution: build one line four ways, and see which inputs survive.
//!
//!   rustc --edition 2024 building_a_string_kata.rs -o /tmp/bsk && /tmp/bsk

use std::fmt::Write as _;

fn main() {
    // The one that does not compile:
    //
    //   let a = String::from("Ada");
    //   let b = String::from("Ben");
    //   let joined = a + b;
    //
    //   error[E0308]: mismatched types
    //     |
    //   4 |     let joined = a + b;
    //     |                      ^ expected `&str`, found `String`
    //     |
    //   help: consider borrowing here
    //     |
    //   4 |     let joined = a + &b;
    //     |                      +
    //
    // `impl Add<&str> for String` is the only one there is: the left side is
    // consumed and reused, the right side is borrowed.

    println!("A — chained +");
    let a1 = String::from("Ada");
    let b1 = String::from("Ben");
    let c1 = String::from("Cara");
    let joined = a1 + ", " + &b1 + ", " + &c1;
    println!("   {joined:?}");
    println!("   a1 is gone (moved into the result). b1 = {b1:?}, c1 = {c1:?} still alive.");
    println!("   Allocations: 0 new buffers — a1's buffer grew and became the answer.");

    println!("\nB — format!");
    let a2 = String::from("Ada");
    let b2 = String::from("Ben");
    let c2 = String::from("Cara");
    let made = format!("{a2}, {b2}, {c2}");
    println!("   {made:?}");
    println!("   all three still alive: {a2:?} {b2:?} {c2:?}");
    println!("   Allocations: 1 new buffer. Nothing was consumed.");

    println!("\nC — push_str in a loop");
    let names = [String::from("Ada"), String::from("Ben"), String::from("Cara")];
    let mut built = String::new();
    for (i, n) in names.iter().enumerate() {
        if i > 0 {
            built.push_str(", ");
        }
        built.push_str(n);
    }
    println!("   {built:?}");
    println!("   all three still alive; `built` grew from empty, capacity {}", built.capacity());
    println!("   Allocations: however many times the buffer doubled — pre-pay with");
    println!("   String::with_capacity if you know the size.");

    println!("\nD — write!");
    let mut out = String::with_capacity(32);
    for (i, n) in names.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write!(out, "{n}").unwrap();
    }
    println!("   {out:?}");
    println!("   Same shape as C, but the formatter writes straight into `out` — no");
    println!("   intermediate String per item, which is what format!-inside-a-loop costs.");

    println!("\nAll four agree: {}", joined == made && made == built && built == out);
    println!("\nWhich to reach for:");
    println!("   two or three known pieces      -> format!, and read it out loud");
    println!("   accumulating in a loop         -> push_str / write! into one buffer");
    println!("   a left value you are done with -> + reuses its buffer");
    println!("   format! inside a loop          -> the one to avoid: an allocation per pass");
}
