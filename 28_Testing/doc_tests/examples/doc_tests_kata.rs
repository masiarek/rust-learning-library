//! Kata solution: an example that compiles and documents the wrong thing.
//!
//!   rustc --edition 2024 doc_tests_kata.rs -o /tmp/dtk && /tmp/dtk
//!   rustc --edition 2024 --crate-type lib --crate-name doc_tests_kata doc_tests_kata.rs
//!   rustdoc --edition 2024 --test doc_tests_kata.rs -L . \
//!       --extern doc_tests_kata=libdoc_tests_kata.rlib

/// Returns the winning candidate's index, or `None` on a tie.
///
/// The example below passes, and documents only half of that sentence:
///
/// ```
/// # use doc_tests_kata::winner;
/// assert_eq!(winner(&[3, 9, 5]), Some(1));
/// ```
///
/// The tie is the case the doc comment promises and the example above never
/// reaches, so here it is:
///
/// ```
/// # use doc_tests_kata::winner;
/// assert_eq!(winner(&[9, 9, 5]), None);
/// ```
///
/// And the empty ballot, which the sentence above says nothing about at all:
///
/// ```
/// # use doc_tests_kata::winner;
/// assert_eq!(winner(&[]), None);
/// ```
pub fn winner(scores: &[u32]) -> Option<usize> {
    let best = *scores.iter().max()?;
    let mut leaders = scores.iter().enumerate().filter(|(_, s)| **s == best);
    let first = leaders.next()?;
    if leaders.next().is_some() { None } else { Some(first.0) }
}

fn main() {
    println!("1. The three examples, run here as ordinary code");
    println!("   winner(&[3, 9, 5]) = {:?}", winner(&[3, 9, 5]));
    println!("   winner(&[9, 9, 5]) = {:?}", winner(&[9, 9, 5]));
    println!("   winner(&[])        = {:?}", winner(&[]));

    println!();
    println!("2. Why the first example was not enough");
    println!("   The doc comment makes two promises: a winner's INDEX, and None on");
    println!("   a TIE. One clear-winner example tests the first half and leaves");
    println!("   the second as a sentence nobody checks. A doc test earns its");
    println!("   place when it demonstrates the part of the contract a reader");
    println!("   would otherwise have to take on trust — which is usually the");
    println!("   None, the Err, or the edge.");
    println!("   And the third case was not promised at ALL: the sentence says");
    println!("   nothing about an empty ballot. Writing the example is what");
    println!("   surfaced the gap; the fix is a better sentence, not just a test.");

    println!();
    println!("3. What `#` hid, and what it must not hide");
    println!("   Each block above starts with a hidden `# use doc_tests_kata::winner;`");
    println!("   so the rendered example is one line. That is the right use: the");
    println!("   import is noise a reader can reconstruct.");
    println!("   Hiding the SETUP would not be — an example whose visible lines do");
    println!("   not run when copied is worse than no example, because it looks");
    println!("   like it should.");

    println!();
    println!("4. The fence annotations, and the one to distrust");
    println!("   ```              compiled and run");
    println!("   ```no_run        compiled, not run  (it opens a socket)");
    println!("   ```should_panic  run, must panic");
    println!("   ```ignore        not even compiled");
    println!("   ```text          not Rust at all, so nothing is attempted");
    println!("   `ignore` looks like a test and is a comment. If the block is not");
    println!("   Rust, say `text`; if it is Rust that cannot run here, say");
    println!("   `no_run`; `ignore` is for the rare block that is real Rust, is");
    println!("   worth showing, and genuinely cannot compile — and it should carry");
    println!("   a note saying why.");

    println!();
    println!("5. Why a doc test is an integration test");
    println!("   Each block is wrapped in its own `fn main()` and compiled as its");
    println!("   own crate, linked against yours. So it sees the PUBLIC API only,");
    println!("   exactly as a user does — which is why the hidden line says");
    println!("   `use doc_tests_kata::winner` and not `use crate::winner`, and why");
    println!("   a doc test is the cheapest way to find out that something you");
    println!("   meant to export is still private.");
}
