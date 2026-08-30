//! Every `match` whose arms only rewrap the value has a method that says it.
//!
//!   rustc --edition 2024 transforms_instead_of_match.rs -o /tmp/tim && /tmp/tim

use std::cell::Cell;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Voter {
    name: String,
    ballot: Option<String>,
}

// Counts how many times an "expensive" fallback was actually built, which is
// how the eager/lazy pairs below are proved rather than asserted.
thread_local! {
    static BUILDS: Cell<u32> = const { Cell::new(0) };
}

fn expensive_default() -> String {
    BUILDS.with(|b| b.set(b.get() + 1));
    "built the expensive thing".to_string()
}

fn parse(s: &str) -> Result<u32, ParseIntError> {
    s.parse::<u32>()
}

impl Voter {
    /// The trap Item 3 ends on: `&self` cannot give away what it only borrows.
    /// `as_ref()` turns `&Option<String>` into `Option<&String>` so the borrow
    /// is what travels, not the value.
    fn shout(&self) -> String {
        // self.ballot.unwrap_or(String::new())    // E0507 — see the page
        self.ballot.as_ref().map_or("(no ballot)".to_string(), |b| b.to_uppercase())
    }
}

fn main() {
    let cast: Option<u32> = Some(3);
    let blank: Option<u32> = None;

    println!("1. The shape: a `match` whose arms put the value back");
    let doubled_by_match = match cast {
        Some(n) => Some(n * 2),
        None => None,
    };
    println!("   match {{ Some(n) => Some(n * 2), None => None }} = {doubled_by_match:?}");
    println!("   cast.map(|n| n * 2)                            = {:?}", cast.map(|n| n * 2));
    println!("   same answer: {}", doubled_by_match == cast.map(|n| n * 2));
    println!("   The method is not shorter by accident — `map` NAMES the shape:");
    println!("   change the payload, leave the Some/None decision alone.");

    println!("\n2. `map` or `and_then`? Ask what your closure returns");
    let half = |n: u32| if n % 2 == 0 { Some(n / 2) } else { None };
    println!("   the closure returns u32          -> map      -> {:?}", cast.map(|n| n * 2));
    println!("   the closure returns Option<u32>  -> map      -> {:?}   nested!", cast.map(half));
    println!("   the closure returns Option<u32>  -> and_then -> {:?}   flat", cast.and_then(half));
    println!("   Some(4).and_then(half) = {:?}   Some(3).and_then(half) = {:?}", Some(4u32).and_then(half), Some(3u32).and_then(half));
    println!("   `and_then` is the one that may say \"and now it is missing\".");

    println!("\n3. Crossing between the two types");
    println!("   Option -> Result   cast.ok_or(\"no ballot\")   = {:?}", cast.ok_or("no ballot"));
    println!("                      blank.ok_or(\"no ballot\")  = {:?}", blank.ok_or("no ballot"));
    println!("   Result -> Option   parse(\"12\").ok()          = {:?}", parse("12").ok());
    println!("                      parse(\"x\").ok()           = {:?}   the WHY is gone", parse("x").ok());
    println!("   `.ok()` is a downgrade: four different parse failures all arrive");
    println!("   as one indistinguishable None. Cross that way on purpose only.");

    println!("\n4. `map_err` — the same value, a different error type");
    let as_string: Result<u32, String> = parse("x").map_err(|e| format!("bad count: {e}"));
    println!("   parse(\"x\").map_err(|e| format!(\"bad count: {{e}}\")) = {as_string:?}");
    println!("   parse(\"12\").map_err(..)                            = {:?}   untouched", parse("12").map_err(|e| format!("bad count: {e}")));

    println!("\n5. Eager or lazy: the `_else` suffix is not decoration");
    BUILDS.with(|b| b.set(0));
    let _ = cast.map(|n| n).unwrap_or(expensive_default().len() as u32);
    println!("   Some(3).unwrap_or(expensive_default())      builds = {}", BUILDS.with(|b| b.get()));
    BUILDS.with(|b| b.set(0));
    let _ = cast.map(|n| n).unwrap_or_else(|| expensive_default().len() as u32);
    println!("   Some(3).unwrap_or_else(|| expensive())      builds = {}", BUILDS.with(|b| b.get()));
    println!("   The value was never used either time. The eager form built it anyway,");
    println!("   because an argument is evaluated before the call it is an argument to.");
    println!("   Same split: ok_or / ok_or_else, or / or_else, map_or / map_or_else.");

    println!("\n6. The rest of the vocabulary, on one line each");
    println!("   filter   Some(3).filter(|n| n % 2 == 0)  = {:?}   keep it only if", Some(3u32).filter(|n| n % 2 == 0));
    println!("   or       blank.or(Some(9))               = {:?}   first one present", blank.or(Some(9)));
    println!("   xor      Some(3).xor(Some(9))            = {:?}   exactly one, or None", Some(3u32).xor(Some(9)));
    println!("   zip      Some(3).zip(Some(9))            = {:?}   both, or None", Some(3u32).zip(Some(9)));
    let mut slot = Some(3u32);
    let old = slot.replace(9);
    println!("   replace  slot.replace(9) -> {old:?}, slot is now {slot:?}");
    let taken = slot.take();
    println!("   take     slot.take()     -> {taken:?}, slot is now {slot:?}");

    println!("\n7. `as_ref()` — when `&self` owns nothing it may give away");
    let voted = Voter { name: "Ada".into(), ballot: Some("yes".into()) };
    let silent = Voter { name: "Ben".into(), ballot: None };
    println!("   voted.shout()  = {}", voted.shout());
    println!("   silent.shout() = {}", silent.shout());
    println!("   Without as_ref(): E0507, cannot move out of `self.ballot` which is");
    println!("   behind a shared reference. as_ref() rewrites &Option<String> into");
    println!("   Option<&String>, so the Option is rebuilt around a borrow.");
    println!("   voted.name is still ours afterwards: {}", voted.name);

    println!("\n8. Where a `match` is still the right answer");
    let outcome: Result<u32, ParseIntError> = parse("7");
    let sentence = match outcome {
        Ok(n) if n > 5 => format!("{n} is a landslide"),
        Ok(n) => format!("{n} is close"),
        Err(e) => format!("not a number: {e}"),
    };
    println!("   {sentence}");
    println!("   Three outcomes, a guard, and different WORK per arm — no single");
    println!("   transform expresses that. The advice is to stop writing the matches");
    println!("   that only rewrap, not to stop writing `match`.");
}
