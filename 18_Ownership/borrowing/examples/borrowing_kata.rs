//! Kata solution: many readers, or one writer — and where the borrow ended.
//!
//!   rustc --edition 2024 borrowing_kata.rs -o /tmp/bwk && /tmp/bwk

/// Reads. Takes `&[u8]`, not `&Vec<u8>` — every caller with something
/// slice-shaped can call it, and it promises to change nothing.
fn total(scores: &[u8]) -> u32 {
    scores.iter().map(|s| *s as u32).sum()
}

/// Writes. One of these may exist at a time, and no reader alongside it.
fn cap_at(scores: &mut Vec<u8>, cap: u8) {
    for s in scores.iter_mut() {
        if *s > cap {
            *s = cap;
        }
    }
}

fn main() {
    let mut scores = vec![5u8, 9, 3, 7];

    println!("Many readers at once — fine:");
    let a = &scores;
    let b = &scores;
    println!("  a.len() = {}, b.len() = {}, total = {}", a.len(), b.len(), total(b));

    // Both shared borrows are last used above, so they are over by this line.
    // That is what lets the mutable borrow below exist at all.
    println!("\nOne writer, once the readers are finished:");
    cap_at(&mut scores, 5);
    println!("  capped -> {scores:?}");

    println!("\nWhere the borrow ends is the whole game:");
    let first = &scores[0];
    println!("  read through the borrow -> {first}");
    // `first` is not used again, so its borrow has ended here...
    scores.push(4);
    println!("  ...so pushing is allowed now -> {scores:?}");
    println!("      Move that println! below the push and it stops compiling:");
    println!("      E0502, cannot borrow `scores` as mutable because it is also");
    println!("      borrowed as immutable. Nothing about the code moved — only");
    println!("      the last USE of the borrow, which is what defines its end.");

    println!("\nThe bug the rule exists to prevent:");
    let mut v = vec![1u8, 2, 3];
    let len = v.len(); // read it OUT, do not hold a borrow across the push
    v.push(4);
    println!("  len read before the push -> {len}, now {} ", v.len());
    println!("      Holding `&v[0]` across that push would be a dangling pointer");
    println!("      in a language that allowed it: push can reallocate, and the");
    println!("      old buffer is freed. Rust rejects it at compile time instead.");

    println!("\n`&` means shared, not immutable — the interior-mutability escape:");
    use std::cell::Cell;
    let counter = Cell::new(0u32);
    let bump = |c: &Cell<u32>| c.set(c.get() + 1);
    bump(&counter);
    bump(&counter);
    println!("  counter behind a & -> {}", counter.get());
}
