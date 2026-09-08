//! Kata solution: what the compiler wrote for you.
//!
//! A closure is a function that can also see the variables around where it was
//! written. The compiler makes that true by writing a struct: one field per
//! captured variable, plus an implementation of one of the closure traits. So
//! the SIZE of a closure is the size of what it captured, and no two closures
//! have the same type even when they have the same signature.
//!
//!   rustc --edition 2024 what_a_closure_is_kata.rs -o /tmp/wac && /tmp/wac

use std::mem::size_of_val;

fn main() {
    let factor = 3u64;
    let label = String::from("times");
    let table = [1u8; 32];

    let captures_nothing = |n: u64| n + 1;
    let captures_one = |n: u64| n * factor;
    let captures_two = |n: u64| format!("{n} {label} {factor}");
    let captures_array = move |n: usize| table[n % table.len()];

    println!("SIZE IS WHAT IT CAPTURED");
    println!("  captures nothing        {:>3} bytes", size_of_val(&captures_nothing));
    println!("  captures one u64        {:>3} bytes", size_of_val(&captures_one));
    println!("  captures a &String too  {:>3} bytes", size_of_val(&captures_two));
    println!("  captures a [u8; 32]     {:>3} bytes", size_of_val(&captures_array));
    println!();
    println!("  A closure that captures nothing is a zero-sized value: there is");
    println!("  nothing to store, so it costs no memory at all. Each capture");
    println!("  adds exactly its own field, by reference or by value depending");
    println!("  on what the body needs and whether `move` was written.");
    println!();

    println!("EVERY CLOSURE HAS ITS OWN TYPE");
    let a = |n: u64| n + 1;
    let b = |n: u64| n + 1;
    println!("  a(1) = {}, b(1) = {}, and both are {} bytes", a(1), b(1), size_of_val(&a));
    println!("  a and b have identical source text and identical signatures,");
    println!("  and they are still two different types. `let mut f = a; f = b;`");
    println!("  does not compile -- the error names two closures at two");
    println!("  different lines. That is why returning one needs `impl Fn` or a");
    println!("  `Box<dyn Fn>`: the type has no name you could write down.");
    println!();

    println!("SO WHAT DOES IT DO?");
    println!("  captures_one(7)   = {}", captures_one(7));
    println!("  captures_two(7)   = {}", captures_two(7));
    println!("  captures_array(7) = {}", captures_array(7));
    println!();
    println!("  factor is still usable here: {factor}");
    println!("  label is still usable here: {label}");
    println!("  Neither was moved, because neither body needed to own it -- the");
    println!("  compiler captured both by shared reference. `table` WAS moved,");
    println!("  because `move` was written, and naming it below this line would");
    println!("  not compile.");

    assert_eq!(size_of_val(&captures_nothing), 0);
    assert_eq!(size_of_val(&captures_one), 8);
    assert_eq!(captures_one(7), 21);
}
