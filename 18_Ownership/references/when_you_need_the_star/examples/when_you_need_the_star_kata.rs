//! Kata solution: fewest stars.
//!
//! The task wrote every marked line with no `*` and no borrow `&`. Four of the
//! six were refused by rustc 1.98.0: (a) `if first` E0308, (c) `n > limit` E0308,
//! (e) `lamp = first | dips` E0308, (f) `report(flags, readings, limit, lamp)`
//! E0308. (b) and (d) compiled as written. Each fix below adds the fewest
//! sigils; the comment names the rule, and the error one sigil fewer gives.
//!
//!   rustc --edition 2024 when_you_need_the_star_kata.rs -o /tmp/when_you_need_the_star_kata && /tmp/when_you_need_the_star_kata

fn report(flags: &[bool], readings: &[i32], limit: &i32, lamp: &mut bool) {
    let first = &flags[0]; // &bool
    let peak = readings.iter().max().unwrap(); // &i32

    // (a) 1 `*`. A condition wants exactly `bool`: no trait, no coercion.
    if *first {
        println!("(a) if *first                      -> taken");
    }
    println!("    rule: a condition wants `bool`");

    // (b) 0. Both sides are `&i32`: `PartialOrd<&B> for &A` compares them.
    //     `*peak > *limit` compiles too, with two stars it did not need.
    let over = peak > limit;
    println!("(b) peak > limit                   -> {over}");
    println!("    rule: same depth, &i32 > &i32");

    // (c) 1 `*`. `iter()` yields `&i32`, `filter` passes `&&i32`; one `*` reaches
    //     `limit`'s depth. With none: E0308, expected `&&i32`, found `&i32`.
    //     `**n > *limit` compiles with three.
    let high = readings.iter().filter(|n| *n > limit).count();
    println!("(c) filter(|n| *n > limit).count() -> {high}");
    println!("    rule: filter adds a `&`; then same depth");

    // (d) 0. `&i32` has no `is_negative`, so the dot dereferences to `i32`.
    let dips = readings.iter().any(|n| n.is_negative());
    println!("(d) any(|n| n.is_negative())       -> {dips}");
    println!("    rule: the dot");

    // (e) 1 `*`. `*lamp` is the place; without it, E0308, expected `&mut bool`,
    //     found `bool`. The right side needs none: `impl BitOr<bool> for &bool`.
    *lamp = first | dips;
    println!("(e) *lamp = first | dips           -> *lamp = {}", *lamp);
    println!("    rule: `*lamp` is the place; `|` has an impl for &bool");
}

fn main() {
    let flags = vec![true, false, false];
    let readings = vec![4, -2, 9];
    let limit = 7;
    let mut lamp = false;

    // (f) one `&` each. `&flags` is a `&Vec<bool>`, and the parameter's `&[bool]`
    //     accepts it: `Vec<bool>` derefs to `[bool]`. Without them, E0308.
    report(&flags, &readings, &limit, &mut lamp);
    println!("(f) report(&flags, &readings, &limit, &mut lamp)");
    println!("    rule: deref coercion, &Vec<bool> -> &[bool] and &Vec<i32> -> &[i32]");

    println!();
    println!("stars in report: 3 (a, c, e); lamp is now {lamp}");
}
