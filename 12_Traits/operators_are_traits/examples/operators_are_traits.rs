//! `a + b` is `Add::add(a, b)`. Every operator is a trait, and you can have one.
//!
//!   rustc --edition 2024 operators_are_traits.rs -o /tmp/ops && /tmp/ops

use std::fmt;
use std::ops::{Add, AddAssign, Index, Mul, Neg, Sub};

/// Points on a ballot. A newtype, so `Points + u32` cannot happen by accident.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Points(i32);

impl Add for Points {
    type Output = Points;
    fn add(self, other: Points) -> Points {
        Points(self.0 + other.0)
    }
}

impl Sub for Points {
    type Output = Points;
    fn sub(self, other: Points) -> Points {
        Points(self.0 - other.0)
    }
}

/// A different right-hand type: Points * i32, not Points * Points.
impl Mul<i32> for Points {
    type Output = Points;
    fn mul(self, weight: i32) -> Points {
        Points(self.0 * weight)
    }
}

impl AddAssign for Points {
    fn add_assign(&mut self, other: Points) {
        self.0 += other.0;
    }
}

impl Neg for Points {
    type Output = Points;
    fn neg(self) -> Points {
        Points(-self.0)
    }
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}pt", self.0)
    }
}

/// Indexing is a trait too, and its Output is a reference.
struct Ballot {
    scores: [Points; 3],
}

impl Index<usize> for Ballot {
    type Output = Points;
    fn index(&self, i: usize) -> &Points {
        &self.scores[i]
    }
}

fn main() {
    println!("1. The operator is the method");
    let a = Points(5);
    let b = Points(3);
    println!("   a + b        = {}   is Add::add(a, b)", a + b);
    println!("   Add::add(a, b) = {}   the same call, written out", Add::add(a, b));
    println!("   a - b        = {}", a - b);
    println!("   -a           = {}", -a);
    println!("   a * 3        = {}   Mul<i32>, a DIFFERENT trait from Mul<Points>", a * 3);

    println!();
    println!("2. `type Output` is why `+` can change the type");
    println!("   Add's signature is `fn add(self, rhs: Rhs) -> Self::Output`, so");
    println!("   none of the three types has to match. `Instant + Duration` is an");
    println!("   Instant, `Instant - Instant` is a DURATION, and `String + &str` is");
    println!("   a String. Here Output is Points because that is what made sense,");
    println!("   not because the trait required it.");

    println!();
    println!("3. The assigning forms are separate traits");
    let mut running = Points(0);
    for score in [Points(5), Points(3), Points(4)] {
        running += score;
    }
    println!("   after three `+=`: {running}");
    println!("   `+=` is AddAssign, and implementing Add does NOT give it to you.");
    println!("   The difference is real: add_assign takes &mut self and can mutate");
    println!("   in place, where add consumes and returns. For a Copy newtype that");
    println!("   is a formality; for a String or a Vec it is an allocation saved.");

    println!();
    println!("4. Indexing, and what it returns");
    let ballot = Ballot { scores: [Points(5), Points(3), Points(0)] };
    println!("   ballot[1] = {}", ballot[1]);
    println!("   Index::index returns a REFERENCE — `&Self::Output` — and `[]` is");
    println!("   sugar for `*ballot.index(1)`. That is why a HashMap's `[]` panics");
    println!("   rather than returning Option: the trait has nowhere to put a None.");

    println!();
    println!("5. What the operators buy, and the trap");
    let scores = [Points(5), Points(3), Points(4)];
    let total = scores.iter().copied().fold(Points(0), |acc, p| acc + p);
    let sorted = {
        let mut v = scores;
        v.sort();
        v
    };
    println!("   fold with + : {total}");
    println!("   sorted (Ord): {sorted:?}");
    println!("   A newtype with the right operators reads exactly like the number");
    println!("   it wraps, while still refusing `Points + Seats`. That is the");
    println!("   whole argument for the newtype: the type is a unit.");
    println!("   The trap is the other direction — implementing Add on a type whose");
    println!("   addition is not obvious. `Ballot + Ballot` could be a merge, a");
    println!("   concatenation, or an error, and the operator hides the question.");
    println!("   If the answer is not the one every reader would guess, write a");
    println!("   method with a name.");

    println!();
    println!("6. What cannot be overloaded");
    println!("   && and ||   they short-circuit; a trait method cannot");
    println!("   =           assignment is not a trait");
    println!("   .           field access and method calls (Deref is not this)");
    println!("   ?           it has its own traits, Try and FromResidual, unstable");
    println!("   Everything else — + - * / % & | ^ << >> ! [] () == < > and the");
    println!("   nine assigning forms — is a trait in std::ops or std::cmp.");
}
