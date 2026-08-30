//! Kata solution: six questions about one line, and only one of them needs a `Vec`.
//!
//!   rustc --edition 2024 the_cheapest_answer_kata.rs -o /tmp/tca && /tmp/tca

use std::cell::Cell;
use std::collections::HashSet;

fn main() {
    let roster = "cara:ada:ben:ada";
    let seen = Cell::new(0usize);
    let count = |p| {
        seen.set(seen.get() + 1);
        p
    };
    let reset = || seen.set(0);

    println!("roster = {roster:?}   (4 names, one of them twice)");
    println!();

    // 1 -----------------------------------------------------------------
    reset();
    let n = roster.split(':').map(count).count();
    println!("1. How many names?            {n}");
    println!("   .count()          — {} pieces walked, nothing kept", seen.get());

    // 2 -----------------------------------------------------------------
    reset();
    let first = roster.split(':').map(count).min();
    println!("2. Alphabetically first?      {first:?}");
    println!("   .min()            — {} pieces walked, one &str kept", seen.get());

    // 3 -----------------------------------------------------------------
    reset();
    let last = roster.split(':').map(count).next_back();
    println!("3. The last name?             {last:?}");
    println!("   .next_back()      — {} piece walked: it read from the far end", seen.get());

    // 4 -----------------------------------------------------------------
    reset();
    print!("4. In reverse order?         ");
    for name in roster.split(':').map(count).rev() {
        print!(" {name}");
    }
    println!();
    println!("   .rev()            — no sort, no second pass, and no Vec: the");
    println!("                       names went straight to the screen. `Split` walks");
    println!("                       backwards on its own, so reversing is free.");
    println!("                       True for split(':') and NOT for split(\":\") —");
    println!("                       only the char searcher is double-ended.");

    // 5 -----------------------------------------------------------------
    reset();
    let mut met = HashSet::new();
    let repeated = roster.split(':').map(count).any(|name| !met.insert(name));
    let walked_here = seen.get();
    reset();
    let mut met_early = HashSet::new();
    let early = "cara:ada:ada:ben";
    let repeated_early = early.split(':').map(count).any(|name| !met_early.insert(name));
    println!("5. Any name twice?            {repeated}");
    println!("   HashSet + .any()  — {walked_here} pieces walked here, because the repeat");
    println!("                       IS the last name. On {early:?}");
    println!("                       the same line answers {repeated_early} after {} pieces and stops.", seen.get());
    println!("                       A collection, but a set — with a Vec you would");
    println!("                       be writing the inner loop yourself.");

    // 6 -----------------------------------------------------------------
    reset();
    let mut sorted: Vec<&str> = roster.split(':').map(count).collect();
    sorted.sort_unstable();
    println!("6. The names in order?        {sorted:?}");
    println!("   collect + sort    — {} pieces walked, all of them kept. THIS is", seen.get());
    println!("                       the question that needs the Vec: sorting cannot");
    println!("                       begin until the last name has arrived, which is");
    println!("                       why there is no `Iterator::sort` to reach for.");
    println!();

    println!("Four of the six answers built nothing at all; the fifth wanted a set");
    println!("rather than a Vec. The reflex to `collect` first and ask afterwards");
    println!("pays for a Vec on all six — and question 3 shows what that costs");
    println!("beyond the allocation: collecting reads the whole line to find a name");
    println!("the iterator hands back after a single step.");
}
