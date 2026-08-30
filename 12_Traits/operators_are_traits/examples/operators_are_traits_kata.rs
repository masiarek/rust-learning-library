//! Kata solution: four impls for one `*`, and an operator that should not exist.
//!
//!   rustc --edition 2024 operators_are_traits_kata.rs -o /tmp/opk && /tmp/opk

use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Points(i32);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Turnout(f64);

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}pt", self.0)
    }
}

// 1. Points * i32
impl Mul<i32> for Points {
    type Output = Points;
    fn mul(self, n: i32) -> Points {
        Points(self.0 * n)
    }
}

// 2. i32 * Points — a SEPARATE impl, and legal only because Points is local.
impl Mul<Points> for i32 {
    type Output = Points;
    fn mul(self, p: Points) -> Points {
        Points(self * p.0)
    }
}

// 3. &Points * i32, so a borrowed value works without a deref at the call site.
impl Mul<i32> for &Points {
    type Output = Points;
    fn mul(self, n: i32) -> Points {
        Points(self.0 * n)
    }
}

// 4. Points + Points, the ordinary one.
impl Add for Points {
    type Output = Points;
    fn add(self, other: Points) -> Points {
        Points(self.0 + other.0)
    }
}

/// Deliberately NOT `impl Add for Turnout` — see section 3.
impl Turnout {
    fn combine(self, other: Turnout, voters: (f64, f64)) -> Turnout {
        let (a, b) = voters;
        Turnout((self.0 * a + other.0 * b) / (a + b))
    }
}

fn main() {
    println!("1. Four impls, and what each one is for");
    let p = Points(5);
    println!("   Points * i32   : {}", p * 3);
    println!("   i32 * Points   : {}", 3 * p);
    println!("   &Points * i32  : {}", &p * 3);
    println!("   Points + Points: {}", p + Points(3));
    println!("   `3 * p` is NOT free once you have written `p * 3`. Mul is generic");
    println!("   over its right-hand type, not symmetric: `impl Mul<i32> for Points`");
    println!("   and `impl Mul<Points> for i32` are two unrelated impls.");

    println!();
    println!("2. Why the second one is allowed at all");
    println!("   `impl Mul<Points> for i32` implements a foreign trait for a");
    println!("   FOREIGN type — and it compiles, because the orphan rule looks at");
    println!("   the whole impl: a local type appears in it, as the parameter.");
    println!("   That is why std can write `impl Add<&str> for String` and why you");
    println!("   can write this one, and also why `impl Mul<i32> for f64` is");
    println!("   refused: nothing in it is yours.");

    println!();
    println!("3. The operator that should not exist");
    let morning = Turnout(0.80);
    let evening = Turnout(0.40);
    println!("   morning {:.0}%, evening {:.0}%", morning.0 * 100.0, evening.0 * 100.0);
    println!("   a naive `+`      would give {:.0}%", (morning.0 + evening.0) * 100.0);
    println!("   the mean         would give {:.0}%", (morning.0 + evening.0) / 2.0 * 100.0);
    let combined = morning.combine(evening, (100.0, 900.0));
    println!("   weighted by size : {:.0}%   <- the only right answer", combined.0 * 100.0);
    println!("   Three plausible readings of `morning + evening`, and the correct");
    println!("   one needs an argument the operator has nowhere to put. So Turnout");
    println!("   has no Add: it has `combine(other, voters)`, whose name and");
    println!("   signature say what it does. An operator hides the question, and");
    println!("   the reader never learns there was one.");

    println!();
    println!("4. The test for whether to implement an operator");
    println!("   Would every reader guess the same answer without reading the impl?");
    println!("   Points + Points     yes — it is a count");
    println!("   Turnout + Turnout   no  — three readings, above");
    println!("   Ballot + Ballot     no  — merge? concatenate? refuse?");
    println!("   Path / &str         yes, and std does exactly that for PathBuf");
    println!("   When the answer is no, a named method is not a worse API. It is");
    println!("   the honest one.");

    println!();
    println!("5. What you still get for free, and what you do not");
    let scores = [Points(5), Points(3), Points(4)];
    let total: Points = scores.iter().copied().fold(Points(0), |a, b| a + b);
    println!("   fold with `+`  : {total}");
    println!("   `.sum()`       : needs `impl Sum for Points` — Iterator::sum is a");
    println!("                    trait method with its own bound, and Add alone");
    println!("                    does not satisfy it.");
    println!("   `+=`           : needs AddAssign, separately.");
    println!("   `Points * 3.0` : needs Mul<f64>, separately.");
    println!("   Each operator is one trait for one pair of types. That is verbose,");
    println!("   and it is why `Points + Seats` cannot compile by accident.");
}
