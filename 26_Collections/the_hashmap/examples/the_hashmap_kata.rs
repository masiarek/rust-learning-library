//! Kata solution: four ways to count, and the two that are wrong.
//!
//!   rustc --edition 2024 the_hashmap_kata.rs -o /tmp/hmk && /tmp/hmk

use std::collections::HashMap;

const BALLOTS: [(&str, u32); 7] = [
    ("Cara", 5),
    ("Ada", 3),
    ("Cara", 4),
    ("Ben", 0),
    ("Cara", 2),
    ("Ada", 5),
    ("Dan", 1),
];

fn sorted<'a>(m: &'a HashMap<&'a str, u32>) -> Vec<(&'a str, u32)> {
    let mut rows: Vec<(&str, u32)> = m.iter().map(|(k, v)| (*k, *v)).collect();
    rows.sort();
    rows
}

fn main() {
    println!("1. entry().or_insert() — one lookup, and the right answer");
    let mut totals: HashMap<&str, u32> = HashMap::new();
    for (name, score) in BALLOTS {
        *totals.entry(name).or_insert(0) += score;
    }
    println!("   {:?}", sorted(&totals));

    println!();
    println!("2. or_default() — the same, with the type choosing the zero");
    let mut d: HashMap<&str, u32> = HashMap::new();
    for (name, score) in BALLOTS {
        *d.entry(name).or_default() += score;
    }
    println!("   {:?}   identical: {}", sorted(&d), sorted(&d) == sorted(&totals));

    println!();
    println!("3. The wrong one that compiles: insert in the loop");
    let mut lost: HashMap<&str, u32> = HashMap::new();
    for (name, score) in BALLOTS {
        lost.insert(name, score);
    }
    println!("   {:?}", sorted(&lost));
    println!("   Cara scored 5 + 4 + 2 = 11 and this says {}. `insert` overwrites,",
             lost["Cara"]);
    println!("   so every repeat key threw away the running total. Nothing warns:");
    println!("   the return value that would have told you is discarded.");

    println!();
    println!("4. The wrong one that is subtler: contains_key then insert");
    let mut two_pass: HashMap<&str, u32> = HashMap::new();
    for (name, score) in BALLOTS {
        if two_pass.contains_key(name) {
            let old = two_pass[name];
            two_pass.insert(name, old + score);
        } else {
            two_pass.insert(name, score);
        }
    }
    println!("   {:?}   correct: {}", sorted(&two_pass), sorted(&two_pass) == sorted(&totals));
    println!("   Right answer, three hash lookups per ballot instead of one, and");
    println!("   five lines where `entry` is one. This is the shape a Python or");
    println!("   Java habit produces, and it is why `entry` exists.");

    println!();
    println!("5. and_modify().or_insert() — when the first sighting is special");
    let mut seen: HashMap<&str, u32> = HashMap::new();
    for (name, _) in BALLOTS {
        seen.entry(name).and_modify(|n| *n += 1).or_insert(1);
    }
    println!("   ballots per voter: {:?}", sorted(&seen));
    println!("   `or_insert(1)` runs only for a name never seen before, so the");
    println!("   two branches can differ. With `or_insert(0)` and a `+= 1` after,");
    println!("   they cannot.");

    println!();
    println!("6. The answer the tally was for");
    let mut rows = sorted(&totals);
    rows.sort_by_key(|(name, total)| (std::cmp::Reverse(*total), *name));
    println!("   ranked: {rows:?}");
    println!("   Sorting a HashMap means leaving it: collect the pairs into a Vec");
    println!("   and sort that. A hash map has no order to sort in place.");
}
