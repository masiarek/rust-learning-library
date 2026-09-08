//! Kata solution: the comparison the optimizer is allowed to delete.
//!
//!   rustc --edition 2024 signed_overflow_kata.rs -o /tmp/sok && /tmp/sok
//!   rustc --edition 2024 -O signed_overflow_kata.rs -o /tmp/sok && /tmp/sok

fn main() {
    let x = i32::MAX;

    println!("THE C SHAPE");
    println!("  int x = INT_MAX;  if (x + 1 > x) ...");
    println!("  At -O0 the addition wraps and the comparison is false. At -O2");
    println!("  the optimizer reasons: signed overflow is UNDEFINED, therefore");
    println!("  x + 1 > x cannot be false, therefore the branch is dead -- and");
    println!("  deletes it. Same source, two behaviours, and neither compiler");
    println!("  is wrong.");
    println!();

    println!("RUST'S ANSWER IS TO DEFINE IT, TWICE");
    println!("  debug builds:   overflow PANICS -- 'attempt to add with overflow'");
    println!("  release builds: overflow WRAPS, two's complement");
    println!("  Both are defined behaviour. The optimizer may not assume the");
    println!("  addition cannot overflow, so no branch anywhere is deleted on");
    println!("  that reasoning, and the two builds differ in what they DO");
    println!("  rather than in what is true.");
    println!();

    println!("AND WHEN YOU CARE, YOU SAY WHICH");
    println!("  x = i32::MAX = {x}");
    println!("  x.checked_add(1)     {:?}    <- None: ask, and handle it", x.checked_add(1));
    println!("  x.wrapping_add(1)    {}      <- wrap, on purpose", x.wrapping_add(1));
    println!("  x.saturating_add(1)  {}      <- clamp at the maximum", x.saturating_add(1));
    println!("  x.overflowing_add(1) {:?}   <- the value and a did-it-wrap flag",
             x.overflowing_add(1));
    println!();
    println!("  Four named behaviours where C has one undefined one. The point");
    println!("  is not that Rust picked a better default -- it is that the");
    println!("  choice is written at the call site, so a reader can see which");
    println!("  one this line meant.");
    println!();

    println!("THE PART THAT TRANSFERS BACK TO C");
    println!("  'Undefined' does not mean 'unpredictable result'. It means the");
    println!("  compiler may assume it never happens and rewrite the code around");
    println!("  that assumption -- so the damage lands somewhere else entirely,");
    println!("  usually in a check you wrote to prevent it. That is why");
    println!("  `if (x + 1 < x)` is not an overflow test in C, and why the");
    println!("  correct test compares against INT_MAX before adding.");
    println!();

    println!("A DETAIL WORTH KNOWING");
    println!("  Unsigned overflow in C is DEFINED to wrap, so the same trick is");
    println!("  not available to the optimizer there. The undefined-ness is a");
    println!("  property of the signed types alone -- which is why so much");
    println!("  hardening advice is 'use unsigned', and why that advice brings");
    println!("  its own family of wrap-around bugs.");

    assert_eq!(x.checked_add(1), None);
    assert_eq!(x.wrapping_add(1), i32::MIN);
    assert_eq!(x.saturating_add(1), i32::MAX);
}
