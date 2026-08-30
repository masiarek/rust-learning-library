//! Kata solution: a ballot line, parsed three ways, two of which accept junk.
//!
//!   rustc --edition 2024 tryfrom_and_tryinto_kata.rs -o /tmp/tfk && /tmp/tfk

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Score(u8);

#[derive(Debug, PartialEq)]
enum ScoreError {
    NotANumber(String),
    OutOfRange(u32),
}

impl fmt::Display for ScoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScoreError::NotANumber(s) => write!(f, "{s:?} is not a number"),
            ScoreError::OutOfRange(n) => write!(f, "{n} is outside 0-5"),
        }
    }
}

impl TryFrom<u32> for Score {
    type Error = ScoreError;
    fn try_from(n: u32) -> Result<Self, Self::Error> {
        if n <= 5 {
            Ok(Score(n as u8))
        } else {
            Err(ScoreError::OutOfRange(n))
        }
    }
}

/// `FromStr` is what `.parse()` calls. Implement this rather than
/// `TryFrom<&str>` when the source is text: you get `"4".parse()?` free.
impl FromStr for Score {
    type Err = ScoreError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u32 = s
            .trim()
            .parse()
            .map_err(|_| ScoreError::NotANumber(s.to_string()))?;
        Score::try_from(n)
    }
}

/// The version that reports the first bad cell.
fn parse_line(line: &str) -> Result<Vec<Score>, ScoreError> {
    line.split(',').map(str::parse).collect()
}

/// The version that quietly accepts everything.
fn parse_line_as(line: &str) -> Vec<Score> {
    line.split(',')
        .map(|c| Score(c.trim().parse::<u32>().unwrap_or(0) as u8))
        .collect()
}

fn main() {
    println!("1. The good line");
    println!("   parse_line(\"5,3,0\")    = {:?}", parse_line("5,3,0"));
    println!("   `.parse()` reached FromStr, FromStr reached TryFrom, and collect");
    println!("   turned Vec<Result<_>> into Result<Vec<_>>.");

    println!();
    println!("2. The three ways one line can be wrong");
    for line in ["5,x,0", "5,9,0", "5, 3 ,0"] {
        match parse_line(line) {
            Ok(v) => println!("   {line:12} -> Ok({:?})", v.iter().map(|s| s.0).collect::<Vec<_>>()),
            Err(e) => println!("   {line:12} -> Err({e})"),
        }
    }
    println!("   The third is not an error: FromStr trims, so whitespace is fine.");
    println!("   Deciding that is part of writing the impl, and it is written down");
    println!("   in one place instead of at every call site.");

    println!();
    println!("3. The same lines through `as`");
    for line in ["5,x,0", "5,9,0"] {
        let v = parse_line_as(line);
        println!("   {line:12} -> {:?}", v.iter().map(|s| s.0).collect::<Vec<_>>());
    }
    println!("   Both produce a full, plausible, wrong ballot. `x` became a zero");
    println!("   score — indistinguishable from a real zero — and 9 became 9,");
    println!("   which is not a legal score at all: the `as u8` cast let it past");
    println!("   the type that exists to prevent exactly that.");

    println!();
    println!("4. Two traits, and which to implement");
    println!("   TryFrom<u32> for Score  -> Score::try_from(4u32), 4u32.try_into()");
    println!("   FromStr     for Score   -> \"4\".parse::<Score>()");
    println!("   Implementing FromStr also gives you `.parse()` inside iterator");
    println!("   chains, as `map(str::parse)` above — which is why `TryFrom<&str>`");
    println!("   is the wrong choice for a type parsed out of text.");

    println!();
    println!("5. The error type is the deliverable");
    let e = "9".parse::<Score>().unwrap_err();
    println!("   \"9\".parse::<Score>() error = {e}");
    println!("   debug form: {e:?}");
    println!("   ScoreError keeps the offending value, so the message can name it.");
    println!("   std's TryFromIntError does not, because there is only ever one");
    println!("   thing to say. Decide which of those your caller needs before you");
    println!("   write `type Error = String`.");
}
