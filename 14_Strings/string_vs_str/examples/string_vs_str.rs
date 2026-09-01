//! One function, three callers: the literal, the owned String, the borrowed view.
//!
//!   rustc --edition 2024 string_vs_str.rs -o /tmp/svs && /tmp/svs

use std::mem::size_of;

/// Takes the view. Every kind of string can afford to call this.
fn shout(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    println!("1. Three ways to have text");
    let literal = "The Big Lebowski"; // &'static str — the bytes are in the binary
    let owned = String::from(literal); // a fresh heap buffer, yours to grow
    let view = &owned[4..7]; // &str — a window into `owned`, nothing copied
    println!("   literal  &'static str  {literal:?}");
    println!("   owned    String        {owned:?}");
    println!("   view     &str          {view:?}   <- bytes 4..7 of `owned`, borrowed");

    println!("\n2. What each one costs on the stack");
    println!("   size_of::<&str>()    = {} bytes  (pointer + length)", size_of::<&str>()); // 16
    println!("   size_of::<String>()  = {} bytes  (pointer + length + capacity)", size_of::<String>()); // 24
    println!("   size_of::<&String>() = {} bytes  (just a pointer)", size_of::<&String>()); // 8

    println!("\n3. One &str parameter serves every caller");
    println!("   shout(literal) = {:?}", shout(literal));
    println!("   shout(&owned)  = {:?}   <- &String coerced to &str, free", shout(&owned));
    println!("   shout(view)    = {:?}", shout(view));

    println!("\n4. The owner can grow; the view never can");
    let mut grows = String::from("count");
    grows.push_str("ed");
    println!("   grows = {grows:?}   <- push_str needs a `mut String`");
    println!("   a &str has no push_str — the text may not even be yours to change");

    println!("\n5. String moves; &str copies");
    let s1 = "hello"; // &str is Copy: duplicating a view costs 16 bytes, owes nothing
    let s2 = s1;
    println!("   &str:   s1 = {s1:?}, s2 = {s2:?}   <- both alive");
    let big1 = String::from("hello");
    let big2 = big1; // the buffer changed owners
    println!("   String: big2 = {big2:?}, and `big1` is now unusable — E0382");

    println!("\n6. `str` is fixed-length, NOT immutable");
    let mut name = String::from("per martin-lof");
    name.as_mut_str().make_ascii_uppercase(); // a &mut str, mutated in place
    println!("   through a &mut str:  {name:?}   <- no allocation, same buffer");
    let mut half = String::from("per martin-lof");
    {
        let (first, last) = half.split_at_mut(3); // two &mut str into one buffer
        first.make_ascii_uppercase();
        println!("   split_at_mut halves: {first:?} + {last:?}");
    }
    println!("   only the first half changed: {half:?}");
    println!("   what a str cannot do is change LENGTH — one byte out, one byte in.");
    println!("   `&str` is read-only because the `&` is, not because `str` is.");
}
