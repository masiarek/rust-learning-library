//! Beside the code, or outside it — and what each of the two can see.
//!
//!   rustc --edition 2024 where_a_test_goes.rs -o /tmp/wtg && /tmp/wtg

pub fn tally(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

/// Private, so only a unit test in this module can reach it.
fn is_spoiled(score: u32) -> bool {
    score > 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_a_ballot() {
        assert_eq!(tally(&[5, 3, 0]), 8);
    }

    #[test]
    fn spots_a_spoiled_score() {
        // Reaching a PRIVATE function: only an inline test can do this.
        assert!(is_spoiled(9));
        assert!(!is_spoiled(5));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn indexing_past_the_end_panics() {
        let scores = [5u32, 3];
        let _ = scores[std::hint::black_box(9)];
    }

    #[test]
    #[ignore = "slow; run with --ignored"]
    fn the_expensive_one() {
        assert_eq!(tally(&vec![1; 1_000_000]), 1_000_000);
    }
}

fn main() {
    println!("1. The two places, and the one difference that decides it");
    println!("   src/lib.rs        #[cfg(test)] mod tests   -> UNIT tests");
    println!("   tests/api.rs      no attribute needed      -> INTEGRATION tests");
    println!("   A unit test is inside the module, so it can call private items.");
    println!("   An integration test is a separate crate that `use`s yours, so it");
    println!("   sees exactly what a real user sees — which is the point of it.");
    println!("   Test the behaviour from outside; test the awkward internals from");
    println!("   inside, and only when the outside cannot reach them.");
    println!("   tally(&[5, 3, 0]) = {}, is_spoiled(9) = {}", tally(&[5, 3, 0]), is_spoiled(9));

    println!();
    println!("2. `#[cfg(test)]` is not a run-time skip");
    println!("   The `tests` module above is compiled ONLY under `cargo test`.");
    println!("   In this binary it does not exist: cfg!(test) = {}", cfg!(test));
    println!("   So test-only helpers, fixtures and dev-dependencies cost the");
    println!("   shipped artefact nothing at all.");

    println!();
    println!("3. `use super::*` is the one glob nobody argues about");
    println!("   The test module is a CHILD of the module under test, so it can");
    println!("   see its parent's private items — but it still has to name them.");
    println!("   `use super::*;` is the first line of almost every test module in");
    println!("   Rust, and it is safe for the reason globs usually are not: there");
    println!("   is exactly one place those names can have come from.");

    println!();
    println!("4. The two attributes worth knowing on day one");
    println!("   #[should_panic(expected = \"…\")]  the test PASSES if it panics,");
    println!("     and the message must contain that substring. Without `expected`");
    println!("     it passes on ANY panic, including one from a typo — which is");
    println!("     how a should_panic test quietly stops testing anything.");
    println!("   #[ignore = \"reason\"]  compiled, not run; `cargo test -- --ignored`");
    println!("     runs exactly these. The reason string is printed in the summary.");

    println!();
    println!("5. What `cargo test` runs, in one list");
    println!("   a. unit tests      #[test] fns anywhere in src/, usually in a");
    println!("                      #[cfg(test)] module");
    println!("   b. integration     every .rs file directly in tests/, each its own");
    println!("                      crate; tests/common/mod.rs for shared helpers,");
    println!("                      because a subdirectory is not compiled as a test");
    println!("   c. doc tests       the ``` blocks in your /// comments");
    println!("   All three in one command, and the doc tests are the ones people");
    println!("   forget they are shipping.");
}
