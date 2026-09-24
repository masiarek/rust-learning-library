//! Every fix from the HashMap and HashSet errors page, in one program that compiles and runs.
//! The numbers match the entries on the page.
//!
//!   rustc --edition 2024 hashmap_err_fixes.rs -o /tmp/hef && /tmp/hef

use std::collections::{HashMap, HashSet};

// 17. A struct key needs Eq and Hash; deriving both keeps them in step.
#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

// 12. entry needs a &mut, so the function asks for one.
fn bump(word_counts: &mut HashMap<&str, u32>) {
    *word_counts.entry("Hello").or_insert(0) += 1;
}

fn main() {
    println!("1. use std::collections::HashMap; sits at the top of this file");

    println!("2. The type comes from an annotation, or from the first insert");
    let annotated: HashMap<&str, u32> = HashMap::new();
    let mut inferred = HashMap::new();
    inferred.insert("Hello", 5);
    println!("   annotated.len() = {}, inferred.len() = {}", annotated.len(), inferred.len());

    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    word_counts.insert("world", 2);

    println!("3. get is an Option: keep it as one, or open it with a default");
    let found: Option<&u32> = word_counts.get("Hello");
    let count: u32 = word_counts.get("Hello").copied().unwrap_or(0);
    println!("   found = {found:?}, count = {count}");

    println!("4. Compare the Option with an Option, or open it first");
    println!("   get(\"Hello\") == Some(&5): {}", word_counts.get("Hello") == Some(&5));
    println!("   get(\"Hello\").copied() == Some(5): {}", word_counts.get("Hello").copied() == Some(5));

    println!("5. Arithmetic on the number inside, not on the Option");
    let more = word_counts.get("Hello").copied().unwrap_or(0) + 1;
    println!("   more = {more}");

    println!("6. In a loop over &map the value is a &u32: dereference it, or destructure the &");
    let mut fives = 0;
    for (_word, count) in &word_counts {
        if *count == 5 {
            fives += 1;
        }
    }
    for (_word, &count) in &word_counts {
        if count == 5 {
            fives += 1;
        }
    }
    println!("   keys holding 5, counted both ways: {fives}");

    println!("7. Indexing names a place: borrow it or clone it, never move it");
    let mut names: HashMap<u32, String> = HashMap::new();
    names.insert(1, String::from("Hello"));
    let first: &String = &names[&1];
    let owned: String = names[&1].clone();
    println!("   first = {first}, owned = {owned}");

    println!("8. A key that might be missing is asked for with get, not []");
    println!("   get(\"hello\") = {:?}", word_counts.get("hello"));

    println!("9. &str keys, a String in hand: look up with as_str()");
    let wanted = String::from("Hello");
    println!("   get(wanted.as_str()) = {:?}", word_counts.get(wanted.as_str()));

    println!("10. String keys, a String in hand: pass a reference");
    let mut owned_keys: HashMap<String, u32> = HashMap::new();
    owned_keys.insert(String::from("Hello"), 5);
    println!("   get(&wanted) = {:?}, get(\"Hello\") = {:?}", owned_keys.get(&wanted), owned_keys.get("Hello"));

    println!("11. Change a value through get_mut");
    if let Some(count) = word_counts.get_mut("Hello") {
        *count += 1;
    }
    println!("   Hello -> {:?}", word_counts.get("Hello"));

    println!("12. entry through a &mut parameter");
    bump(&mut word_counts);
    println!("   Hello -> {:?}", word_counts.get("Hello"));

    println!("13. insert needs let mut");
    let mut fresh: HashMap<&str, u32> = HashMap::new();
    fresh.insert("Hello", 5);
    println!("   fresh.len() = {}", fresh.len());

    println!("14. The * in front: += works on the u32, not on the &mut u32");
    let mut tally: HashMap<&str, u32> = HashMap::new();
    for word in ["the", "cat", "the"] {
        *tally.entry(word).or_insert(0) += 1;
    }
    println!("   the -> {:?}", tally.get("the"));

    println!("15. Loop over &map to keep the map");
    let mut total = 0;
    for (_word, count) in &word_counts {
        total += count;
    }
    println!("   {total} over {} words", word_counts.len());

    println!("16. Decide during the loop, insert after it");
    let mut frequent: Vec<u32> = Vec::new();
    for (_word, count) in &word_counts {
        if *count > 3 {
            frequent.push(*count);
        }
    }
    for count in frequent {
        word_counts.insert("frequent", count);
    }
    println!("   frequent -> {:?}", word_counts.get("frequent"));

    println!("17. A struct key with the traits derived");
    let mut labels: HashMap<Point, &str> = HashMap::new();
    labels.insert(Point { x: 0, y: 0 }, "origin");
    println!("   {:?} -> {:?}", Point { x: 0, y: 0 }, labels.get(&Point { x: 0, y: 0 }));

    println!("18. No f64 keys: keep the price in cents, as an integer");
    let mut prices: HashMap<u32, &str> = HashMap::new();
    prices.insert(999, "book");
    println!("   999 cents -> {:?}", prices.get(&999));

    println!("19. Keys that must outlive the text are Strings");
    let mut lasting: HashMap<String, u32> = HashMap::new();
    {
        let text = String::from("the cat and the dog");
        for word in text.split_whitespace() {
            *lasting.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    println!("   the -> {:?}", lasting.get("the"));

    println!("20. HashSet::insert's bool is yours to use or to drop; nothing here is #[must_use]");
    let mut words: HashSet<&str> = HashSet::new();
    words.insert("Hello");
    if !words.insert("Hello") {
        println!("   \"Hello\" was already there");
    }
}
