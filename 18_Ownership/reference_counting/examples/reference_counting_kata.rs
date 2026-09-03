//! Kata solution: predict the count four times, then find the edge that leaks.
//!
//!   rustc --edition 2024 reference_counting_kata.rs -o /tmp/rck && /tmp/rck

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Part 1 and 2: one roster, several readers.
struct Scoreboard {
    roster: Rc<Vec<String>>,
    points: Vec<u32>,
}

impl Scoreboard {
    fn new(roster: &Rc<Vec<String>>) -> Self {
        Scoreboard { roster: Rc::clone(roster), points: vec![0; roster.len()] }
    }
    fn score(&mut self, i: usize) {
        self.points[i] += 1;
    }
    fn leader(&self) -> &str {
        let best = self
            .points
            .iter()
            .enumerate()
            .max_by_key(|(i, v)| (**v, std::cmp::Reverse(*i)))
            .map(|(i, _)| i)
            .unwrap();
        &self.roster[best]
    }
}

/// Part 3: a team owns its members, and each member refers back to it.
struct Team {
    name: &'static str,
    members: RefCell<Vec<Rc<Member>>>,
}

struct Member {
    name: &'static str,
    /// The back edge. `Weak` is the answer; `Rc<Team>` here is the leak.
    team: RefCell<Weak<Team>>,
}

impl Drop for Team {
    fn drop(&mut self) {
        println!("     drop ran for team {}", self.name);
    }
}

impl Drop for Member {
    fn drop(&mut self) {
        println!("     drop ran for member {}", self.name);
    }
}

fn enroll(team: &Rc<Team>, name: &'static str) {
    let member = Rc::new(Member { name, team: RefCell::new(Weak::new()) });
    *member.team.borrow_mut() = Rc::downgrade(team);
    team.members.borrow_mut().push(member);
}

fn main() {
    println!("Part 1 — predict the count at four points.\n");

    let roster = Rc::new(vec!["Ada".to_string(), "Ben".to_string(), "Cara".to_string()]);
    println!("  (a) let roster = Rc::new(..)        predicted 1  actual {}", Rc::strong_count(&roster));

    let mut morning = Scoreboard::new(&roster);
    let mut evening = Scoreboard::new(&roster);
    println!("  (b) two Scoreboard::new(&roster)    predicted 3  actual {}", Rc::strong_count(&roster));

    {
        let _spare = Scoreboard::new(&roster);
        println!("  (c) a third, inside a block         predicted 4  actual {}", Rc::strong_count(&roster));
    }
    println!("  (d) the block ended                 predicted 3  actual {}", Rc::strong_count(&roster));

    println!("\n  The count tracks OWNERS, not uses. `morning` and `evening` are");
    println!("  still alive at (d), so the roster is; `_spare` is not, so its");
    println!("  share went back. The Vec is freed when the last one leaves, and");
    println!("  no single owner's scope decides that.");

    morning.score(0);
    morning.score(0);
    morning.score(1);
    evening.score(2);
    evening.score(2);
    evening.score(2);
    println!("\n  Three Scoreboard values, one roster, three names stored once:");
    println!("    morning leader {}   evening leader {}", morning.leader(), evening.leader());

    println!("\nPart 2 — swap one Rc::clone for a deep clone. Which numbers move?\n");
    let independent: Vec<String> = (*roster).clone();
    println!("  Rc::strong_count(&roster)   {}   <- unchanged: a deep clone", Rc::strong_count(&roster));
    println!("                                   makes a value with NO owner in");
    println!("                                   common with this one.");
    println!("  same buffer as the roster?  {}", roster.as_ptr() == independent.as_ptr());
    println!("  Three fresh Strings and a fresh Vec, so a name edited here would");
    println!("  not be seen by any Scoreboard. That divergence is the bug the count");
    println!("  was preventing, and it compiles either way.");

    println!("\nPart 3 — the back edge, and whether Drop runs.\n");
    println!("  With `team: RefCell<Rc<Team>>` the prediction is NOTHING");
    println!("  prints: the team owns each member and each member owns the");
    println!("  team, so both counts stop at 1 and neither reaches zero.");
    println!("  `Weak` breaks it — a member can SEE its team without owning it:\n");
    {
        let riverside = Rc::new(Team { name: "Riverside", members: RefCell::new(Vec::new()) });
        enroll(&riverside, "Ada");
        enroll(&riverside, "Ben");
        let ada = Rc::clone(&riverside.members.borrow()[0]);
        let seen = ada.team.borrow().upgrade().map(|t| t.name);
        println!("    Ada can still reach her team: {seen:?}");
        println!("    strong team {}   weak team {}   strong Ada {}",
                 Rc::strong_count(&riverside),
                 Rc::weak_count(&riverside),
                 Rc::strong_count(&ada));
        println!("    leaving the block:");
    }
    println!("\n  All three ran, in owner order: the team's count hit zero");
    println!("  first, which dropped its Vec, which dropped the members.");
    println!("  The rule to carry away: a cycle needs every edge to be an owner,");
    println!("  so make exactly one of them an observer. Child-to-parent is");
    println!("  almost always the one to demote.");
}
