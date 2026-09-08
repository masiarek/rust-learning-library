//! Kata solution: a closed set, and the match that will not compile without you.
//!
//!   rustc --edition 2024 what_an_enum_is_kata.rs -o /tmp/wei && /tmp/wei

use std::mem::size_of;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Status { Queued, Running, Done }

fn describe(s: Status) -> &'static str {
    match s {
        Status::Queued => "waiting for a worker",
        Status::Running => "in progress",
        Status::Done => "finished",
    }
}

fn main() {
    println!("A VALUE IS EXACTLY ONE OF THEM");
    for s in [Status::Queued, Status::Running, Status::Done] {
        println!("  {s:?}  ->  {}", describe(s));
    }
    println!();

    println!("WHAT IT COSTS");
    println!("  size_of::<Status>()  {} byte", size_of::<Status>());
    println!("  Three alternatives fit in one byte, because the value only has");
    println!("  to record WHICH of the three it is. A struct with three bools");
    println!("  would be three bytes and would also permit five states that");
    println!("  mean nothing.");
    println!();

    println!("THE PART THAT PAYS FOR ITSELF");
    println!("  Add a fourth variant -- `Status::Failed` -- and `describe` stops");
    println!("  compiling with E0004: \"non-exhaustive patterns: `Status::Failed`");
    println!("  not covered\". The compiler found every place in the program");
    println!("  that has to think about the new case, and named them.");
    println!("  That is the whole argument for a closed set: the set is written");
    println!("  down in one place, and changing it is a compile error");
    println!("  everywhere it matters rather than a silent fallthrough.");
    println!();

    println!("AND THE WAY TO GIVE THAT UP BY ACCIDENT");
    println!("  A `_ => ...` arm makes the match exhaustive forever. It is the");
    println!("  right thing when you genuinely mean \"anything else\", and it is");
    println!("  the wrong thing on a set you own -- because the next variant");
    println!("  will quietly take that arm instead of being reported.");
    println!();

    println!("COMPARING VALUES");
    println!("  Status::Queued == Status::Queued  {}", Status::Queued == Status::Queued);
    println!("  Status::Queued == Status::Done    {}", Status::Queued == Status::Done);
    println!("  PartialEq is derived, not free: without the derive, `==` on two");
    println!("  Status values does not compile. Rust asks you to say which");
    println!("  behaviours a type has rather than assuming them.");

    assert_eq!(size_of::<Status>(), 1);
    assert_eq!(describe(Status::Done), "finished");
}
