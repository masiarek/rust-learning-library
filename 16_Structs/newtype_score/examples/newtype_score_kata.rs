//! Kata solution: make the invalid score impossible to build.
//!
//!   rustc --edition 2024 newtype_score_kata.rs -o /tmp/nsk && /tmp/nsk

mod rating {
    /// One private field, so the only way in is the door below.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Score(u8);

    impl Score {
        pub const MAX: u8 = 5;

        /// The validating constructor — the whole point of the type.
        pub fn new(n: u8) -> Option<Score> {
            (n <= Self::MAX).then_some(Score(n))
        }

        pub fn get(self) -> u8 {
            self.0
        }
    }

    /// Inside the module the field IS reachable — privacy is per module, not
    /// per struct. This is why the door only helps if the module stays small.
    pub fn unchecked_backdoor(n: u8) -> Score {
        Score(n)
    }

    /// Downstream code needs no checks: an out-of-range Score cannot exist.
    pub fn total(scores: &[Score]) -> u32 {
        scores.iter().map(|s| s.get() as u32).sum()
    }
}

use rating::Score;

fn main() {
    println!("The door validates, and says so in the type:");
    for n in [0, 5, 9] {
        println!("  Score::new({n}) -> {:?}", Score::new(n));
    }

    let review: Vec<Score> = [5, 3, 0].iter().filter_map(|n| Score::new(*n)).collect();
    println!("\n  review -> {review:?}");
    println!("  total  -> {}", rating::total(&review));
    println!("      `total` does no range checking. It cannot need any: every");
    println!("      value that reaches it came through Score::new.");

    println!("\nOutside the module, the field is unreachable:");
    println!("  `Score(9)` here is E0423 — cannot initialize a tuple struct");
    println!("  which contains private fields. The invalid value is not merely");
    println!("  discouraged; there is no syntax for it.");

    println!("\nBut privacy is per MODULE, so the module itself can cheat:");
    let smuggled = rating::unchecked_backdoor(9);
    println!("  unchecked_backdoor(9) -> {smuggled:?}   (score {})", smuggled.get());
    println!("      Nothing outside `mod rating` could have written that. Keep the");
    println!("      module small and the invariant holds; grow it and every new");
    println!("      function in there is another place the guarantee can leak.");
}
