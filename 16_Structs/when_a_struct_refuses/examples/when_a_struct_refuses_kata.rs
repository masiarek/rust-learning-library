//! Kata solution: seven errors, five root causes, three edits.
//!
//!   rustc --edition 2024 when_a_struct_refuses_kata.rs -o /tmp/wsrk && /tmp/wsrk

use std::fmt;

// The struct as it arrives is four lines and refuses seven times:
//
//     #[derive(Debug, Default, Eq)]
//     struct Player { name: str, level: u8 }
//     impl Default for Player { fn default() -> Self { unimplemented!() } }
//     fn main() { let v = Player { level: 1 }; println!("{}", v); }
//
// Fixed, with a note on each edit:

#[derive(Debug, PartialEq, Eq)] // edit 2: Default dropped (it clashes), PartialEq added
struct Player {
    name: String, // edit 1: str -> String
    level: u8,
}

impl Default for Player {
    fn default() -> Self {
        Player { name: String::from("anonymous"), level: 0 }
    }
}

impl fmt::Display for Player {
    // edit 3: `{}` needs this written by hand
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "level {} — {}", self.level, self.name)
    }
}

fn main() {
    println!("The error COUNT is not the work count.\n");

    println!("  7 errors   as it arrives");
    println!("  4 errors   after edit 1: `name: str` -> `name: String`");
    println!("  2 errors   after edit 2: drop derived Default, add PartialEq");
    println!("  0 errors   after edit 3: impl Display, and name every field\n");

    println!("Edit 1 removed THREE of the seven, because one root cause produced them:");
    println!("  E0277  str doesn't have a size known at compile-time   (x2)");
    println!("  E0277  the trait bound `str: Default` is not satisfied");
    println!("An unsized field poisons every derive that has to touch it. Chasing the");
    println!("three separately would have been three investigations of one mistake.\n");

    println!("Edit 2 removed two more, and they were unrelated to each other:");
    println!("  E0119  conflicting implementations of `Default`  — derive AND impl");
    println!("  E0277  can't compare `Player` with `Player`        — Eq without PartialEq");
    println!("Keeping the hand-written Default is the right call: it knows a domain");
    println!("default the derive could never guess.");
    println!("  Player::default() = {}", Player::default());
    println!("  ...where the derive would have said name: \"\", level: 0\n");

    println!("Edit 3 was the only one rustc could not write for you:");
    println!("  E0063  missing field `name`   — it named the field");
    println!("  E0277  doesn't implement Display — it suggested {{:?}} instead");
    println!("The suggestion is a real option, and often the right one. Choosing to");
    println!("write Display means you decided a human reads this type.");
    let v = Player { name: String::from("Ada"), level: 7 };
    println!("  Display: {v}");
    println!("  Debug:   {v:?}");
    println!("  and the pair now compares: {}", v == Player { name: "Ada".into(), level: 7 });

    println!("\nThe habit: read all seven before editing any of them, and group them by");
    println!("root cause. rustc reports every error it can reach in one pass — it is not");
    println!("a queue to be worked one at a time.");
}
