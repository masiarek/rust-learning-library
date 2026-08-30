//! Kata solution: four causes, one `None` — and then four causes kept apart.
//!
//!   rustc --edition 2024 none_on_error_kata.rs -o /tmp/noek && /tmp/noek

use std::fmt;
use std::num::ParseIntError;

/// The line everyone writes first. `.ok()` is where the reason goes to die.
fn seats_lossy(raw: &str) -> Option<u32> {
    raw.trim().parse::<u32>().ok().filter(|n| (1..=20).contains(n))
}

#[derive(Debug)]
enum SeatsError {
    Missing,
    NotANumber(ParseIntError),
    Zero,
    TooMany(u32),
}

impl fmt::Display for SeatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeatsError::Missing => write!(f, "no seat count was given"),
            SeatsError::NotANumber(e) => write!(f, "not a whole number: {e}"),
            SeatsError::Zero => write!(f, "an election with 0 seats elects nobody"),
            SeatsError::TooMany(n) => write!(f, "{n} seats is past the 20 this ballot supports"),
        }
    }
}

fn seats(raw: &str) -> Result<u32, SeatsError> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(SeatsError::Missing);
    }
    let n = raw.parse::<u32>().map_err(SeatsError::NotANumber)?;
    match n {
        0 => Err(SeatsError::Zero),
        1..=20 => Ok(n),
        _ => Err(SeatsError::TooMany(n)),
    }
}

fn main() {
    let lines = ["3", "", "three", "0", "99"];

    println!("Option — the caller cannot tell a typo from a policy limit:");
    for raw in lines {
        println!("  {:>7} -> {:?}", format!("{raw:?}"), seats_lossy(raw));
    }

    println!("\nResult — every one of them can be answered differently:");
    for raw in lines {
        match seats(raw) {
            Ok(n) => println!("  {:>7} -> {n} seats", format!("{raw:?}")),
            Err(e) => println!("  {:>7} -> {e}", format!("{raw:?}")),
        }
    }

    println!("\nThe one case where None was right all along:");
    let ballots = ["ada", "ben"];
    println!("  ballots.iter().position(|b| *b == \"cara\") -> {:?}", ballots.iter().position(|b| *b == "cara"));
    println!("      There is exactly one reason that is absent, and the caller");
    println!("      already knows it. Nothing was discarded to say None here.");
}
