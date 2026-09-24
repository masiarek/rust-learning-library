//! Kata solution: count the words of a sentence with entry, then look three of them up.
//!
//!   rustc --edition 2024 first_hashmap_kata.rs -o /tmp/fhmk && /tmp/fhmk

use std::collections::HashMap;

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog the end";

    println!("1. Count every word");
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in sentence.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    let mut rows: Vec<(&str, u32)> = counts.iter().map(|(word, count)| (*word, *count)).collect();
    rows.sort();
    println!("   {} words, {} distinct", sentence.split_whitespace().count(), counts.len());
    println!("   by word:  {rows:?}");
    rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("   by count: {rows:?}");

    println!();
    println!("2. Look three words up");
    for word in ["the", "fox", "cat"] {
        let present = counts.contains_key(word);
        let count = format!("{:?}", counts.get(word));
        let number = counts.get(word).copied().unwrap_or(0);
        println!("   {word:>3}: contains_key = {present:<5}  get = {count:<8}  count = {number}");
    }

    println!();
    println!("3. What the three lookups say");
    println!("   contains_key answers yes or no; get hands back the count in a Some, or a");
    println!("   None for a word that was never counted. The 0 for \"cat\" is not in the");
    println!("   map: unwrap_or(0) supplied it.");
}
