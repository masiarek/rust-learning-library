//! Kata solution: same four inputs, once without the reason and once with it.
//!
//!   rustc --edition 2024 option_vs_result_kata.rs -o /tmp/ovrk && /tmp/ovrk

use std::fmt;
use std::num::ParseIntError;

/// The Option version. Every failure arrives as the same `None`.
fn score_option(raw: &str) -> Option<u8> {
    let n: u8 = raw.trim().parse().ok()?;
    if n <= 5 { Some(n) } else { None }
}

#[derive(Debug)]
enum ScoreError {
    Empty,
    NotANumber(ParseIntError),
    AboveCap { got: u8, cap: u8 },
}

impl fmt::Display for ScoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScoreError::Empty => write!(f, "the cell was blank"),
            ScoreError::NotANumber(e) => write!(f, "not a number: {e}"),
            ScoreError::AboveCap { got, cap } => write!(f, "score {got} is above the {cap} cap"),
        }
    }
}

// This `From` is what lets `?` convert a ParseIntError on its own.
impl From<ParseIntError> for ScoreError {
    fn from(e: ParseIntError) -> Self {
        ScoreError::NotANumber(e)
    }
}

/// The Result version. Each failure keeps the thing the caller might act on.
fn score_result(raw: &str) -> Result<u8, ScoreError> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(ScoreError::Empty);
    }
    let n: u8 = raw.parse()?; // ParseIntError -> ScoreError, via From
    if n > 5 {
        return Err(ScoreError::AboveCap { got: n, cap: 5 });
    }
    Ok(n)
}

fn main() {
    let cells = ["4", "", "x", "9"];

    println!("Option — one sad answer for four different problems:");
    for raw in cells {
        println!("  {raw:>3?} -> {:?}", score_option(raw));
    }

    println!("\nResult — the caller can tell them apart, and tell the voter:");
    for raw in cells {
        match score_result(raw) {
            Ok(n) => println!("  {raw:>3?} -> counted as {n}"),
            Err(e) => println!("  {raw:>3?} -> rejected: {e}"),
        }
    }

    println!("\nAnd going back down throws it away again:");
    println!("  score_result(\"9\").ok() -> {:?}", score_result("9").ok());
}
