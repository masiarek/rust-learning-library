//! A first HashSet: insert's bool, contains, remove, take, iteration, and a set from a Vec.
//!
//!   rustc --edition 2024 first_hashset.rs -o /tmp/fhs && /tmp/fhs

use std::any::type_name_of_val;
use std::collections::{BTreeSet, HashSet};

/// A HashSet has no order, so this is how every set on this page is printed:
/// copied into a Vec and sorted.
fn sorted<'a>(set: &'a HashSet<&'a str>) -> Vec<&'a str> {
    let mut items: Vec<&str> = set.iter().copied().collect();
    items.sort();
    items
}

fn main() {
    println!("1. HashSet::new(), then insert: true means it went in");
    let mut words: HashSet<&str> = HashSet::new();
    for word in ["Hello", "world", "Rust", "Programming"] {
        let went_in = words.insert(word);
        println!("   insert({word:?}) = {went_in}");
    }
    println!("   len = {}   sorted: {:?}", words.len(), sorted(&words));

    println!();
    println!("2. A second insert of the same value: false, and nothing changes");
    let again = words.insert("Hello");
    println!("   insert(\"Hello\") = {again}   type: {}   len = {}", type_name_of_val(&again), words.len());

    println!();
    println!("3. contains: bool");
    let has = words.contains("world");
    println!("   contains(\"world\") = {has}   contains(\"World\") = {}   type: {}", words.contains("World"), type_name_of_val(&has));

    println!();
    println!("4. remove: bool, true only the first time");
    let removed = words.remove("Rust");
    let removed_again = words.remove("Rust");
    println!("   remove(\"Rust\") = {removed}   remove(\"Rust\") again = {removed_again}   len = {}", words.len());

    println!();
    println!("5. take: the value itself, wrapped in an Option");
    let taken = words.take("world");
    let taken_again = words.take("world");
    println!("   take(\"world\") = {taken:?}   take(\"world\") again = {taken_again:?}   len = {}", words.len());
    if let Some(word) = taken {
        println!("   Removed: {word}");
    }

    println!();
    println!("6. Iterating: every value once, in no particular order");
    let mut seen: Vec<&str> = Vec::new();
    for word in &words {
        seen.push(*word);
    }
    seen.sort();
    println!("   sorted after the loop: {seen:?}");
    let ordered: BTreeSet<&str> = words.iter().copied().collect();
    println!("   the same in a BTreeSet: {ordered:?}");
    println!("   The loop's own order changes from run to run, which is why this");
    println!("   program never prints a HashSet directly.");

    println!();
    println!("7. A set from a Vec: collect drops the duplicates");
    let visits = vec!["Ada", "Ben", "Ada", "Cara", "Ben", "Ada"];
    let people: HashSet<&str> = visits.iter().copied().collect();
    println!("   {visits:?}");
    println!("   {} visits, {} people: {:?}", visits.len(), people.len(), sorted(&people));
    let mut first_seen: Vec<&str> = Vec::new();
    let mut known: HashSet<&str> = HashSet::new();
    for name in visits.iter().copied() {
        if known.insert(name) {
            first_seen.push(name);
        }
    }
    println!("   first-seen order, kept in a Vec beside the set: {first_seen:?}");

    println!();
    println!("8. One set operation, as a preview: intersection");
    let monday: HashSet<&str> = ["Ada", "Ben", "Cara"].into_iter().collect();
    let tuesday: HashSet<&str> = ["Ben", "Cara", "Dan"].into_iter().collect();
    let mut both: Vec<&str> = monday.intersection(&tuesday).copied().collect();
    both.sort();
    println!("   monday & tuesday = {both:?}");
}
