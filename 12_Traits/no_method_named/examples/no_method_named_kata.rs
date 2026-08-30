//! Kata solution: three calls that all failed with `E0599`, three different
//! fixes. The cause is named beside each one.
//!
//!   rustc --edition 2024 no_method_named_kata.rs -o /tmp/nmnk && /tmp/nmnk

use std::fmt;
use std::fmt::Write; // FIX 2 — trait not in scope. `String` already had
                     // `write_str`; nothing could reach it without this line.

struct Precinct {
    ballots: u32,
    registered: u32,
}

// FIX 1 — the method did not exist. No import could have helped: nothing had
// ever written it.
impl Precinct {
    fn turnout(&self) -> f64 {
        (self.ballots as f64 / self.registered as f64) * 100.0
    }
}

struct Station {
    id: u32,
    town: &'static str,
}

// FIX 3 — the bound was not satisfied. `to_string` is never implemented by
// hand; it arrives for any `T: Display`, so `Display` is the missing piece.
impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "station {} ({})", self.id, self.town)
    }
}

fn main() {
    let p = Precinct { ballots: 812, registered: 1_000 };
    println!("1. inherent method written  -> p.turnout() = {:.1}%", p.turnout());

    let mut log = String::new();
    write!(log, "{} of {} voted", p.ballots, p.registered).unwrap();
    log.write_str(" — provisional")
        .expect("writing to a String cannot fail");
    println!("2. std::fmt::Write imported -> log = {log:?}");

    let s = Station { id: 7, town: "Ada Falls" };
    println!("3. Display implemented      -> s.to_string() = {:?}", s.to_string());

    println!();
    println!("The three causes, in the order the compiler rules them out:");
    println!("  1. no such method anywhere               -> write it");
    println!("  2. trait implemented, but not in scope   -> import it");
    println!("  3. trait (or its bound) not implemented  -> implement it");
}
