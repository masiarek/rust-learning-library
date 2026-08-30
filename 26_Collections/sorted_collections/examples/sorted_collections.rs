//! Sorted collections: `BTreeMap` and `BTreeSet`.
//!
//!   rustc --edition 2024 sorted_collections.rs -o /tmp/sc && /tmp/sc

use std::collections::{BTreeMap, BTreeSet, HashMap};

const BALLOTS: [&str; 9] = ["Cara", "Ada", "Ben", "Cara", "Dan", "Ada", "Cara", "Ben", "Ada"];

fn main() {
    println!("1. Collecting into a BTreeMap is a sort you did not write");
    let mut tally: BTreeMap<&str, u32> = BTreeMap::new();
    for b in BALLOTS {
        *tally.entry(b).or_insert(0) += 1;
    }
    println!("   BTreeMap  {tally:?}");

    let mut hash: HashMap<&str, u32> = HashMap::new();
    for b in BALLOTS {
        *hash.entry(b).or_insert(0) += 1;
    }
    let mut pairs: Vec<(&str, u32)> = hash.into_iter().collect();
    pairs.sort();
    println!("   HashMap   {pairs:?}");
    println!("   The same four counts. The second line needed a Vec and a sort to be");
    println!("   printable in a fixed order at all; the first was already in one.");

    println!();
    println!("2. Ordering buys three questions a HashMap cannot answer");
    println!("   first_key_value()   {:?}", tally.first_key_value());
    println!("   last_key_value()    {:?}", tally.last_key_value());
    let window: Vec<(&&str, &u32)> = tally.range("B".."D").collect();
    println!("   range(\"B\"..\"D\")     {window:?}");
    println!("   A HashMap has no first, no last and no range: there is no order for");
    println!("   those questions to be asked against.");

    println!();
    println!("3. The trap: sorted by KEY, and what you wanted sorted is the value");
    for (name, votes) in &tally {
        println!("   {name} {votes}");
    }
    println!("   That is alphabetical, not a leaderboard. Votes-descending still");
    println!("   leaves the map:");
    let mut board: Vec<(&str, u32)> = tally.iter().map(|(k, v)| (*k, *v)).collect();
    board.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("   {board:?}");
    println!("   Ada and Cara tie at 3, and `then` breaks it by name — so the order is");
    println!("   total and the answer is the same on every run.");

    println!();
    println!("4. BTreeSet is BTreeMap<T, ()>: collecting into one sorts and dedups");
    let seen: BTreeSet<&str> = BALLOTS.into_iter().collect();
    println!("   {seen:?}");
    let eligible: BTreeSet<&str> = ["Ada", "Ben", "Cara", "Dan", "Eve"].into_iter().collect();
    let missing: Vec<&str> = eligible.difference(&seen).copied().collect();
    println!("   eligible but never voted  {missing:?}");
    println!("   The set operations yield their items in order too, so that Vec");
    println!("   needed no sort either.");

    println!();
    println!("5. The price is Ord, not Hash");
    let by_score: BTreeMap<u32, &str> = [(55, "Ben"), (91, "Ada"), (72, "Cara")]
        .into_iter()
        .collect();
    println!("   keyed on an integer score  {by_score:?}");
    println!("   Integers, char, &str, String, and tuples and Vecs of those are all");
    println!("   Ord. f64 is not — NaN leaves it PartialOrd only — so a BTreeMap keyed");
    println!("   on a float does not compile. Scale to an integer, as above.");
}
