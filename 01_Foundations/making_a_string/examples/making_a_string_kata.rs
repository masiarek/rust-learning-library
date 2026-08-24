//! Kata solution: implement Display once, collect four abilities.
//!
//!   rustc --edition 2024 making_a_string_kata.rs -o /tmp/msk && /tmp/msk

use std::fmt;

#[derive(Debug)]
struct Ballot {
    voter: &'static str,
    scores: [u8; 3],
}

impl fmt::Display for Ballot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.voter, self.scores.map(|s| s.to_string()).join("/"))
    }
}

// Adding this to a type that already implements Display does not compile:
//
//   impl ToString for Ballot {
//       fn to_string(&self) -> String { … }
//   }
//
//   error[E0119]: conflicting implementations of trait `ToString` for type `Ballot`
//     |
//   6 | impl ToString for Ballot {
//     | ^^^^^^^^^^^^^^^^^^^^^^^^
//     |
//     = note: conflicting implementation in crate `alloc`:
//             - impl<T> ToString for T
//               where T: std::fmt::Display, T: ?Sized;

/// One signature that accepts anything printable — the payoff for implementing Display.
fn label(x: impl fmt::Display) -> String {
    x.to_string()
}

fn main() {
    let b = Ballot { voter: "Ada", scores: [5, 2, 0] };

    println!("One impl, four abilities:");
    println!("   {{}}            {b}");
    println!("   to_string()   {:?}", b.to_string());
    println!("   format!()     {:?}", format!("<{b}>"));
    println!("   impl Display  {:?}", label(&b));

    println!("\nThe same function serves every printable type:");
    println!("   label(42)          {:?}", label(42));
    println!("   label(true)        {:?}", label(true));
    println!("   label('x')         {:?}", label('x'));
    println!("   label(\"a literal\") {:?}", label("a literal"));
    println!("   label(3.5)         {:?}", label(3.5));
    println!("   label(&b)          {:?}", label(&b));

    println!("\nDebug is the other one, and it is NOT free:");
    println!("   {{:?}}   {b:?}   <- from #[derive(Debug)]");
    println!("   Display is for users and you write it; Debug is for you and you derive it.");

    println!("\nWhat you did NOT have to write: ToString, and it would not compile if you tried.");
}
