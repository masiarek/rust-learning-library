//! What a struct is: three flavors, one expression, and where behaviour lives.
//!
//!   rustc --edition 2024 what_a_struct_is.rs -o /tmp/wasi && /tmp/wasi

// ---- Flavor 1: named fields (the "classic C struct") ----------------------
#[derive(Debug)]
struct Ballot {
    voter: String,
    scores: Vec<u8>,
}

// ---- Flavor 2: tuple struct — same thing, fields anonymous ----------------
#[derive(Debug)]
struct Precinct(u32);

// ---- Flavor 3: unit struct — no fields, exactly one value -----------------
#[derive(Debug)]
struct Sealed;

// A fourth SPELLING, and it is not the same as flavor 3: braces with nothing
// in them. It must be constructed with braces; `Sealed` is a value by itself.
#[derive(Debug)]
struct AlsoEmpty {}

// ---- Behaviour lives OUTSIDE the struct, in an impl block -----------------
impl Ballot {
    /// Associated function: no `self`, called as `Ballot::new(..)`.
    /// This is the "constructor" — Rust has no special constructor syntax,
    /// it is just a function that happens to return Self.
    fn new(voter: &str) -> Self {
        Ballot { voter: voter.to_string(), scores: Vec::new() }
    }

    /// Method: takes `&self`, called as `ballot.total()`.
    fn total(&self) -> u32 {
        self.scores.iter().map(|&s| s as u32).sum()
    }

    /// Method taking `&mut self` — needs a `mut` binding at the call site.
    fn score(&mut self, s: u8) {
        self.scores.push(s);
    }
}

// ---- Privacy is per MODULE, not per struct --------------------------------
mod polling {
    #[derive(Debug)]
    pub struct Sealed {
        pub id: u32,
        count: u32, // no `pub`: invisible outside `polling`
    }

    impl Sealed {
        pub fn new(id: u32, count: u32) -> Self {
            Sealed { id, count } // field init shorthand
        }
        pub fn count(&self) -> u32 {
            self.count // inside the module, so this is allowed
        }
    }
}

fn main() {
    println!("1. Three flavors, and a fourth spelling");
    println!("   named   {:?}", Ballot::new("Ada"));
    println!("   tuple   {:?}   field reached by index: {}", Precinct(7), Precinct(7).0);
    println!("   unit    {:?}          — the type has exactly one value", Sealed);
    println!("   braces  {:?}      — `AlsoEmpty {{}}`, NOT the same as a unit struct", AlsoEmpty {});

    println!("\n2. A tuple struct is a named-field struct whose fields are numbers");
    let p = Precinct { 0: 7 }; // legal, and identical to Precinct(7)
    println!("   Precinct {{ 0: 7 }} == Precinct(7)  ->  {}", p.0 == Precinct(7).0);

    println!("\n3. Field ORDER in the expression is free; the value is the same");
    let a = Ballot { voter: "Ben".to_string(), scores: vec![5, 2] };
    let b = Ballot { scores: vec![5, 2], voter: "Ben".to_string() };
    println!("   {:?}", a);
    println!("   {:?}", b);
    println!("   same value: {}", a.voter == b.voter && a.scores == b.scores);

    println!("\n4. Data in the struct, behaviour in the impl block");
    let mut ballot = Ballot::new("Cara"); // associated function: Ballot::new
    ballot.score(5); //                      method on &mut self
    ballot.score(2);
    ballot.score(0);
    println!("   {} scored {:?}, total {}", ballot.voter, ballot.scores, ballot.total());
    println!("   Ballot::total(&ballot) == ballot.total()  ->  {}",
        Ballot::total(&ballot) == ballot.total());

    println!("\n5. Privacy is per module — `count` is private, and that is why");
    println!("   the module has to hand you a method to read it");
    let s = polling::Sealed::new(12, 431);
    println!("   id (pub)        {}", s.id);
    println!("   count (private) {}   <- via count(), not s.count", s.count());
    println!("   s.count would be E0616: field `count` of struct `Sealed` is private");
}
