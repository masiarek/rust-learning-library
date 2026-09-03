//! Kata solution: four causes, one `None` — and then four causes kept apart.
//!
//!   rustc --edition 2024 none_on_error_kata.rs -o /tmp/noek && /tmp/noek

use std::fmt;
use std::num::ParseIntError;

/// The line everyone writes first. `.ok()` is where the reason goes to die.
fn columns_lossy(raw: &str) -> Option<u32> {
    raw.trim().parse::<u32>().ok().filter(|n| (1..=20).contains(n))
}

#[derive(Debug)]
enum ColumnsError {
    Missing,
    NotANumber(ParseIntError),
    Zero,
    TooMany(u32),
}

impl fmt::Display for ColumnsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColumnsError::Missing => write!(f, "no seat count was given"),
            ColumnsError::NotANumber(e) => write!(f, "not a whole number: {e}"),
            ColumnsError::Zero => write!(f, "a table with 0 columns holds nothing"),
            ColumnsError::TooMany(n) => write!(f, "{n} columns is past the 20 this row supports"),
        }
    }
}

fn columns(raw: &str) -> Result<u32, ColumnsError> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(ColumnsError::Missing);
    }
    let n = raw.parse::<u32>().map_err(ColumnsError::NotANumber)?;
    match n {
        0 => Err(ColumnsError::Zero),
        1..=20 => Ok(n),
        _ => Err(ColumnsError::TooMany(n)),
    }
}

fn main() {
    let lines = ["3", "", "three", "0", "99"];

    println!("Option — the caller cannot tell a typo from a policy limit:");
    for raw in lines {
        println!("  {:>7} -> {:?}", format!("{raw:?}"), columns_lossy(raw));
    }

    println!("\nResult — every one of them can be answered differently:");
    for raw in lines {
        match columns(raw) {
            Ok(n) => println!("  {:>7} -> {n} columns", format!("{raw:?}")),
            Err(e) => println!("  {:>7} -> {e}", format!("{raw:?}")),
        }
    }

    println!("\nThe one case where None was right all along:");
    let rows = ["ada", "ben"];
    println!("  rows.iter().position(|b| *b == \"cara\") -> {:?}", rows.iter().position(|b| *b == "cara"));
    println!("      There is exactly one reason that is absent, and the caller");
    println!("      already knows it. Nothing was discarded to say None here.");
}
