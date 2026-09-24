//! A first HashMap: the problem two Vecs make, then the map, one printed step at a time.
//!
//!   rustc --edition 2024 first_hashmap.rs -o /tmp/fhm && /tmp/fhm

use std::any::type_name_of_val;
use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, HashMap};

/// A HashMap has no order, so this is how every map on this page is printed:
/// copied into a Vec of pairs and sorted by key.
fn sorted<'a>(map: &'a HashMap<&'a str, u32>) -> Vec<(&'a str, u32)> {
    let mut rows: Vec<(&str, u32)> = map.iter().map(|(k, v)| (*k, *v)).collect();
    rows.sort();
    rows
}

fn main() {
    println!("0. The problem: four words, four counts, two Vecs");
    let words = vec!["Hello", "world", "Rust", "Programming"];
    let counts = vec![5, 2, 15, 5];
    let i = words.iter().position(|w| *w == "Rust").unwrap();
    println!("   words[{i}] = {:?}, so counts[{i}] = {}", words[i], counts[i]);
    println!("   The two Vecs are tied only by position: reorder one and every count is wrong.");

    println!();
    println!("   One Vec of pairs, searched with the book's loop:");
    let word_counts = vec![("Hello", 5), ("world", 2), ("Rust", 15), ("Programming", 5)];
    let target_word = "hello";
    let mut found = false;
    for (word, _) in &word_counts {
        if word.to_lowercase() == target_word.to_lowercase() {
            found = true;
            break;
        }
    }
    println!("   looking for {target_word:?} with to_lowercase on both sides: found = {found}");
    let position = word_counts.iter().position(|(w, _)| *w == "Rust");
    println!("   position of \"Rust\" = {position:?}, count = {}", word_counts[position.unwrap()].1);
    println!("   Nothing stops a second (\"Rust\", 3) pair from being pushed after it.");

    println!();
    println!("1. A HashMap: HashMap::new(), then insert");
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    let first = word_counts.insert("Hello", 5);
    println!("   insert(\"Hello\", 5) returned {first:?}   (nothing was there before)");
    word_counts.insert("world", 2);
    word_counts.insert("Rust", 15);
    word_counts.insert("Programming", 5);
    println!("   len = {}   sorted: {:?}", word_counts.len(), sorted(&word_counts));

    println!();
    println!("2. Keys are unique: a second insert of the same key replaces the value");
    let old = word_counts.insert("Programming", 15);
    println!("   insert(\"Programming\", 15) returned {old:?}   (the value that was replaced)");
    println!("   len is still {}   sorted: {:?}", word_counts.len(), sorted(&word_counts));

    println!();
    println!("3. get asks: Option<&u32>");
    let rust = word_counts.get("Rust");
    let zig = word_counts.get("Zig");
    println!("   get(\"Rust\") = {rust:?}   get(\"Zig\") = {zig:?}");
    println!("   the type of what get returns: {}", type_name_of_val(&rust));
    match word_counts.get("Rust") {
        Some(count) => println!("   Rust appears {count} times"),
        None => println!("   Rust is not in the map"),
    }
    let count = word_counts.get("Zig").copied().unwrap_or(0);
    println!("   get(\"Zig\").copied().unwrap_or(0) = {count}");

    println!();
    println!("4. contains_key answers yes or no: bool");
    let has = word_counts.contains_key("Programming");
    println!("   contains_key(\"Programming\") = {has}   type: {}", type_name_of_val(&has));
    println!("   contains_key(\"programming\") = {}   (keys are case-sensitive)", word_counts.contains_key("programming"));

    println!();
    println!("5. Indexing asserts: map[key] is the value, or a panic");
    println!("   word_counts[\"Rust\"] = {}", word_counts["Rust"]);
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let missing = std::panic::catch_unwind(|| word_counts[std::hint::black_box("Zig")]);
    std::panic::set_hook(hook);
    println!("   word_counts[\"Zig\"]  -> {}", if missing.is_err() { "panicked" } else { "returned" });

    println!();
    println!("6. entry: the slot for a key, whether or not it is filled");
    println!("   entry(\"Hello\") = {:?}", word_counts.entry("Hello"));
    println!("   entry(\"Zig\")   = {:?}", word_counts.entry("Zig"));
    match word_counts.entry("Hello") {
        Entry::Occupied(slot) => println!("   \"Hello\" is Occupied, holding {}", slot.get()),
        Entry::Vacant(slot) => println!("   {:?} is Vacant", slot.key()),
    }
    let present = word_counts.entry("Hello").or_insert(0);
    println!("   entry(\"Hello\").or_insert(0) -> &mut holding {present}   (already there: the 0 was not used)");
    let absent = word_counts.entry("Zig").or_insert(0);
    println!("   entry(\"Zig\").or_insert(0)   -> &mut holding {absent}   (was absent: the 0 went in)");
    *absent += 1;
    println!("   after *absent += 1: get(\"Zig\") = {:?}   len = {}", word_counts.get("Zig"), word_counts.len());

    println!();
    println!("   The counting loop, on a sentence:");
    let text = "the cat and the dog and the bird";
    let mut tally: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *tally.entry(word).or_insert(0) += 1;
    }
    println!("   {text:?}");
    println!("   -> {:?}", sorted(&tally));
    println!("   \"the\" is met three times. The first time the slot is Vacant, so");
    println!("   or_insert(0) puts a 0 there and hands back a &mut to it; += 1 makes it 1.");
    println!("   The next two times the slot is Occupied, the 0 is ignored, and += 1");
    println!("   makes it 2, then 3.");

    println!();
    println!("7. remove: Option of what was there");
    let gone = tally.remove("cat");
    let again = tally.remove("cat");
    println!("   remove(\"cat\") = {gone:?}   remove(\"cat\") again = {again:?}   len = {}", tally.len());

    println!();
    println!("8. Iterating: every pair, in no particular order");
    let mut pairs: Vec<(&str, u32)> = Vec::new();
    for (word, count) in &tally {
        pairs.push((*word, *count));
    }
    pairs.sort();
    println!("   sorted after the loop: {pairs:?}");
    let ordered: BTreeMap<&str, u32> = tally.iter().map(|(k, v)| (*k, *v)).collect();
    println!("   the same in a BTreeMap: {ordered:?}");
    println!("   The loop's own order is not the insertion order and changes from run");
    println!("   to run, which is why this program never prints a HashMap directly.");

    println!();
    println!("9. Case matters: \"Hello\" and \"hello\" are two keys");
    let mut greetings: HashMap<&str, u32> = HashMap::new();
    greetings.insert("Hello", 5);
    println!("   get(\"Hello\") = {:?}   get(\"hello\") = {:?}", greetings.get("Hello"), greetings.get("hello"));
    let mut folded: HashMap<String, u32> = HashMap::new();
    for word in ["Hello", "hello", "HELLO", "world"] {
        *folded.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    let mut rows: Vec<(&String, &u32)> = folded.iter().collect();
    rows.sort();
    println!("   lowercased at insert time: {rows:?}");

    println!();
    println!("10. &str keys borrow the text; String keys own a copy of it");
    let borrowed: HashMap<&str, u32> = tally;
    println!("   HashMap<&str, u32>: {} keys pointing into text that must outlive the map", borrowed.len());
    let line = String::from("the cat and the dog and the bird");
    let mut owned: HashMap<String, u32> = HashMap::new();
    for word in line.split_whitespace() {
        *owned.entry(word.to_string()).or_insert(0) += 1;
    }
    drop(line);
    println!("   HashMap<String, u32> after the line is dropped: get(\"the\") = {:?}", owned.get("the"));
    println!("   A String key is looked up with a plain &str, so get(\"the\") reads the same on both.");
}
