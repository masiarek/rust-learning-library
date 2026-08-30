//! `HashSet`: membership, uniqueness, and the four set operations.
//!
//!   rustc --edition 2024 the_hashset.rs -o /tmp/hs && /tmp/hs

use std::collections::HashSet;

fn sorted<'a>(s: &'a HashSet<&'a str>) -> Vec<&'a str> {
    let mut v: Vec<&str> = s.iter().copied().collect();
    v.sort();
    v
}

fn main() {
    println!("1. A set is a map with nothing on the right-hand side");
    println!("   HashSet<T> is literally HashMap<T, ()> underneath, so the same");
    println!("   rules apply: T: Eq + Hash, and no defined iteration order.");

    println!();
    println!("2. `insert` answers the question you were about to ask");
    let mut voted: HashSet<&str> = HashSet::new();
    for name in ["Ada", "Ben", "Ada", "Cara", "Ben"] {
        let first_time = voted.insert(name);
        println!("   insert(\"{name}\") -> {first_time}{}",
                 if first_time { "" } else { "   <- already voted" });
    }
    println!("   set: {:?}", sorted(&voted));
    println!("   The bool is `true` when the value was NOT already there, which");
    println!("   makes duplicate detection one line with no second lookup.");

    println!();
    println!("3. The four operations");
    let round1: HashSet<&str> = ["Ada", "Ben", "Cara"].into_iter().collect();
    let round2: HashSet<&str> = ["Ben", "Cara", "Dan"].into_iter().collect();
    let show = |label: &str, mut v: Vec<&str>| {
        v.sort();
        println!("   {label:<38} {v:?}");
    };
    show("round1 | round2  union", round1.union(&round2).copied().collect());
    show("round1 & round2  intersection", round1.intersection(&round2).copied().collect());
    show("round1 - round2  difference", round1.difference(&round2).copied().collect());
    show("round1 ^ round2  symmetric_difference",
         round1.symmetric_difference(&round2).copied().collect());
    println!("   Each returns an iterator, not a set — nothing is allocated until");
    println!("   you collect. The operator forms (`&`, `|`, `-`, `^`) exist too and");
    println!("   build a new HashSet directly.");

    println!();
    println!("4. Containment, both directions");
    println!("   round1.contains(\"Ada\")        = {}", round1.contains("Ada"));
    println!("   round1.is_subset(&round2)     = {}", round1.is_subset(&round2));
    println!("   round1.is_disjoint(&round2)   = {}", round1.is_disjoint(&round2));
    let core: HashSet<&str> = ["Ben", "Cara"].into_iter().collect();
    println!("   core.is_subset(&round1)       = {}", core.is_subset(&round1));

    println!();
    println!("5. Deduplicating, and what it costs");
    let raw = ["Cara", "Ada", "Cara", "Ben", "Ada", "Cara"];
    let unique: HashSet<&str> = raw.into_iter().collect();
    println!("   {raw:?}");
    println!("   -> {} distinct: {:?}", unique.len(), sorted(&unique));
    let mut kept_order: Vec<&str> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for name in raw {
        if seen.insert(name) {
            kept_order.push(name);
        }
    }
    println!("   first-seen order: {kept_order:?}");
    println!("   A HashSet throws the order away. If you need it, keep a Vec beside");
    println!("   the set and let `insert`'s bool decide whether to push.");
}
