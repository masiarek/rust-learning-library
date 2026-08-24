//! Kata solution: three flavors, and the two things the compiler keeps apart.
//!
//!   rustc --edition 2024 what_a_struct_is_kata.rs -o /tmp/wasik && /tmp/wasik

use std::fmt;

// Same three fields, same types, two different TYPES. This is the whole
// point of a tuple struct over a bare tuple: the name is load-bearing.
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn paint(c: &Color) -> String {
    format!("#{:02x}{:02x}{:02x}", c.0, c.1, c.2)
}

mod sealed {
    // `pub` on the STRUCT is not `pub` on the FIELD, and for a tuple struct
    // that also makes the CONSTRUCTOR private — you cannot write Ballot(9)
    // from outside this module, because doing so would set a private field.
    #[derive(Debug)]
    pub struct Ballot(u32);

    impl Ballot {
        pub fn new(n: u32) -> Self {
            Ballot(n)
        }
        pub fn get(&self) -> u32 {
            self.0
        }
    }
}

// A unit struct carries no data, so the only thing it can be is a place to
// hang behaviour. That is not a consolation prize — it is the use case.
struct Blank;

impl fmt::Display for Blank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(no ballot returned)")
    }
}

fn main() {
    println!("1. Identical shapes, different types");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("   Color(0,0,0) -> {}", paint(&black));
    println!("   {origin:?} is the same THREE i32s and paint(&origin) will not compile:");
    println!("     error[E0308]: mismatched types — expected `&Color`, found `&Point`");
    println!("   A bare (i32, i32, i32) would have accepted either. The name is the guard.");

    println!("\n2. Destructuring — the tuple struct comes apart like a tuple");
    let Color(r, g, b) = black;
    println!("   let Color(r, g, b) = black;  ->  r={r} g={g} b={b}");
    let Point(x, y, z) = origin;
    println!("   let Point(x, y, z) = origin; ->  x={x} y={y} z={z}");
    println!("   Same shape, same destructuring, and still not interchangeable.");

    println!("\n3. A private field makes a tuple struct's CONSTRUCTOR private");
    let ballot = sealed::Ballot::new(431);
    println!("   sealed::Ballot::new(431) -> {ballot:?}, get() = {}", ballot.get());
    println!("   sealed::Ballot(431) from out here is E0603:");
    println!("     `a constructor is private if any of the fields is private`");
    println!("   That is the newtype's whole guarantee: one door, and the module owns it.");

    println!("\n4. A unit struct holds nothing, so behaviour is all it can hold");
    println!("   {Blank}   <- via its Display impl; the value carries no data at all");
}
