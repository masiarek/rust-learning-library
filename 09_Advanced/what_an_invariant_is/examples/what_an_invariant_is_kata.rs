//! Kata solution: the invariant nobody wrote down, and the ordinary `pub fn`
//! that broke it.
//!
//!   rustc --edition 2024 what_an_invariant_is_kata.rs -o /tmp/ik && /tmp/ik

mod tally {
    /// Word counts, plus the running total nobody wants to recompute.
    pub struct Tally {
        counts: Vec<u32>,
        total: u32, // INVARIANT: total == counts.iter().sum()
    }

    impl Tally {
        pub fn new() -> Tally {
            Tally { counts: Vec::new(), total: 0 }
        }

        /// Maintains the invariant, so it is safe to call from anywhere.
        pub fn record(&mut self, n: u32) {
            self.counts.push(n);
            self.total += n;
        }

        /// O(1): it reads the cache instead of the data. Correct only while
        /// the invariant holds.
        pub fn total(&self) -> u32 {
            self.total
        }

        /// The leak. `pub`, ordinary, no `unsafe` anywhere — and it forgets
        /// the second half of what `record` does.
        pub fn record_unchecked(&mut self, n: u32) {
            self.counts.push(n);
        }

        /// The fix, if the fast path really is needed: restore the invariant
        /// before returning.
        pub fn resync(&mut self) {
            self.total = self.counts.iter().sum();
        }

        /// The invariant, written as code so a test can ask.
        pub fn holds(&self) -> bool {
            self.total == self.counts.iter().sum::<u32>()
        }

        pub fn counts(&self) -> &[u32] {
            &self.counts
        }
    }
}

use tally::Tally;

fn main() {
    let mut t = Tally::new();
    for n in [12, 30, 8] {
        t.record(n);
    }
    println!("1. While every door maintains it");
    println!("   counts = {:?}", t.counts());
    println!("   total() = {}   holds() = {}", t.total(), t.holds());
    println!();

    println!("2. One ordinary `pub fn` later");
    t.record_unchecked(100);
    println!("   counts = {:?}", t.counts());
    println!("   total() = {}   holds() = {}", t.total(), t.holds());
    println!("   The number is wrong by exactly the value that skipped the cache.");
    println!("   No `unsafe`, no panic, no error -- a quietly wrong answer.");
    println!();

    println!("3. Where the audit has to look");
    println!("   `total` is private, so only this module can desync it.");
    println!("   The suspect list is every fn in `mod tally`, not every caller.");
    println!();

    println!("4. Three fixes, in the order to prefer them");
    println!("   a. delete `record_unchecked` -- the invariant has one door again");
    println!("   b. make it private, so the leak cannot escape the module");
    println!("   c. keep it and call `resync` -- last resort, because it is");
    println!("      now a rule a reader has to remember rather than one the");
    println!("      type enforces");
    t.resync();
    println!("   after resync: total() = {}   holds() = {}", t.total(), t.holds());
    println!();

    println!("5. Make it checkable, not just documented");
    debug_assert!(t.holds(), "Tally invariant: total == sum(counts)");
    println!("   debug_assert!(t.holds()) costs nothing in release and turns");
    println!("   the comment into something a test run can fail on.");
}
