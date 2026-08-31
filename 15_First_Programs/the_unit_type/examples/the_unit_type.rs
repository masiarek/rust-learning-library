//! `()` is the empty tuple: one value, zero bytes. Everything here is a place
//! that value already turns up in code nobody thinks of as using it.

use std::collections::{HashMap, HashSet};
use std::mem::{align_of, size_of};
use std::sync::mpsc;
use std::thread;

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

    println!("\n=== why zero: memory exists to tell states apart ===");
    println!("  {:<8} {:>7}  {:>5}  {:>6}", "type", "values", "bits", "bytes");
    println!("  {:<8} {:>7}  {:>5}  {:>6}", "u8", 256, (256f64).log2() as u32, size_of::<u8>());
    println!("  {:<8} {:>7}  {:>5}  {:>6}", "bool", 2, (2f64).log2() as u32, size_of::<bool>());
    println!("  {:<8} {:>7}  {:>5}  {:>6}", "()", 1, (1f64).log2() as u32, size_of::<()>());
    println!("  bits = log2(values). One value needs log2(1) = 0 bits, so there is nothing");
    println!("  to store: if a variable has type (), its value must be (). bool is the row");
    println!("  where the two columns part -- 1 bit of information, 1 whole byte of space,");
    println!("  because a byte is the smallest thing a machine can address.");

    println!("\n=== the equality is decided at compile time, not run time ===");
    println!("  () == ()   = {}   <- one value, so it cannot be otherwise", () == ());
    println!("  compiled with -O, `fn unit_eq(a: (), b: ()) -> bool {{ a == b }}` is:");
    println!("      movb  $1, %al        <- load the constant 1, and return");
    println!("  neither argument is read. The bool version really compares:");
    println!("      movl  %edi, %eax ; xorl %esi, %eax ; xorb $1, %al");

    println!("\n=== zero bytes is not 'no address' ===");
    println!("  align_of::<()>()      = {}   <- still aligned, still a real place", align_of::<()>());
    let here: &() = &();
    println!("  &() is a real reference at a nonzero address: {}", (here as *const ()) as usize != 0);
    let mut many: Vec<()> = Vec::new();
    for _ in 0..1_000_000 {
        many.push(());
    }
    println!("  a Vec<()> after 1,000,000 pushes: len {}", many.len());
    println!("  ...and it never allocated: capacity == usize::MAX is {}", many.capacity() == usize::MAX);
    println!("  there is no data to store, so the Vec is just a counter with a spare field");

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

    println!("\n=== where it turns up #5: a channel that carries only the fact ===");
    let (tx, rx) = mpsc::channel::<()>();
    let worker = thread::spawn(move || {
        tx.send(()).expect("receiver still alive");
    });
    rx.recv().expect("sender still alive");
    worker.join().expect("worker did not panic");
    println!("  mpsc::channel::<()>() -- the message IS the signal, with no payload");
    println!("  received one; size of what crossed the channel = {} bytes", size_of::<()>());

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
