//! Kata solution: three conversions, one of which the compiler forbids.
//!
//!   rustc --edition 2024 from_and_into_kata.rs -o /tmp/fik && /tmp/fik

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Score(u8);

#[derive(Debug, PartialEq)]
struct Ballot {
    voter: String,
    score: Score,
}

// 1. Local type FROM a foreign one. Always allowed: Score is ours.
impl From<u8> for Score {
    fn from(n: u8) -> Self {
        Score(n.min(5))
    }
}

// 2. Foreign type FROM a local one. Also allowed, for the same reason —
//    the *local* type is what the orphan rule looks for, on either side.
impl From<Score> for u8 {
    fn from(s: Score) -> Self {
        s.0
    }
}

impl From<Score> for String {
    fn from(s: Score) -> Self {
        format!("{}/5", s.0)
    }
}

// 3. A tuple of borrowed pieces into an owned struct.
impl From<(&str, u8)> for Ballot {
    fn from((voter, score): (&str, u8)) -> Self {
        Ballot { voter: voter.to_string(), score: Score::from(score) }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/5", self.0)
    }
}

/// The signature that takes anything convertible. One function, three callers.
fn record<B: Into<Ballot>>(b: B) -> String {
    let ballot: Ballot = b.into();
    format!("{} scored {}", ballot.voter, ballot.score)
}

fn main() {
    println!("1. Both directions, written once each");
    let s = Score::from(4u8);
    let back: u8 = s.into();
    let printed: String = s.into();
    println!("   Score::from(4u8)      = {s:?}");
    println!("   u8 from that Score    = {back}");
    println!("   String from it        = {printed:?}");

    println!();
    println!("2. What the orphan rule actually says");
    println!("   impl From<u8> for Score     OK   — Score is local");
    println!("   impl From<Score> for u8     OK   — Score is local");
    println!("   impl From<Score> for String OK   — Score is local");
    println!("   impl From<Vec<u8>> for String    E0117: \"only traits defined in");
    println!("   the current crate can be implemented for types defined outside");
    println!("   of the crate\". Neither Vec nor String is ours, so this impl");
    println!("   could be written twice, in two crates, with different answers.");
    println!("   The way round it is a newtype: wrap one side in a local struct.");

    println!();
    println!("3. `impl Into<T>` in an argument position");
    println!("   record((\"Ada\", 4))                 -> {}", record(("Ada", 4u8)));
    println!("   record(Ballot {{ .. }})              -> {}",
             record(Ballot { voter: "Ben".into(), score: Score(3) }));
    println!("   The second call works because of the reflexive impl — From<T> for");
    println!("   T exists for every T — so `Into<Ballot>` accepts a Ballot and");
    println!("   converts nothing. That is why this signature is a superset of");
    println!("   taking `Ballot` directly, and costs the caller nothing.");

    println!();
    println!("4. Where the bound goes, and what it costs");
    println!("   `fn record<B: Into<Ballot>>(b: B)` is monomorphised once per");
    println!("   argument type, so the convenience is paid for in code size, not");
    println!("   at run time. Taking `Ballot` and making the caller write");
    println!("   `.into()` is one generic parameter cheaper and one keystroke");
    println!("   more — which is the trade, and it is why std's own APIs use");
    println!("   `impl Into<String>` sparingly.");

    println!();
    println!("5. The conversion that should not be a From at all");
    println!("   Score::from(200u8) = {:?}   <- clamped, silently", Score::from(200u8));
    println!("   `From` promises success, so this impl had to choose between");
    println!("   clamping and panicking, and it chose the one that loses data");
    println!("   quietly. A score arriving as 200 is a bug in the caller, and the");
    println!("   right trait for reporting it is TryFrom.");
}
