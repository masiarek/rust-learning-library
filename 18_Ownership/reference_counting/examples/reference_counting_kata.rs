//! Kata solution: predict the count four times, then find the edge that leaks.
//!
//!   rustc --edition 2024 reference_counting_kata.rs -o /tmp/rck && /tmp/rck

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Part 1 and 2: one roster, several readers.
struct Tally {
    roster: Rc<Vec<String>>,
    votes: Vec<u32>,
}

impl Tally {
    fn new(roster: &Rc<Vec<String>>) -> Self {
        Tally { roster: Rc::clone(roster), votes: vec![0; roster.len()] }
    }
    fn cast(&mut self, i: usize) {
        self.votes[i] += 1;
    }
    fn leader(&self) -> &str {
        let best = self
            .votes
            .iter()
            .enumerate()
            .max_by_key(|(i, v)| (**v, std::cmp::Reverse(*i)))
            .map(|(i, _)| i)
            .unwrap();
        &self.roster[best]
    }
}

/// Part 3: a precinct owns its voters, and each voter refers back to it.
struct Precinct {
    name: &'static str,
    voters: RefCell<Vec<Rc<Voter>>>,
}

struct Voter {
    name: &'static str,
    /// The back edge. `Weak` is the answer; `Rc<Precinct>` here is the leak.
    precinct: RefCell<Weak<Precinct>>,
}

impl Drop for Precinct {
    fn drop(&mut self) {
        println!("     drop ran for precinct {}", self.name);
    }
}

impl Drop for Voter {
    fn drop(&mut self) {
        println!("     drop ran for voter {}", self.name);
    }
}

fn enroll(precinct: &Rc<Precinct>, name: &'static str) {
    let voter = Rc::new(Voter { name, precinct: RefCell::new(Weak::new()) });
    *voter.precinct.borrow_mut() = Rc::downgrade(precinct);
    precinct.voters.borrow_mut().push(voter);
}

fn main() {
    println!("Part 1 — predict the count at four points.\n");

    let roster = Rc::new(vec!["Ada".to_string(), "Ben".to_string(), "Cara".to_string()]);
    println!("  (a) let roster = Rc::new(..)        predicted 1  actual {}", Rc::strong_count(&roster));

    let mut morning = Tally::new(&roster);
    let mut evening = Tally::new(&roster);
    println!("  (b) two Tally::new(&roster)         predicted 3  actual {}", Rc::strong_count(&roster));

    {
        let _absentee = Tally::new(&roster);
        println!("  (c) a third, inside a block         predicted 4  actual {}", Rc::strong_count(&roster));
    }
    println!("  (d) the block ended                 predicted 3  actual {}", Rc::strong_count(&roster));

    println!("\n  The count tracks OWNERS, not uses. `morning` and `evening` are");
    println!("  still alive at (d), so the roster is; `_absentee` is not, so its");
    println!("  share went back. The Vec is freed when the last one leaves, and");
    println!("  no single owner's scope decides that.");

    morning.cast(0);
    morning.cast(0);
    morning.cast(1);
    evening.cast(2);
    evening.cast(2);
    evening.cast(2);
    println!("\n  Three Tally values, one roster, three names stored once:");
    println!("    morning leader {}   evening leader {}", morning.leader(), evening.leader());

    println!("\nPart 2 — swap one Rc::clone for a deep clone. Which numbers move?\n");
    let independent: Vec<String> = (*roster).clone();
    println!("  Rc::strong_count(&roster)   {}   <- unchanged: a deep clone", Rc::strong_count(&roster));
    println!("                                   makes a value with NO owner in");
    println!("                                   common with this one.");
    println!("  same buffer as the roster?  {}", roster.as_ptr() == independent.as_ptr());
    println!("  Three fresh Strings and a fresh Vec, so a name edited here would");
    println!("  not be seen by any Tally. That divergence is the bug the count");
    println!("  was preventing, and it compiles either way.");

    println!("\nPart 3 — the back edge, and whether Drop runs.\n");
    println!("  With `precinct: RefCell<Rc<Precinct>>` the prediction is NOTHING");
    println!("  prints: the precinct owns each voter and each voter owns the");
    println!("  precinct, so both counts stop at 1 and neither reaches zero.");
    println!("  `Weak` breaks it — a voter can SEE its precinct without owning it:\n");
    {
        let riverside = Rc::new(Precinct { name: "Riverside", voters: RefCell::new(Vec::new()) });
        enroll(&riverside, "Ada");
        enroll(&riverside, "Ben");
        let ada = Rc::clone(&riverside.voters.borrow()[0]);
        let seen = ada.precinct.borrow().upgrade().map(|p| p.name);
        println!("    Ada can still reach her precinct: {seen:?}");
        println!("    strong precinct {}   weak precinct {}   strong Ada {}",
                 Rc::strong_count(&riverside),
                 Rc::weak_count(&riverside),
                 Rc::strong_count(&ada));
        println!("    leaving the block:");
    }
    println!("\n  All three ran, in owner order: the precinct's count hit zero");
    println!("  first, which dropped its Vec, which dropped the voters.");
    println!("  The rule to carry away: a cycle needs every edge to be an owner,");
    println!("  so make exactly one of them an observer. Child-to-parent is");
    println!("  almost always the one to demote.");
}
