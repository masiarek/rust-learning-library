//! A star rating is not a number — it is an integer 0..=5.
//!
//! `u8` holds 256 values. A 0-5 star rating has 6 legal ones. This program is
//! about the gap between those two facts, and about paying the validation cost
//! exactly once instead of at every call site that ever touches a rating.

mod rating {
    /// A validated 0-5 rating. The field is private *outside this module*, so
    /// `Score::new` is the only door in.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Score(u8);

    impl Score {
        pub const MAX: u8 = 5;

        /// The door. Partial function made total: every `u8` gets an answer.
        pub fn new(raw: u8) -> Option<Score> {
            if raw <= Self::MAX {
                Some(Score(raw))
            } else {
                None
            }
        }

        pub fn value(self) -> u8 {
            self.0
        }
    }

    /// The same rule expressed as six variants. There is no seventh to write.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Stars {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
    }

    impl Stars {
        pub fn new(raw: u8) -> Option<Stars> {
            use Stars::*;
            Some(match raw {
                0 => Zero,
                1 => One,
                2 => Two,
                3 => Three,
                4 => Four,
                5 => Five,
                _ => return None,
            })
        }

        pub fn value(self) -> u8 {
            self as u8
        }
    }
}

use rating::{Score, Stars};

/// Sums whatever it is handed. It cannot know whether the cells are legal.
fn total_raw(row: &[u8]) -> u32 {
    row.iter().map(|&s| s as u32).sum()
}

/// Sums scores. There is nothing here to validate, and that is the point.
fn total(row: &[Score]) -> u32 {
    row.iter().map(|s| s.value() as u32).sum()
}

fn rule(title: &str) {
    println!("\n──── {title}");
}

fn main() {
    rule("Step 1: A bare u8 has no opinion about the scale");

    let honest: [u8; 3] = [5, 3, 0];
    let typo: [u8; 3] = [5, 30, 0]; // a slipped keystroke: 3 became 30
    println!("  total_raw([5, 3, 0])  = {}", total_raw(&honest));
    println!("  total_raw([5, 30, 0]) = {}", total_raw(&typo));
    println!("      Both compile, both run, both return a number. The second");
    println!("      review just outweighed ten honest ones, and no type objected.");

    rule("Step 2: Score::new is the only door, and it is checked once");

    for raw in [0u8, 3, 5, 6, 30, 255] {
        match Score::new(raw) {
            Some(s) => println!("  Score::new({raw:<3}) -> Some({})", s.value()),
            None => println!("  Score::new({raw:<3}) -> None"),
        }
    }
    println!("      Same shape as any partial function: a u8 that is out of");
    println!("      range has no Score, so the answer is None rather than a lie.");

    rule("Step 3: A whole review row, accepted or rejected as a unit");

    for row in [[5u8, 3, 0], [5, 30, 0]] {
        let parsed: Option<Vec<Score>> = row.iter().map(|&c| Score::new(c)).collect();
        match parsed {
            Some(scores) => println!("  {row:?} -> accepted, {} cells", scores.len()),
            None => println!("  {row:?} -> rejected (one bad cell voids the row)"),
        }
    }
    println!("      collect() into Option<Vec<_>> stops at the first None. One");
    println!("      illegal cell means you never hold a half-valid review.");

    rule("Step 4: Downstream code has nothing left to check");

    let row: Vec<Score> = [5u8, 3, 0].iter().filter_map(|&c| Score::new(c)).collect();
    println!("  total(&row)          = {}", total(&row));
    println!("  row.iter().max()     = {:?}", row.iter().max().map(|s| s.value()));
    println!("      `total` does no validation because an invalid Score does not");
    println!("      exist. Ord comes free from the derive, so max/sort behave");
    println!("      like the numbers they wrap — which is what sorting by rating needs.");

    rule("Step 5: What actually closes the door is the module boundary");

    println!("  Inside `mod rating`, `Score(7)` still compiles — privacy is per");
    println!("  module, not per type. Outside it, the same expression is a");
    println!("  compile error: E0423, \"cannot initialize a tuple struct which");
    println!("  contains private fields\".");
    println!("      So the newtype is only as strong as the boundary you put it");
    println!("      behind. One file with no `mod` is a convention, not a wall.");

    rule("Step 6: The stronger design is also the smaller one");

    println!(
        "  size_of::<Score>()          = {}",
        std::mem::size_of::<Score>()
    );
    println!(
        "  size_of::<Option<Score>>()  = {}",
        std::mem::size_of::<Option<Score>>()
    );
    println!(
        "  size_of::<Stars>()          = {}",
        std::mem::size_of::<Stars>()
    );
    println!(
        "  size_of::<Option<Stars>>()  = {}",
        std::mem::size_of::<Option<Stars>>()
    );
    println!(
        "  Stars::new(4) -> {:?} (value {:?})",
        Stars::new(4),
        Stars::new(4).map(|s| s.value())
    );
    println!("  Stars::new(9) -> {:?}", Stars::new(9));
    println!("      Score wraps a u8 that uses all 256 patterns, so Option needs");
    println!("      a second byte for the tag. Stars uses 6 of 256, and Option");
    println!("      hides None in one of the 250 spare patterns — free.");
}
