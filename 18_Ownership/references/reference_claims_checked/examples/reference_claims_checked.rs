//! Claims about references from a Stack Overflow thread, The Book,
//! Programming Rust and Learn Rust the Dangerous Way, run on rustc 1.98.0.
//! The refused programs are on the page as real rustc output.
//!
//!   rustc --edition 2024 reference_claims_checked.rs -o /tmp/reference_claims_checked && /tmp/reference_claims_checked

use std::cell::Cell;
use std::mem::size_of;
use std::rc::Rc;

/// The Book's cons list from ch. 15.4, with the fields read so it builds clean.
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn sum(list: &List) -> i32 {
    match list {
        Cons(head, tail) => head + sum(tail),
        Nil => 0,
    }
}

const BODIES_COUNT: usize = 5;

#[derive(Clone, Copy)]
struct Body {
    velocity: f64,
    mass: f64,
}

const INITIAL_BODIES: [Body; BODIES_COUNT] = [Body { velocity: 1.0, mass: 2.0 }; BODIES_COUNT];

/// Cliffle's offset_Momentum shape: the array length is part of the parameter type.
fn offset_momentum(bodies: &mut [Body; BODIES_COUNT]) {
    for i in 1..BODIES_COUNT {
        bodies[0].velocity -= bodies[i].velocity * bodies[i].mass / 16.0;
    }
}

fn main() {
    let w = size_of::<usize>();

    println!("1. Stack Overflow, accepted answer: the pointer snippet, once `x` is `mut`");
    let mut x: u32 = 12;
    let ref1: &u32 = &x;
    println!("   *ref1 = {}", *ref1); // ref1's borrow ends here, at its last use
    let ref2: &mut u32 = &mut x;
    *ref2 += 1;
    let raw2: *mut u32 = &mut x;
    unsafe { *raw2 += 1 };
    println!("   after *ref2 += 1 and *raw2 += 1: x = {x}");

    println!();
    println!("2. Stack Overflow, 2015 answer: a raw pointer from an integer is safe to make");
    let p = 123 as *const String;
    println!("   123 as *const String compiled with no unsafe; null? {}", p.is_null());
    let q = &raw const x;
    println!("   &raw const x, read in unsafe: {}", unsafe { *q });

    println!();
    println!("3. Programming Rust: at run time a reference is an address");
    println!("   &u32: {} word   &str: {} words   &[u8]: {} words", size_of::<&u32>() / w, size_of::<&str>() / w, size_of::<&[u8]>() / w);

    println!();
    println!("4. The Book, ch. 15.4, Listing 15-19: the counts");
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("   count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("   count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("   count after creating c = {}", Rc::strong_count(&a));
        println!("   sum(c) = {}", sum(&c));
    }
    println!("   count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("   sum(b) = {}", sum(&b));

    println!();
    println!("5. The Book, ch. 15.4: \"Rc<T> allows you to share data ... for reading only\"");
    let mut roster = Rc::new(String::from("roster"));
    if let Some(text) = Rc::get_mut(&mut roster) {
        text.push_str(" v2"); // count is 1: written in place
    }
    println!("   count 1, Rc::get_mut wrote in place: {roster:?}");
    let other = Rc::clone(&roster);
    println!("   count 2, Rc::get_mut is None: {}", Rc::get_mut(&mut roster).is_none());
    Rc::make_mut(&mut roster).push('!'); // count is 2: clones, then writes the clone
    println!("   count 2, Rc::make_mut cloned first: roster {roster:?}, other {other:?}");

    println!();
    println!("6. \"References have explicit lifetimes\": this borrow names none");
    let boxed = Box::new(42);
    let reference: &i32 = &*boxed;
    println!("   let reference: &i32 = &*boxed;  -> {reference}");

    println!();
    println!("7. Learn Rust the Dangerous Way, part 2, without the static mut");
    let mut bodies = INITIAL_BODIES;
    offset_momentum(&mut bodies);
    println!("   offset_momentum(&mut bodies): bodies[0].velocity = {}", bodies[0].velocity);

    println!();
    println!("8. std::cell: a Cell is written through shared references");
    let cell = Cell::new(1);
    let r1 = &cell;
    let r2 = &cell;
    r1.set(r2.get() + 1);
    println!("   two & to one Cell, r1.set(r2.get() + 1) -> {}", cell.get());
}
