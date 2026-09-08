//! Kata solution: eight bytes of code address, and nothing else.
//!
//! `fn(u32) -> u32` is a pointer to code. It has no environment, so anything
//! that captured cannot become one -- and the coercion that DOES work is easy
//! to miss, because it happens silently on closures that captured nothing.
//!
//!   rustc --edition 2024 function_pointers_kata.rs -o /tmp/fpk && /tmp/fpk

use std::mem::size_of;
use std::mem::size_of_val;

fn double(n: u32) -> u32 { n * 2 }

fn apply_ptr(f: fn(u32) -> u32, n: u32) -> u32 { f(n) }
fn apply_generic<F: Fn(u32) -> u32>(f: F, n: u32) -> u32 { f(n) }

fn main() {
    println!("1. THE SIZES");
    println!("  fn(u32) -> u32                    {} bytes", size_of::<fn(u32) -> u32>());
    let non_capturing = |n: u32| n * 2;
    let factor = 3u32;
    let capturing = move |n: u32| n * factor;
    println!("  a closure capturing nothing       {} bytes", size_of_val(&non_capturing));
    println!("  a closure capturing one u32       {} bytes", size_of_val(&capturing));
    println!("  A function pointer is one address. A zero-capture closure is");
    println!("  zero bytes -- SMALLER -- because the code it runs is known at");
    println!("  compile time and does not need to be pointed at.");
    println!();

    println!("2. THE COERCION THAT WORKS");
    let p: fn(u32) -> u32 = non_capturing;
    println!("  a non-capturing closure coerces:  apply_ptr(p, 21) = {}", apply_ptr(p, 21));
    let q: fn(u32) -> u32 = double;
    println!("  a plain fn item coerces too:      apply_ptr(q, 21) = {}", apply_ptr(q, 21));
    println!();

    println!("3. THE ONE THAT CANNOT");
    println!("  `let r: fn(u32) -> u32 = capturing;` does not compile:");
    println!("  a closure that captured has an environment, and a function");
    println!("  pointer has nowhere to put one. The error says exactly that --");
    println!("  \"closures can only be coerced to `fn` types if they do not");
    println!("  capture any variables\".");
    println!("  apply_generic(capturing, 7) = {}", apply_generic(capturing, 7));
    println!("  ...which works, because a generic bound accepts any closure and");
    println!("  monomorphises for its type.");
    println!();

    println!("4. NAMING A FUNCTION DOES NOT GIVE YOU A POINTER EITHER");
    println!("  `double` on its own has a zero-sized FN ITEM type, unique to");
    println!("  that one function -- {} bytes. It COERCES to a pointer when a", size_of_val(&double));
    println!("  pointer is what the context asks for, which is why passing it");
    println!("  to apply_ptr works and why `let x = double;` gives you the item");
    println!("  type rather than the pointer.");
    println!();

    println!("WHEN TO WANT A POINTER");
    println!("  An FFI boundary -- C has nothing else. A table of handlers you");
    println!("  want all one type. Otherwise take a generic `impl Fn`: it is");
    println!("  smaller, it accepts more callers, and it can be inlined.");

    assert_eq!(size_of_val(&non_capturing), 0);
    assert_eq!(size_of_val(&double), 0);
    assert_eq!(apply_ptr(p, 21), 42);
}
