//! `()` is the empty tuple: one value, zero bytes. Everything here is a place
//! that value already turns up in code nobody thinks of as using it.

use std::collections::{HashMap, HashSet};
use std::mem::size_of;

fn no_return_type() {}
fn explicit_unit() -> () {}
fn returns_a_number() -> i32 { 7 }

fn main() {
    println!("=== one value, zero bytes ===");
    let nothing: () = ();
    println!("  ()                     = {:?}", nothing);
    println!("  size_of::<()>()        = {}   <- a zero-sized type: it occupies nothing", size_of::<()>());
    println!("  size_of::<[(); 1000]>() = {}   <- a thousand of them also occupy nothing", size_of::<[(); 1000]>());
    println!("  () == ()               = {}   <- one value, so equality is always true", () == ());
    println!("  it is the only type with exactly one value; bool has 2, u8 has 256, () has 1");

    println!("\n=== where it comes from #1: a function with no -> ===");
    println!("  fn no_return_type() {{}}    returns {:?}", no_return_type());
    println!("  fn explicit_unit() -> () {{}} returns {:?}   <- the same signature, spelled out", explicit_unit());
    println!("  fn returns_a_number() -> i32 returns {:?}", returns_a_number());

    println!("\n=== where it comes from #2: the semicolon ===");
    let with_semicolon = { 7; };
    let without = { 7 };
    println!("  {{ 7; }}  = {:?}   <- the ; discards the value and leaves ()", with_semicolon);
    println!("  {{ 7 }}   = {:?}    <- no ;, so the block IS the value", without);
    println!("  that is the whole mechanism behind `expected i32, found ()`");

    println!("\n=== where it turns up #3: a Result that carries no success value ===");
    fn record_vote(score: u8) -> Result<(), String> {
        if score <= 5 { Ok(()) } else { Err(format!("score {score} is out of range 0..=5")) }
    }
    println!("  record_vote(5) = {:?}", record_vote(5));
    println!("  record_vote(9) = {:?}", record_vote(9));
    println!("  Ok(()) says 'it worked, and there is nothing to hand back'");

    println!("\n=== where it turns up #4: a set is a map whose values are () ===");
    let mut set: HashSet<&str> = HashSet::new();
    set.insert("Ada");
    let mut map: HashMap<&str, ()> = HashMap::new();
    map.insert("Ada", ());
    println!("  size_of::<HashSet<&str>>() == size_of::<HashMap<&str, ()>>() : {}",
             size_of::<HashSet<&str>>() == size_of::<HashMap<&str, ()>>());
    println!("  ...because () costs nothing to store, so the map's value column is free");
    println!("  set.contains(\"Ada\")       = {}", set.contains("Ada"));
    println!("  map.contains_key(\"Ada\")   = {}", map.contains_key("Ada"));

    println!("\n=== the operations that hand you one back ===");
    let mut names = vec!["Cara", "Ada", "Ben"];
    let sorted: () = names.sort();
    println!("  names.sort()              -> {:?}   <- sorts in place, returns nothing", sorted);
    println!("  names                     =  {:?}", names);
    let pushed: () = names.push("Dev");
    println!("  names.push(\"Dev\")         -> {:?}", pushed);
    let printed: () = println!("  println! itself           -> the line you are reading");
    println!("  ...and its value          =  {:?}", printed);
    println!("  that is why `let x = v.sort();` compiles and then confuses you:");
    println!("  x is (), not the sorted vector");

    println!("\n=== () versus the two things it is confused with ===");
    println!("  ()          one value,  zero bytes   'nothing to say'");
    println!("  Option::None one variant of a type   'there might have been something'");
    println!("  !           NO values                'this never returns at all'");
    let never_ran: Option<()> = None;
    println!("  Option<()>  = {:?} or Some(()) -- a bool wearing two extra characters", never_ran);
    println!("  size_of::<Option<()>>() = {}   <- one byte, because None needs a tag", size_of::<Option<()>>());
}
