//! Kata solution: five paths to one function, from four vantage points.
//!
//!   rustc --edition 2024 one_module_per_file_kata.rs -o /tmp/ompk && /tmp/ompk

mod election {
    pub mod ballots {
        pub fn count() -> u32 {
            12
        }

        pub mod spoiled {
            /// From two levels down: `super::super::` or `crate::`.
            pub fn share() -> f64 {
                let total = super::count();
                let bad = 1;
                f64::from(bad) / f64::from(total)
            }

            /// The same thing written absolutely.
            pub fn share_absolute() -> f64 {
                let total = crate::election::ballots::count();
                f64::from(1u32) / f64::from(total)
            }
        }
    }

    pub mod tally {
        /// A sibling module: up one, then down.
        pub fn total() -> u32 {
            super::ballots::count() * 5
        }

        /// `self::` is optional and occasionally load-bearing — it forces the
        /// name to resolve as a module path rather than anything in scope.
        pub fn total_twice() -> u32 {
            self::total() * 2
        }
    }
}

use election::ballots::spoiled;

fn main() {
    println!("1. The tree, as files");
    println!("   src/main.rs                     mod election;");
    println!("   src/election.rs                 mod ballots;  mod tally;");
    println!("   src/election/ballots.rs         mod spoiled;");
    println!("   src/election/ballots/spoiled.rs");
    println!("   src/election/tally.rs");
    println!("   Note ballots.rs is BOTH a module and a directory's parent. That");
    println!("   is the whole 2018 change: a file beside a folder of the same");
    println!("   name, instead of mod.rs inside it.");

    println!();
    println!("2. Four ways to name the same function");
    println!("   from spoiled  : super::count()                    = {}",
             election::ballots::count());
    println!("   from spoiled  : crate::election::ballots::count() = {}",
             election::ballots::count());
    println!("   from tally    : super::ballots::count()           = {}",
             election::ballots::count());
    println!("   from main     : election::ballots::count()        = {}",
             election::ballots::count());
    println!("   All four resolve to one function. The path is relative to where");
    println!("   you are STANDING in the module tree, and the tree came from the");
    println!("   `mod` declarations rather than from the directories.");

    println!();
    println!("3. Running them");
    println!("   spoiled::share()          = {:.4}", spoiled::share());
    println!("   spoiled::share_absolute() = {:.4}", spoiled::share_absolute());
    println!("   election::tally::total()       = {}", election::tally::total());
    println!("   election::tally::total_twice() = {}", election::tally::total_twice());

    println!();
    println!("4. Which form to reach for");
    println!("   crate::  when the item is far away, or the module might move.");
    println!("            Absolute paths survive a rename of the module you are IN.");
    println!("   super::  for a sibling, once. Two `super::super::` in a row is a");
    println!("            sign the tree is wrong, not that you need a third.");
    println!("   self::   rarely; it disambiguates a module path from a local name.");
    println!("   a `use`  when the same path appears more than twice in a file.");

    println!();
    println!("5. The failure that has no error message");
    println!("   Add src/election/recount.rs and forget `mod recount;` in");
    println!("   election.rs, and the file is simply not part of the crate. It is");
    println!("   not compiled, so its syntax errors do not appear, its tests do");
    println!("   not run, and nothing warns. `cargo build` succeeds. That is the");
    println!("   most common \"why is my code not running\" in a new Rust project,");
    println!("   and the answer is always the missing declaration.");
}
