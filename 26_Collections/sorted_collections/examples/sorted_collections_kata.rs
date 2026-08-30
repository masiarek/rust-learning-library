//! Kata solution: two orders from one tally, and the key you cannot use.
//!
//!   rustc --edition 2024 sorted_collections_kata.rs -o /tmp/sck && /tmp/sck

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

const BALLOTS: [&str; 11] = [
    "Cara", "Ada", "Ben", "Cara", "Dan", "Ada", "Cara", "Ben", "Ada", "Eve", "Dan",
];

/// The tally. Alphabetical order is a property of the container, not of this code.
fn tally() -> BTreeMap<&'static str, u32> {
    let mut t = BTreeMap::new();
    for b in BALLOTS {
        *t.entry(b).or_insert(0) += 1;
    }
    t
}

fn main() {
    let t = tally();

    println!("1. The roll, alphabetical — nothing here sorts");
    for (name, votes) in &t {
        println!("   {name} {votes}");
    }

    println!();
    println!("2. The leaderboard, by leaving the map");
    let mut board: Vec<(&str, u32)> = t.iter().map(|(k, v)| (*k, *v)).collect();
    board.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for (name, votes) in &board {
        println!("   {name} {votes}");
    }
    println!("   Two ties — Ada/Cara at 3 and Ben/Dan at 2 — and `then` settles both");
    println!("   by name. Without it `sort_by` is still *stable*, so the order would");
    println!("   be the map's alphabetical one; but the rule would be implicit, and a");
    println!("   switch to sort_unstable_by would silently change the answer.");

    println!();
    println!("3. The leaderboard, by making the count part of the key");
    let ranked: BTreeSet<(Reverse<u32>, &str)> =
        t.iter().map(|(k, v)| (Reverse(*v), *k)).collect();
    for (Reverse(votes), name) in &ranked {
        println!("   {name} {votes}");
    }
    println!("   Same order, and no sort call anywhere. A tuple is Ord when its parts");
    println!("   are, compared left to right, so (Reverse(count), name) orders by");
    println!("   count descending and then by name — the tie-break is in the type.");

    println!();
    println!("4. A question a HashMap cannot be asked");
    let window: Vec<&str> = t.range("B".."D").map(|(k, _)| *k).collect();
    println!("   candidates in \"B\"..\"D\"  {window:?}");
    println!("   half-open, so \"D\" is a bound and Dan is excluded; range(\"B\"..=\"D\")");
    println!("   would still exclude him, because \"Dan\" > \"D\" as a string.");
    let inclusive: Vec<&str> = t.range("B"..="D").map(|(k, _)| *k).collect();
    println!("   candidates in \"B\"..=\"D\" {inclusive:?}");

    println!();
    println!("5. The key you cannot use");
    // let mut by_share: BTreeMap<f64, &str> = BTreeMap::new();
    // by_share.insert(0.273, "Ada");        // E0277: the trait bound `f64: Ord` is not satisfied
    println!("   f64 is PartialOrd but not Ord, because NaN compares false against");
    println!("   everything including itself, so a BTreeMap keyed on one does not");
    println!("   compile. Scale to an integer and the ordering survives:");
    let total: u32 = t.values().sum();
    let by_share: BTreeMap<u32, &str> = t.iter().map(|(k, v)| (v * 10_000 / total, *k)).collect();
    println!("   share in basis points  {by_share:?}");

    println!();
    println!("6. ...and look at what that just cost");
    println!("   Five candidates went in and {} came out.", by_share.len());
    println!("   Ada and Cara both hold 2727 basis points, Ben and Dan both 1818, and");
    println!("   a map keeps one value per key — so the second insert of each pair");
    println!("   overwrote the first. Keying on a DERIVED value drops ties silently:");
    println!("   nothing errors, the type is right, and two candidates are gone.");
    let by_share_kept: BTreeSet<(u32, &str)> =
        t.iter().map(|(k, v)| (v * 10_000 / total, *k)).collect();
    println!("   as a set of pairs      {by_share_kept:?}");
    println!("   The pair is the key, so equal shares no longer collide — and the");
    println!("   ordering is still share-then-name, for the same left-to-right reason");
    println!("   as the leaderboard in 3.");
}
