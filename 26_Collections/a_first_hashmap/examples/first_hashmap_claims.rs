//! What §5.5 and §5.6 of Rust: The Practical Guide say about HashMap and HashSet, run.
//!
//!   rustc --edition 2024 first_hashmap_claims.rs -o /tmp/fhc && /tmp/fhc

use std::any::type_name_of_val;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("§5.5 HashMaps, with the book's map: HashMap<&str, u8>");
    let mut word_counts: HashMap<&str, u8> = HashMap::new();
    word_counts.insert("Hello", 5);
    word_counts.insert("world", 2);
    word_counts.insert("Rust", 15);
    word_counts.insert("Programming", 5);

    println!();
    println!("1. \"keys are unique\": insert(\"Programming\", 15) on a key already there");
    let old = word_counts.insert("Programming", 15);
    let mut rows: Vec<(&str, u8)> = word_counts.iter().map(|(k, v)| (*k, *v)).collect();
    rows.sort();
    println!("   returned {old:?}   len {}   sorted {rows:?}", word_counts.len());
    println!("   HOLDS: the value is 15 and \"Programming\" is there once. The book prints");
    println!("   the map itself, whose order is not fixed, so this program sorts first.");

    println!();
    println!("2. \"contains_key returns an Option wrapping the value in Some\"");
    let target_word = word_counts.contains_key("Programming");
    println!("   contains_key(\"Programming\") = {target_word}   type: {}", type_name_of_val(&target_word));
    println!("   FAILS: it is a bool. The method that returns the value in an Option is get:");
    let looked_up = word_counts.get("Programming");
    println!("   get(\"Programming\") = {looked_up:?}   type: {}", type_name_of_val(&looked_up));

    println!();
    println!("3. \"entry returns an instance of the Entry enum, with two variants, Occupied and Vacant\"");
    println!("   entry(\"Hello\") = {:?}", word_counts.entry("Hello"));
    println!("   entry(\"Zig\")   = {:?}", word_counts.entry("Zig"));
    println!("   HOLDS.");

    println!();
    println!("4. \"or_insert inserts a value if the key does not exist; Hello exists, so nothing is inserted\"");
    let before = word_counts.len();
    let value = *word_counts.entry("Hello").or_insert(0);
    println!("   entry(\"Hello\").or_insert(0): the &mut holds {value}   len {before} -> {}", word_counts.len());
    println!("   HOLDS, and the book stops there: or_insert also returns a &mut to the value,");
    println!("   which is what the counting loop needs.");

    println!();
    println!("5. The book's earlier loop compared lowercased words; the map does not");
    println!("   get(\"programming\") = {:?}   get(\"Programming\") = {:?}", word_counts.get("programming"), word_counts.get("Programming"));
    println!("   Not a claim the book makes, but a change it does not mention: keys are");
    println!("   case-sensitive, so \"programming\" is a different key from \"Programming\".");

    println!();
    println!("6. The book's value type is u8");
    println!("   250u8.checked_add(10) = {:?}: a u8 count cannot pass 255, and += 1 past it", 250u8.checked_add(10));
    println!("   panics in a debug build. A count is usually a u32 or a usize.");

    println!();
    println!("§5.6 HashSets, with the book's set: HashSet<&str>");
    let mut words: HashSet<&str> = HashSet::new();
    for word in ["Hello", "world", "Rust", "Programming"] {
        words.insert(word);
    }

    println!();
    println!("7. \"insert a duplicate entry: Hello appears only once\"");
    let duplicate = words.insert("Hello");
    let mut items: Vec<&str> = words.iter().copied().collect();
    items.sort();
    println!("   insert(\"Hello\") = {duplicate}   len {}   sorted {items:?}", words.len());
    println!("   HOLDS, and the book does not say that insert returns a bool: false here,");
    println!("   true for the four that went in.");

    println!();
    println!("8. \"contains returns a bool\"");
    let target_word = words.contains("world");
    println!("   contains(\"world\") = {target_word}   type: {}", type_name_of_val(&target_word));
    println!("   HOLDS.");

    println!();
    println!("9. \"take removes the item and returns it if it exists\"");
    if let Some(word) = words.take("world") {
        println!("   Removed: {word}");
    }
    println!("   take(\"world\") again = {:?}   len {}", words.take("world"), words.len());
    println!("   HOLDS: Some(\"world\") the first time, None once it is gone.");
}
