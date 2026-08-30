//! `HashMap`: a key finds a value, and `entry` is the method the loop wants.
//!
//!   rustc --edition 2024 the_hashmap.rs -o /tmp/hm && /tmp/hm

use std::collections::{BTreeMap, HashMap};

/// Every print of a HashMap in this file goes through here. Iteration order is
/// deliberately not defined, so an example that printed it raw would be a
/// different answer key on every run.
fn sorted<'a>(m: &'a HashMap<&'a str, u32>) -> Vec<(&'a str, u32)> {
    let mut rows: Vec<(&str, u32)> = m.iter().map(|(k, v)| (*k, *v)).collect();
    rows.sort();
    rows
}

fn main() {
    let ballots = ["Cara", "Ada", "Cara", "Ben", "Cara", "Ada"];

    println!("1. Counting, the way you will actually write it");
    let mut tally: HashMap<&str, u32> = HashMap::new();
    for name in ballots {
        *tally.entry(name).or_insert(0) += 1;
    }
    println!("   {ballots:?}");
    println!("   -> {:?}", sorted(&tally));
    println!("   `entry(k).or_insert(0)` returns a &mut to the value, inserting");
    println!("   the default first if the key was absent. One lookup, not the");
    println!("   two that `if map.contains_key(k)` costs.");

    println!();
    println!("2. Reading: `get` asks, `[]` asserts");
    println!("   tally.get(\"Ada\")   = {:?}", tally.get("Ada"));
    println!("   tally.get(\"Nobody\") = {:?}", tally.get("Nobody"));
    println!("   tally[\"Ada\"]        = {}", tally["Ada"]);
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let missing = std::panic::catch_unwind(|| tally[std::hint::black_box("Nobody")]);
    std::panic::set_hook(hook);
    println!("   tally[\"Nobody\"]     -> {}", if missing.is_err() { "panicked" } else { "returned" });
    println!("   Same split as slice indexing: `[]` is a claim, `.get` is a question.");
    println!("   tally.get(\"x\").copied().unwrap_or(0) = {}", tally.get("x").copied().unwrap_or(0));

    println!();
    println!("3. `insert` returns what was there before");
    let mut m: HashMap<&str, u32> = HashMap::new();
    println!("   insert(\"Ada\", 1) -> {:?}   (nothing was there)", m.insert("Ada", 1));
    println!("   insert(\"Ada\", 9) -> {:?}   (the old value, handed back)", m.insert("Ada", 9));
    println!("   m is now {:?} — insert OVERWRITES. The return value is the only", sorted(&m));
    println!("   warning you get, and ignoring it is the usual way a duplicate key");
    println!("   silently loses a row.");

    println!();
    println!("4. Iteration order is not defined, and not stable between runs");
    println!("   sorted for printing:    {:?}", sorted(&tally));
    let ordered: BTreeMap<&str, u32> = tally.iter().map(|(k, v)| (*k, *v)).collect();
    println!("   the same in a BTreeMap: {:?}", ordered.iter().collect::<Vec<_>>());
    println!("   std's HashMap seeds its hasher randomly per process, so two runs");
    println!("   of the same program iterate in different orders. That is a defence");
    println!("   against hash-flooding, and it means any output you compare against");
    println!("   must be sorted first — or must come from a BTreeMap, which is");
    println!("   ordered by key by construction.");

    println!();
    println!("5. What a key has to be");
    println!("   `HashMap<K, V>` needs K: Eq + Hash. Both are derivable, and both");
    println!("   must agree: two keys that are `==` must hash the same, or lookups");
    println!("   miss entries that are provably in the map.");
    let winner = tally.iter().max_by_key(|(name, count)| (**count, std::cmp::Reverse(**name)));
    println!("   the max, tie broken by name: {winner:?}");
    println!("   `max_by_key` over a HashMap needs an explicit tiebreak for exactly");
    println!("   the reason above: with no tiebreak, a tie is resolved by whichever");
    println!("   equal entry the iterator happened to reach last.");
}
