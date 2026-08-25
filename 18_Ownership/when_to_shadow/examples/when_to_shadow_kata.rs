//! Kata solution: three shadows in one function, and only one of them is earned.
//!
//! The broken version below compiles, runs, prints a plausible report, and is
//! wrong twice. It gets exactly ONE warning, which points at neither bug in
//! terms you would recognise.
//!
//! The two fixes are deliberately different, and that difference is the answer
//! to "when should I shadow?":
//!
//!   * the lost sum is fixed by DELETING a shadow — the loop wanted to write to
//!     the outer variable, so it wanted `mut`, not a new binding;
//!   * the wrong filter is fixed by RENAMING — both values are legitimate, they
//!     are simply not the same concept and should never have shared a word.
//!
//! The third shadow is left exactly as it is. It is the idiom.
//!
//!   rustc --edition 2024 when_to_shadow_kata.rs -o /tmp/wtsk && /tmp/wtsk

/// One voter's card as it arrives: a name and their scores as raw text.
const CARDS: [(&str, &str); 5] = [
    ("Ada", " 5 "),
    ("Ben", "2"),
    ("Cara", " 4"),
    ("Dan", "0 "),
    ("Eve", " 3 "),
];

/// The minimum score that counts as support, for the report's second line.
const SUPPORT_AT_LEAST: usize = 3;

// ─────────────────────────────────────────────────────────────── the bug
//
// rustc's only complaint about this function is:
//
//   warning: variable does not need to be mutable
//    --> when_to_shadow_kata.rs
//     |     let mut total = 0;
//
// which is the accumulator bug reported in a vocabulary that does not contain
// the word "shadow". Nothing at all is said about `threshold`.
#[allow(unused_mut)]
fn tally_broken() -> (usize, usize) {
    let mut total = 0;

    for (_who, raw) in CARDS {
        // Shadow #1 — EARNED. Same concept (this voter's score), better form:
        // &str with spaces -> &str trimmed -> usize. No second name needed.
        let raw = raw.trim();
        let raw: usize = raw.parse().expect("every card above is a number");

        // Shadow #2 — the lost sum. This builds a fresh `total` from the outer
        // one and drops it at the closing brace, three times over.
        let total = total + raw;
        let _ = total;
    }

    let threshold = SUPPORT_AT_LEAST;
    println!("  counting support at {threshold} or above");

    // Shadow #3 — one name, two concepts. `threshold` was a SCORE; now it is a
    // COUNT of cards. Both are usize, both are read, so nothing warns.
    let threshold = CARDS.len();
    let supporters = CARDS
        .iter()
        .filter(|(_who, raw)| raw.trim().parse::<usize>().unwrap_or(0) >= threshold)
        .count();

    (total, supporters)
}

// ─────────────────────────────────────────────────────────── the two fixes
fn tally_fixed() -> (usize, usize) {
    let mut total = 0;

    for (_who, raw) in CARDS {
        // Shadow #1 stays. It was never the problem.
        let raw = raw.trim();
        let raw: usize = raw.parse().expect("every card above is a number");

        // Fix A: DELETE the shadow. The loop wanted to write to the variable
        // that outlives it, and `mut` is exactly the promise that allows.
        total += raw;
    }

    // Fix B: RENAME. Both values are wanted; they are simply different things,
    // and the moment they have honest names the bug is unwriteable.
    let min_score = SUPPORT_AT_LEAST;
    let card_count = CARDS.len();
    let supporters = CARDS
        .iter()
        .filter(|(_who, raw)| raw.trim().parse::<usize>().unwrap_or(0) >= min_score)
        .count();

    println!("  counting support at {min_score} or above, over {card_count} cards");

    (total, supporters)
}

fn main() {
    println!("──── The broken version");
    let (total, supporters) = tally_broken();
    println!("  total score = {total}      (five cards scoring 5, 2, 4, 0, 3)");
    println!("  supporters  = {supporters}      (should be the scores of 3 or more)");
    println!("      Both numbers are wrong, and both look like they could be");
    println!("      right — which is the whole hazard. A total of 0 reads as a");
    println!("      blank election, and 1 supporter as a weak field. Neither");
    println!("      number looks like a bug; they look like a finding.");

    println!("\n──── The fixed version");
    let (total, supporters) = tally_fixed();
    println!("  total score = {total}     (5 + 2 + 4 + 0 + 3)");
    println!("  supporters  = {supporters}      (5, 4 and 3 clear the bar of 3)");

    println!("\n──── Why the two fixes differ");
    println!("  The lost sum was a shadow standing where `mut` belonged: the");
    println!("  loop needed to CHANGE something that outlives it, and that is");
    println!("  the one job shadowing cannot do. Deleting the `let` fixed it.");
    println!();
    println!("  The wrong filter was a shadow standing where a SECOND NAME");
    println!("  belonged: a minimum score and a card count are different");
    println!("  quantities that happen to share a type. Renaming fixed it,");
    println!("  and no shadow was involved in the repair at all.");
    println!();
    println!("  Shadow #1 survived both passes untouched. Same concept, new");
    println!("  form, two lines apart — that is what an earned shadow reads");
    println!("  like, and it is why the answer is never \"stop shadowing\".");
}
