//! Kata solution: how many times does the counted closure actually run?
//!
//!   rustc --edition 2024 iterators_are_lazy_kata.rs -o /tmp/ialk && /tmp/ialk

use std::cell::Cell;

const SCORES: [i32; 6] = [5, 3, 0, 4, 2, 1];
const NAMES: [&str; 3] = ["Ada", "Ben", "Cara"];

fn main() {
    let calls = Cell::new(0);
    // The counted closure. It is deliberately trivial: what is being measured
    // is how often the chain asks for it, not what it does.
    let seen = |s: &i32| {
        calls.set(calls.get() + 1);
        *s
    };
    let mut report = |label: &str, answer: String| {
        println!("   {label:<44} {:<18} calls: {}", answer, calls.get());
        calls.set(0);
    };

    println!("1. Seven chains over the same six scores");

    let out: Vec<i32> = SCORES.iter().map(seen).collect();
    report(".map(seen).collect()", format!("{out:?}"));

    let n = SCORES.iter().map(seen).count();
    report(".map(seen).count()", n.to_string());

    let f = SCORES.iter().map(seen).find(|s| *s == 0);
    report(".map(seen).find(== 0)", format!("{f:?}"));

    let p = SCORES.iter().map(seen).position(|s| s == 4);
    report(".map(seen).position(== 4)", format!("{p:?}"));

    let a = SCORES.iter().map(seen).any(|s| s < 4);
    report(".map(seen).any(< 4)", a.to_string());

    let all = SCORES.iter().map(seen).all(|s| s < 4);
    report(".map(seen).all(< 4)", all.to_string());

    let z: Vec<(i32, &str)> = SCORES.iter().map(seen).zip(NAMES).collect();
    report(".map(seen).zip(3 names).collect()", format!("{}", z.len()));

    println!();
    println!("2. Why each of the short ones stopped");
    println!("   find(== 0)     3 calls — the zero sits at index 2, so the third");
    println!("                  element settled the question.");
    println!("   position(== 4) 4 calls — same reason, one element further along.");
    println!("   any(< 4)       2 calls — the first `true` ends it. `all` is the");
    println!("                  mirror image: it runs until the first `false`,");
    println!("                  which here is the very first score, so 1 call.");
    println!("   zip(3 names)   3 calls — the SHORTER side ends the pair. Nothing");
    println!("                  about the scores stopped it; the names ran out.");
    println!("   collect/count  6 calls — neither can answer without every element.");

    println!();
    println!("3. And the trap: zip pulls the LEFT side first");
    let left = Cell::new(0);
    let right = Cell::new(0);
    let pairs: Vec<(i32, i32)> = SCORES
        .iter()
        .inspect(|_| left.set(left.get() + 1))
        .zip(SCORES.iter().inspect(|_| right.set(right.get() + 1)).take(2))
        .map(|(a, b)| (*a, *b))
        .collect();
    println!("   pairs = {pairs:?}");
    println!("   left side pulled {} times, right side {} times", left.get(), right.get());
    println!("   zip asks the left iterator for an item BEFORE it discovers the");
    println!("   right one is empty — so the longer side is always pulled one");
    println!("   extra time. If that pull has a side effect, this is where it bites.");

    println!();
    println!("4. Which order to write .filter(cheap).filter(expensive) in");
    let expensive_calls = Cell::new(0);
    let expensive = |s: &&i32| {
        expensive_calls.set(expensive_calls.get() + 1);
        **s > 3
    };
    let cheap_first = SCORES.iter().filter(|s| **s % 2 == 1).filter(expensive).count();
    let cheap_first_cost = expensive_calls.get();
    expensive_calls.set(0);
    let expensive_first = SCORES.iter().filter(expensive).filter(|s| **s % 2 == 1).count();
    println!("   cheap first:     {cheap_first} row(s), expensive test ran {cheap_first_cost} times");
    println!("   expensive first: {expensive_first} row(s), expensive test ran {} times",
             expensive_calls.get());
    println!("   Same rows either way, so this is a cost question only — and the");
    println!("   answer is not simply \"cheap first\". Put first whichever test");
    println!("   REJECTS the most rows per unit of work: a cheap filter that keeps");
    println!("   everything saves nothing. You need the data's selectivity, which");
    println!("   is exactly what a query planner spends its life estimating.");
}
