//! Kata solution: a countdown that prints nothing, then three loops over one
//! Vec in the only order that compiles.
//!
//!   rustc --edition 2024 for_loops_kata.rs -o /tmp/flk && /tmp/flk

fn main() {
    println!("=== part 1: the countdown ===");
    let mut lines = 0;
    for _ in 10..1 {
        lines += 1;
    }
    println!("  for n in 10..1       -> {lines} lines   <- start > end is an EMPTY range, not a backwards one");

    let mut out = Vec::new();
    for n in (1..=10).rev() {
        out.push(n.to_string());
    }
    println!("  for n in (1..=10).rev() -> {}", out.join(" "));

    out.clear();
    for n in (1..11).rev() {
        out.push(n.to_string());
    }
    println!("  for n in (1..11).rev()  -> {}   <- same, but the 11 makes you do arithmetic", out.join(" "));
    println!("  rustc compiles 10..1 without a word; clippy's reversed_empty_ranges is deny-by-default");

    println!();
    println!("=== part 2: three doors, one Vec ===");
    let mut scores = vec![72, 85, 90];

    // Door 1: &scores lends each item as &i32, so scores survives.
    let mut total = 0;
    for s in &scores {
        total += s;
    }
    println!("  for s in &scores     -> total {total}, scores still {scores:?}");

    // Door 2: &mut scores lends each item as &mut i32; write through it with *.
    for s in &mut scores {
        *s += 5;
    }
    println!("  for s in &mut scores -> *s += 5, scores now {scores:?}");

    // Door 3: scores itself is moved into the loop. This one has to come last.
    let mut labels: Vec<String> = Vec::new();
    for s in scores {
        labels.push(format!("score: {s}"));
    }
    println!("  for s in scores      -> labels {labels:?}");

    println!();
    println!("=== why door 3 comes last ===");
    println!("  `for s in scores` moves the Vec when that loop STARTS. Put it first and the");
    println!("  next loop is refused: error[E0382]: borrow of moved value: `scores`.");
    println!("  The caret sits on `&scores` in `for s in &scores`, the first use after the move;");
    println!("  the consuming `for` line is labelled");
    println!("  \"`scores` moved due to this implicit call to `.into_iter()`\".");
    println!("  One error, not two: the `&mut scores` loop after it is not reported separately.");
    println!("  The help is a one-character fix on the consuming loop: `for s in &scores`.");
}
