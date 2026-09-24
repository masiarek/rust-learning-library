//! Kata solution: two days of sign-ins, one set per day.
//!
//!   rustc --edition 2024 first_hashset_kata.rs -o /tmp/fhsk && /tmp/fhsk

use std::collections::HashSet;

/// Sets print in no fixed order, so every set here goes through this first.
fn sorted<'a>(set: &'a HashSet<&'a str>) -> Vec<&'a str> {
    let mut items: Vec<&str> = set.iter().copied().collect();
    items.sort();
    items
}

fn main() {
    let monday = ["Ada", "Ben", "Ada", "Cara", "Ben"];
    let tuesday = ["Cara", "Dan", "Ben", "Dan"];

    println!("1. One set per day");
    let mon: HashSet<&str> = monday.into_iter().collect();
    let tue: HashSet<&str> = tuesday.into_iter().collect();
    println!("   Monday:  {} sign-ins, {} people: {:?}", monday.len(), mon.len(), sorted(&mon));
    println!("   Tuesday: {} sign-ins, {} people: {:?}", tuesday.len(), tue.len(), sorted(&tue));

    println!();
    println!("2. Three questions, answered with contains and intersection");
    println!("   was Ada there on Tuesday? contains(\"Ada\") = {}", tue.contains("Ada"));
    println!("   was Dan there on Monday?  contains(\"Dan\") = {}", mon.contains("Dan"));
    let mut both: Vec<&str> = mon.intersection(&tue).copied().collect();
    both.sort();
    println!("   came both days: {both:?}");

    println!();
    println!("3. Monday's sign-ins in first-seen order, repeats dropped by insert's bool");
    let mut seen: HashSet<&str> = HashSet::new();
    let mut order: Vec<&str> = Vec::new();
    for name in monday {
        if seen.insert(name) {
            order.push(name);
        }
    }
    println!("   {order:?}");
    println!("   insert returned false for the second \"Ada\" and the second \"Ben\", so");
    println!("   neither was pushed; the set remembers, the Vec keeps the order.");
}
