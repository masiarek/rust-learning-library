//! `while let`: loop for as long as the pattern keeps matching.
//!
//! The pattern is re-tested before every pass, so the `None` (or the `Err`) is
//! the loop's exit condition — no length, no index, no off-by-one available to
//! get wrong. What the language does NOT give you is any guarantee that the body
//! moves toward that exit, which is the one bug `if let` cannot have and this one
//! can.
//!
//!   rustc --edition 2024 while_let.rs -o /tmp/wl && /tmp/wl

use std::sync::mpsc;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() {
    banner(1, "The exit condition IS the pattern");

    let mut stack = vec![10, 20, 30];
    while let Some(top) = stack.pop() {
        println!("  popped {top}, {} left", stack.len());
    }
    println!("      `pop()` is a partial function returning Option, so running out");
    println!("      of items and ending the loop are the same event. Nothing here");
    println!("      counts, indexes, or checks a length.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "The bug `if let` cannot have: a body that makes no progress");

    let stack = vec![10, 20, 30];
    let mut passes = 0;

    // `last()` LOOKS at the top; `pop()` REMOVES it. This condition is therefore
    // true forever, and the counter below is the only reason this program ends.
    while let Some(top) = stack.last() {
        passes += 1;
        println!("  pass {passes}: top is still {top}");
        if passes == 4 {
            println!("  ...stopped by hand at 4 — nothing in the loop was ever going to stop it");
            break;
        }
    }
    println!("      An `if let` runs once whatever you write in it. A `while let`");
    println!("      re-tests the pattern, and the compiler will not check that the");
    println!("      body changed anything. The scrutinee has to CONSUME.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "Where the borrow ends decides whether you may consume");

    let mut stack = vec![10, 20, 30];
    while let Some(top) = stack.last() {
        let top = *top; // finish with the borrow FIRST...
        stack.pop(); // ...and only then take the mutable one
        println!("  read {top}, then popped it");
    }
    println!("      Move the `println!` of `top` below the `pop()` and this stops");
    println!("      compiling with E0502 — the immutable borrow from `last()` is");
    println!("      still live at that point. Same two statements, opposite order,");
    println!("      and the difference is the last USE of the binding, not the pop.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "`for` is this loop, already written for you");

    let names = ["Ada", "Ben", "Cara"];

    print!("  for            ->");
    for n in names.iter() {
        print!(" {n}");
    }
    println!();

    print!("  while let      ->");
    let mut it = names.iter();
    while let Some(n) = it.next() {
        print!(" {n}");
    }
    println!();
    println!("      Identical, because `for` desugars to roughly the second one.");
    println!("      Hand-writing it is usually a downgrade — unless you need the");
    println!("      iterator itself between passes, which `for` has moved away.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "When you DO need the iterator: peeking and taking");

    // Run-length grouping: each pass consumes a variable number of items, so the
    // loop has to hold the iterator itself. `for` cannot express this.
    let marks = [5, 5, 5, 3, 3, 0];
    let mut it = marks.iter().peekable();
    print!("  runs           ->");
    while let Some(&first) = it.next() {
        let mut run = 1;
        while it.peek() == Some(&&first) {
            it.next();
            run += 1;
        }
        print!(" {first}×{run}");
    }
    println!();

    // `by_ref` takes a bite and leaves the rest for the next loop.
    let mut it = marks.iter();
    let head: Vec<_> = it.by_ref().take(2).collect();
    print!("  first two {head:?}, then the rest ->");
    for m in it {
        // A plain `for` again — the hand-written loop earned its keep above,
        // and there is no reason to keep paying for it here.
        print!(" {m}");
    }
    println!();
    println!("      Both halves need the iterator to survive between passes, which");
    println!("      is the one thing `for` takes away. That — not style — is when");
    println!("      the hand-written loop is the right call.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn step6() {
    banner(6, "Not just Option: any pattern that eventually stops matching");

    let (tx, rx) = mpsc::channel();
    for score in [5, 3, 0] {
        tx.send(score).unwrap();
    }
    drop(tx); // the last sender going away is what ends the loop below

    let mut total = 0;
    while let Ok(score) = rx.recv() {
        total += score;
        println!("  received {score}, running total {total}");
    }
    println!("  channel closed, final total {total}");
    println!("      Convenient, and it carries `if let`'s trade into a loop: `Err`");
    println!("      ends it, and a disconnect and a real error are now the same");
    println!("      event. If those deserve different handling, write the `match`.");
}

// ─────────────────────────────────────────────────────────── Step 7
fn first_two(scores: &[u32]) -> Option<(u32, u32)> {
    let mut it = scores.iter();
    // No `while let … else` exists; a `let … else` inside the loop body is how
    // you leave early on a pattern that failed.
    let Some(&a) = it.next() else { return None };
    let Some(&b) = it.next() else { return None };
    Some((a, b))
}

fn step7() {
    banner(7, "There is no `while let … else`");

    println!("  first_two(&[5, 3, 0]) -> {:?}", first_two(&[5, 3, 0]));
    println!("  first_two(&[5])       -> {:?}", first_two(&[5]));
    println!("      `while … else` is a hard error: \"`while...else` loops are not");
    println!("      supported\". A loop's pattern failing is its NORMAL ending, so");
    println!("      there is nothing for an else to mean. Put the escape inside the");
    println!("      body with `let … else`, or check after the loop.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    step7();
    println!();
}
