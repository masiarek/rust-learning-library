//! `TryFrom`: the conversion that is allowed to say no, and has to say why.
//!
//!   rustc --edition 2024 tryfrom_and_tryinto.rs -o /tmp/tf && /tmp/tf

use std::convert::TryFrom;
use std::fmt;

/// A score on the 0-5 ballot. Unlike the `From` version, this one refuses.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Score(u8);

#[derive(Debug, PartialEq)]
struct ScoreOutOfRange(u8);

impl fmt::Display for ScoreOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is outside the 0-5 range", self.0)
    }
}

impl TryFrom<u8> for Score {
    type Error = ScoreOutOfRange;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        if n <= 5 { Ok(Score(n)) } else { Err(ScoreOutOfRange(n)) }
    }
}

/// The whole point: a caller cannot get a Score without handling the refusal.
fn read_ballot(cells: &[u8]) -> Result<Vec<Score>, ScoreOutOfRange> {
    cells.iter().map(|&n| Score::try_from(n)).collect()
}

fn main() {
    println!("1. One impl, two call syntaxes, one of them fallible");
    println!("   Score::try_from(4) = {:?}", Score::try_from(4u8));
    println!("   Score::try_from(9) = {:?}", Score::try_from(9u8));
    let as_result: Result<Score, _> = 4u8.try_into();
    println!("   4u8.try_into()     = {as_result:?}");
    println!("   `TryInto` comes free from `TryFrom`, exactly as `Into` comes free");
    println!("   from `From` — same blanket impl, one Result deeper.");

    println!();
    println!("2. The difference from From, in one line");
    println!("   From has no error type, so a conversion that cannot honour its");
    println!("   input must clamp, wrap or panic. TryFrom has `type Error`, so it");
    println!("   can hand the bad value back and let the caller decide.");

    println!();
    println!("3. std's own, which you will meet first");
    println!("   u8::try_from(300i32)  = {:?}", u8::try_from(300i32));
    println!("   u8::try_from(200i32)  = {:?}", u8::try_from(200i32));
    println!("   u8::try_from(-1i32)   = {:?}", u8::try_from(-1i32));
    println!("   i32::try_from(9u64)   = {:?}", i32::try_from(9u64));
    println!("   The error is `TryFromIntError`, and it carries nothing — the");
    println!("   failure has exactly one cause, so there is nothing to report.");
    println!("   Compare that with ScoreOutOfRange above, which keeps the value.");

    println!();
    println!("4. Collecting into a Result stops at the first refusal");
    println!("   read_ballot([5, 3, 0])    = {:?}", read_ballot(&[5, 3, 0]));
    println!("   read_ballot([5, 9, 0])    = {:?}", read_ballot(&[5, 9, 0]));
    println!("   `Vec<Result<T, E>>` collected into `Result<Vec<T>, E>` is one of");
    println!("   the most useful shapes in std: it short-circuits, so the third");
    println!("   cell is never converted, and the caller gets one error rather");
    println!("   than a vector of them.");

    println!();
    println!("5. The trap: `as` is the same conversion with the check deleted");
    let big = 300i32;
    println!("   u8::try_from({big}) = {:?}", u8::try_from(big));
    println!("   {big} as u8         = {}", big as u8);
    println!("   Both are \"convert i32 to u8\". One reports that it could not; the");
    println!("   other returns {} and says nothing. Reach for `as` when truncation", big as u8);
    println!("   is what you meant, and for try_from when it is not.");
}
