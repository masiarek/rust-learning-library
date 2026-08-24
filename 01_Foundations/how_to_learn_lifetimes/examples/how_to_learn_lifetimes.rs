//! "Clone your way out" works — until the thing you were avoiding was a mutation.
//!
//! Three pairs. The first is the advice working exactly as advertised. The
//! second is the one nobody warns you about: it compiles, it runs, and it is
//! wrong. The third is why `.clone()` does not even mean one thing.
//!
//!   rustc --edition 2024 how_to_learn_lifetimes.rs -o /tmp/htll && /tmp/htll

use std::rc::Rc;

/// Reads only. Cloning the input here is pure waste and nothing else.
fn total(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

/// Writes. The borrow is load-bearing.
fn add_ballot(scores: &mut Vec<u32>, score: u32) {
    scores.push(score);
}

/// The "clone your way out" version of the same function.
/// It compiles. It returns nothing. It changes nothing.
fn add_ballot_cloned(mut scores: Vec<u32>, score: u32) {
    scores.push(score);
    // `scores` is dropped here, along with the push.
}

fn main() {
    println!("1. Cloning to read: wasteful, harmless, and fine while learning");
    let scores = vec![5, 3, 0, 4];
    println!("   total(&scores)          = {}", total(&scores));
    println!("   total(&scores.clone())  = {}   <- same answer, one pointless copy",
             total(&scores.clone()));

    println!("\n2. Cloning to WRITE: compiles, runs, silently does nothing");
    let mut a = vec![5, 3];
    add_ballot(&mut a, 4);
    println!("   after add_ballot(&mut a, 4)      a = {a:?}   <- the push landed");

    let b = vec![5, 3];
    add_ballot_cloned(b.clone(), 4);
    println!("   after add_ballot_cloned(b.clone(), 4)  b = {b:?}      <- it did not");
    println!("   no error, no warning, no panic. The compiler cannot help here:");
    println!("   mutating a copy is a perfectly legal thing to want.");

    println!("\n3. `.clone()` does not mean one thing");
    let owned = vec![1, 2, 3];
    let copied = owned.clone();
    println!("   Vec::clone   -> a second, independent Vec  ({} and {} elements)",
             owned.len(), copied.len());

    let shared = Rc::new(vec![1, 2, 3]);
    println!("   Rc::strong_count before  = {}", Rc::strong_count(&shared));
    let also_shared = Rc::clone(&shared);
    println!("   Rc::clone    -> the SAME Vec, one more owner");
    println!("   Rc::strong_count after   = {}   (and both see {:?})",
             Rc::strong_count(&shared), also_shared);
    println!("\n   Same method name, opposite meanings: one copies the data,");
    println!("   the other copies only the right to reach it.");
}
