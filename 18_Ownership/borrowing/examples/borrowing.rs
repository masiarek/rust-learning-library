//! Borrowing: using a value without owning it.
//!
//! Ownership answers "who frees this?". Borrowing answers the question that
//! immediately follows: "how does anyone else get to look at it?" One rule
//! covers the whole thing — many readers, or one writer, never both at once —
//! and the interesting part is where the compiler decides a borrow has ENDED.
//!
//!   rustc --edition 2024 borrowing.rs -o /tmp/br && /tmp/br

use std::cell::Cell;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn take_and_return(s: String) -> (usize, String) {
    (s.len(), s) // the only way to be useful without borrowing: hand it back
}

fn just_look(s: &str) -> usize {
    s.len()
}

fn step1() {
    banner(1, "The alternative to borrowing is giving it back");

    let owned = String::from("hello");
    let (n, owned) = take_and_return(owned);
    println!("  moved in and out -> len {n}, and we have {owned:?} again");
    println!("  borrowed         -> len {}, and we never lost it", just_look(&owned));
    println!("      Same answer. The first signature makes every caller thread the");
    println!("      value back through a tuple; the second one just asks to look.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "Many readers at once");

    let scores = vec![5, 3, 0];
    let a = &scores;
    let b = &scores;
    let c = &scores[0];
    println!("  a.len()={} b.first()={:?} c={c}", a.len(), b.first());
    println!("  owner still readable: {scores:?}");
    println!("      Any number of `&T` may coexist, and the owner can still read.");
    println!("      Nothing can change underneath them, so nothing can be surprised.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn add_one(v: &mut Vec<i32>) {
    for x in v.iter_mut() {
        *x += 1;
    }
}

fn step3() {
    banner(3, "One writer, and only one");

    let mut scores = vec![5, 3, 0];
    add_one(&mut scores);
    println!("  after add_one    -> {scores:?}");
    println!("      While that `&mut` was out, `scores` was unusable even for");
    println!("      READING — an exclusive borrow is exclusive against everyone,");
    println!("      including its owner. A second `&mut v` alongside it is E0499.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "A borrow ends at its LAST USE, not at the end of the block");

    let mut scores = vec![5, 3, 0];

    let first = &scores[0]; // shared borrow starts
    println!("  read through the shared borrow: {first}");
    // `first` is never mentioned again, so the borrow is over right here...

    scores.push(9); // ...which is why this exclusive borrow is allowed
    println!("  pushed, now {scores:?}");
    println!("      Move that `println!` of `first` below the `push` and this stops");
    println!("      compiling with E0502. Same two statements, opposite order: what");
    println!("      extends a borrow is the last USE of the binding, not the call");
    println!("      that created it and not the closing brace.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "The bug the rule exists to prevent");

    let mut scores = vec![5, 3, 0];

    // `for x in &scores { scores.push(…) }` is E0502: the loop holds a shared
    // borrow for its whole run. Decide first, then mutate.
    let extra: Vec<i32> = scores.iter().filter(|&&s| s == 0).map(|_| 1).collect();
    scores.extend(extra);
    println!("  grown safely     -> {scores:?}");

    scores.retain(|&s| s > 0);
    println!("  retain           -> {scores:?}");
    println!("      Pushing to a Vec can REALLOCATE it, which would leave the loop's");
    println!("      pointer aimed at freed memory. Python raises at runtime if you");
    println!("      are lucky; C just reads the old buffer. Here it does not build.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn bump(counter: &Cell<i32>) {
    counter.set(counter.get() + 1); // mutation through a SHARED reference
}

fn step6() {
    banner(6, "`&` is shared, not immutable");

    let counter = Cell::new(0);
    bump(&counter);
    bump(&counter);
    println!("  mutated through &Cell<i32> -> {}", counter.get());
    println!("      The rule is about ALIASING, not about writing: `&T` means");
    println!("      'others may hold this too', and a type built for it (Cell,");
    println!("      RefCell, Mutex, atomics) may still change inside. Calling `&`");
    println!("      'immutable' is the shorthand that makes those look like cheats.");
}

// ─────────────────────────────────────────────────────────── Step 7
fn step7() {
    banner(7, "Method calls borrow for you");

    let scores = vec![5, 3, 0];
    println!("  scores.len()      -> {}", scores.len());
    println!("  Vec::len(&scores) -> {}", Vec::len(&scores));

    let mut owned = String::from("ada");
    owned.push('!'); // = String::push(&mut owned, '!')
    println!("  owned.push('!')   -> {owned:?}");
    println!("      The `&` and `&mut` in everyday code are mostly invisible: the");
    println!("      dot operator inserts whichever the method asked for. That is");
    println!("      why a borrow error can arrive from a line with no & in it.");
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
