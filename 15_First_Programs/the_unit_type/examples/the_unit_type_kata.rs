//! Kata solution: three places `()` shows up, and what each one is telling you.

use std::collections::HashMap;

fn main() {
    println!("=== part 1: the in-place methods hand back nothing ===");
    let mut scores = vec![3u8, 5, 1, 5, 0];
    println!("  before                 = {:?}", scores);

    let returned: () = scores.sort();
    println!("  let x = scores.sort()  -> x is {:?}, NOT the sorted vector", returned);
    println!("  scores (mutated)       = {:?}", scores);
    println!("  asking x for its length is where you find out:");
    println!("    error[E0599]: no method named `len` found for unit type `()` in the current scope");
    println!("  the message names the type: `()` is what an in-place method returns,");
    println!("  because the answer was written back into the receiver.");

    println!("\n  the two ways to get a sorted value out of it:");
    let mut in_place = vec![3u8, 5, 1];
    in_place.sort();
    println!("    mutate, then use     = {:?}", in_place);
    let original = vec![3u8, 5, 1];
    let mut copy = original.clone();
    copy.sort();
    println!("    clone, sort the copy = {:?}  (original still {:?})", copy, original);

    println!("\n  every in-place method on Vec does this:");
    let mut v = vec![3u8, 1, 1, 5];
    let a: () = v.push(9);
    let b: () = v.dedup();
    let c: () = v.retain(|&s| s <= 5);
    println!("    push / dedup / retain all return {:?} {:?} {:?} -> v = {:?}", a, b, c, v);
    println!("    so none of them chains: v.push(9).dedup() does not compile");

    println!("\n=== part 2: Ok(()) is 'it worked, and there is nothing to hand back' ===");
    fn check_score(score: u8) -> Result<(), String> {
        if score <= 5 { Ok(()) } else { Err(format!("score {score} is out of range 0..=5")) }
    }
    fn check_ballot(ballot: &[u8]) -> Result<(), String> {
        for &s in ballot {
            check_score(s)?;
        }
        Ok(())
    }
    for ballot in [&[5u8, 3, 0][..], &[5u8, 9, 0][..]] {
        println!("  check_ballot({:?}) = {:?}", ballot, check_ballot(ballot));
    }
    println!("  `check_score(s)?` discards nothing on success -- there was nothing to discard.");
    println!("  Result<(), E> is the return type of a job that either works or explains itself.");

    println!("\n=== part 3: a set is a map whose values are () ===");
    let mut seen: HashMap<&str, ()> = HashMap::new();
    let mut order: Vec<&str> = Vec::new();
    for name in ["Ada", "Ben", "Ada", "Cara", "Ben"] {
        if seen.insert(name, ()).is_none() {
            order.push(name);
        }
    }
    println!("  first appearance order = {:?}", order);
    println!("  seen.contains_key(\"Ada\")  = {}", seen.contains_key("Ada"));
    println!("  seen.len()                = {}", seen.len());
    println!("  `insert` returns Option<()> -- Some(()) means it was already there.");
    println!("  That Option<()> is a bool with extra syntax, which is exactly why");
    println!("  HashSet::insert returns a real bool instead. Same structure, better name.");
}
