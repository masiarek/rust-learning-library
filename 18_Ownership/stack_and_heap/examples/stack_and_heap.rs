//! Where a value's data lives — and why the TYPE decides, not a keyword.
//!
//! Nothing here prints an address. Every line is a size or a length, because a
//! raw address differs on every run and could never be an answer key. Sizes are
//! for a 64-bit machine.
//!
//!   rustc --edition 2024 stack_and_heap.rs -o /tmp/sah && /tmp/sah

use std::rc::Rc;
use std::sync::Arc;

struct Point {
    x: f32,
    y: f32,
}

fn main() {
    println!("1. Fixed size, known at compile time -> the stack slot IS the value");
    println!("   i32          {:>5}", std::mem::size_of::<i32>());
    println!("   bool         {:>5}", std::mem::size_of::<bool>());
    println!("   char         {:>5}   a Unicode scalar, not a byte", std::mem::size_of::<char>());
    println!("   (f64, f64)   {:>5}   a tuple is its fields", std::mem::size_of::<(f64, f64)>());
    println!("   [i32; 5]     {:>5}   an array is length x element", std::mem::size_of::<[i32; 5]>());
    println!("   Point        {:>5}   a struct is its fields, and nothing else", std::mem::size_of::<Point>());
    let p = Point { x: 1.0, y: 2.0 };
    println!("   Making one costs a bump of the stack pointer: p = ({}, {})", p.x, p.y);

    println!("\n2. Growable or unknown size -> a fixed HEADER on the stack, data on the heap.");
    println!("   The header is all `size_of` can see, so it does not move with the content:");
    let short = String::from("Hello");
    let long = "x".repeat(5000);
    println!("   String holding {:>4} bytes of text -> {} bytes on the stack",
             short.len(), std::mem::size_of_val(&short));
    println!("   String holding {:>4} bytes of text -> {} bytes on the stack",
             long.len(), std::mem::size_of_val(&long));
    let names = vec![String::from("Ada"), String::from("Ben"), String::from("Cara")];
    println!("   Vec of {} Strings                  -> {} bytes on the stack",
             names.len(), std::mem::size_of_val(&names));
    println!("   ptr + len + capacity, three words, whatever is at the other end.");

    println!("   `size_of_val` reports what a value OCCUPIES, never what it ALLOCATED:");
    let mut roomy = String::with_capacity(64);
    roomy.push_str("abc");
    println!("     size_of_val(&roomy)           = {:>3}   the header again", std::mem::size_of_val(&roomy));
    println!("     size_of_val(roomy.as_str())   = {:>3}   the live text, on the heap", std::mem::size_of_val(roomy.as_str()));
    println!("     roomy.capacity()              = {:>3}   what was actually bought", roomy.capacity());
    println!("   Hand it the str and it does see heap bytes -- but it answers len, not capacity.");

    println!("\n3. Every pointer type costs the same on the stack. What differs is the far end:");
    println!("   Box<String>  {:>5}   one owner, freed when it drops", std::mem::size_of::<Box<String>>());
    println!("   Rc<String>   {:>5}   several owners, counted, one thread", std::mem::size_of::<Rc<String>>());
    println!("   Arc<String>  {:>5}   several owners, counted atomically, many threads", std::mem::size_of::<Arc<String>>());
    println!("   &String      {:>5}   borrowed, owns nothing", std::mem::size_of::<&String>());
    println!("   &str         {:>5}   ptr + len: a view needs no capacity", std::mem::size_of::<&str>());

    println!("\n4. Box is the one place you SAY 'put this on the heap'");
    let boxed: Box<[u8; 4096]> = Box::new([0; 4096]);
    println!("   [u8; 4096]        {:>5} bytes on the stack", std::mem::size_of::<[u8; 4096]>());
    println!("   Box<[u8; 4096]>   {:>5} bytes on the stack, {} on the heap",
             std::mem::size_of_val(&boxed), std::mem::size_of_val(&*boxed));
    println!("   The far number is COMPUTED, not claimed: `&boxed` is the pointer and");
    println!("   `&*boxed` is what it points at, so deref first and size_of_val crosses over.");
    println!("   The stack is a few MB per thread and cannot grow. That is the whole reason");
    println!("   Box exists: the array is the same size either way, but only one of the two");
    println!("   spends the bounded region on it.");
}
