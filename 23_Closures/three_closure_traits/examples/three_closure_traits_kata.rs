//! Kata solution: the ladder, and what puts a closure on each rung.
//!
//! Fn, FnMut and FnOnce are not a menu. Every closure implements FnOnce; one
//! that does not consume its capture also implements FnMut; one that does not
//! mutate it either also implements Fn. What the BODY does decides how far up
//! it goes -- not how you declared it, and not what it captured.
//!
//!   rustc --edition 2024 three_closure_traits_kata.rs -o /tmp/tct && /tmp/tct

fn call_fn<F: Fn() -> String>(f: F) -> String { format!("{} / {}", f(), f()) }
fn call_fn_mut<F: FnMut() -> u32>(mut f: F) -> (u32, u32) { (f(), f()) }
fn call_fn_once<F: FnOnce() -> String>(f: F) -> String { f() }

fn main() {
    println!("RUNG 1 -- Fn: the body only READS the capture");
    let greeting = String::from("hello");
    let reads = || greeting.clone();
    println!("  call_fn(reads)     {}", call_fn(reads));
    println!("  Callable many times, and callable through a shared reference.");
    println!("  `greeting` is still ours: {greeting}");
    println!();

    println!("RUNG 2 -- FnMut: the body MUTATES the capture");
    let mut count = 0u32;
    let mut counts = || { count += 1; count };
    println!("  call_fn_mut(counts) {:?}", call_fn_mut(&mut counts));
    println!("  Callable many times, but the caller needs `&mut` to do it --");
    println!("  which is why call_fn_mut takes `mut f`. A FnMut is NOT an Fn:");
    println!("  handing this to call_fn would not compile.");
    println!();

    println!("RUNG 3 -- FnOnce: the body CONSUMES the capture");
    let owned = String::from("consumed");
    let consumes = move || owned;
    println!("  call_fn_once(consumes) {}", call_fn_once(consumes));
    println!("  It gives the String away, so there is nothing left to give a");
    println!("  second time. Calling it twice is a move-out-of-a-moved-value");
    println!("  error, and the trait bound is the compiler recording that.");
    println!();

    println!("THE LADDER, NOT THE MENU");
    println!("  Fn      => also FnMut => also FnOnce");
    println!("  So a function taking FnOnce accepts all three, and one taking");
    println!("  Fn accepts only the first. Take the LOOSEST bound your body");
    println!("  needs -- FnOnce if you call it once -- and you accept the most");
    println!("  callers. That is the same reasoning as taking &str over String.");
    println!();

    println!("WHAT DOES NOT DECIDE THE RUNG");
    println!("  `move` does not. It says where the captured values live, not");
    println!("  what the body does with them: `move || println!(\"{{}}\", n)` on a");
    println!("  Copy type is still an Fn. The body is the whole test.");

    let mut n = 0;
    let mut bump = || n += 1;
    bump(); bump();
    assert_eq!(n, 2);
}
