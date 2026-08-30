//! Kata solution: who voted twice, who never voted, and one wrong answer.
//!
//!   rustc --edition 2024 the_hashset_kata.rs -o /tmp/hsk && /tmp/hsk

use std::collections::{BTreeSet, HashSet};

const ROLL: [&str; 6] = ["Ada", "Ben", "Cara", "Dan", "Eve", "Fay"];
const RECEIVED: [&str; 8] = ["Cara", "Ada", "Ben", "Cara", "Dan", "Ada", "Cara", "Zed"];

fn sorted<'a>(s: &'a HashSet<&'a str>) -> Vec<&'a str> {
    let mut v: Vec<&str> = s.iter().copied().collect();
    v.sort();
    v
}

fn main() {
    println!("1. Three questions, one pass");
    let roll: HashSet<&str> = ROLL.into_iter().collect();
    let mut seen: HashSet<&str> = HashSet::new();
    let mut duplicates: Vec<&str> = Vec::new();
    for name in RECEIVED {
        if !seen.insert(name) {
            duplicates.push(name);
        }
    }
    duplicates.sort();
    let never: Vec<&str> = {
        let mut v: Vec<&str> = roll.difference(&seen).copied().collect();
        v.sort();
        v
    };
    let unknown: Vec<&str> = {
        let mut v: Vec<&str> = seen.difference(&roll).copied().collect();
        v.sort();
        v
    };
    println!("   roll     : {ROLL:?}");
    println!("   received : {RECEIVED:?}");
    println!("   voted twice or more : {duplicates:?}");
    println!("   on the roll, never voted : {never:?}");
    println!("   voted but not on the roll : {unknown:?}");
    println!("   The last two are the SAME operation with the arguments swapped.");
    println!("   `difference` is not symmetric, and reading it as \"the difference");
    println!("   between the sets\" is how the two get confused.");

    println!();
    println!("2. The wrong answer: counting duplicates with the set alone");
    println!("   received.len() = {}, distinct = {}, so {} extra ballots arrived.",
             RECEIVED.len(), seen.len(), RECEIVED.len() - seen.len());
    println!("   That is a count of surplus ballots, not of people: Cara sent three,");
    println!("   which is 2 of those {}. distinct-vs-total tells you HOW MANY too",
             RECEIVED.len() - seen.len());
    println!("   many, never WHO — for that you need the `insert` bool above, or a");
    println!("   count per name in a HashMap.");
    println!("   people who repeated: {} ({:?})",
             duplicates.iter().collect::<HashSet<_>>().len(),
             {
                 let mut d: Vec<&str> = duplicates.iter().copied().collect::<HashSet<_>>()
                     .into_iter().collect();
                 d.sort();
                 d
             });

    println!();
    println!("3. Turnout, as a set operation");
    let turned_out: HashSet<&str> = roll.intersection(&seen).copied().collect();
    println!("   eligible and voted: {:?}", sorted(&turned_out));
    println!("   turnout = {}/{} = {:.0}%", turned_out.len(), roll.len(),
             100.0 * turned_out.len() as f64 / roll.len() as f64);
    println!("   Note it is the intersection, not received.len(): the stray ballot");
    println!("   from Zed would otherwise inflate turnout above the electorate.");

    println!();
    println!("4. When you want the order back");
    let ordered: BTreeSet<&str> = seen.iter().copied().collect();
    println!("   BTreeSet: {:?}", ordered);
    println!("   Sorted by construction, O(log n) instead of O(1) per operation,");
    println!("   and it needs Ord rather than Hash. For six names the difference is");
    println!("   nothing and the printout is stable — which is why this library's");
    println!("   examples reach for it whenever output is the point.");
}
