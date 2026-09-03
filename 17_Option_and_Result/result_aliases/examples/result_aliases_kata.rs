//! Kata solution: expand the alias and find out what can go wrong.
//!
//!   rustc --edition 2024 result_aliases_kata.rs -o /tmp/rak && /tmp/rak

use std::convert::Infallible;
use std::fmt;

#[derive(Debug)]
pub enum RowError {
    NoRows,
    BadRow { row: usize },
}

impl fmt::Display for RowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RowError::NoRows => write!(f, "no rows were supplied"),
            RowError::BadRow { row } => write!(f, "row {row} is not a number"),
        }
    }
}

/// The alias: one parameter instead of two, with the E pinned once.
pub type Result<T> = std::result::Result<T, RowError>;

/// Reads as `-> Result<u32>`, and is really `-> std::result::Result<u32, RowError>`.
fn total(rows: &[&str]) -> Result<u32> {
    if rows.is_empty() {
        return Err(RowError::NoRows);
    }
    let mut sum = 0;
    for (i, r) in rows.iter().enumerate() {
        sum += r.trim().parse::<u32>().map_err(|_| RowError::BadRow { row: i })?;
    }
    Ok(sum)
}

/// `Ok(())` — the same trick in the other slot. Nothing to return, but it can fail.
fn check(rows: &[&str]) -> Result<()> {
    total(rows)?;
    Ok(())
}

/// An E with no values at all: this function cannot fail, and the type says so.
fn always_ok(n: u32) -> std::result::Result<u32, Infallible> {
    Ok(n * 2)
}

fn main() {
    println!("The alias hides the E; the behaviour is unchanged:");
    for rows in [vec!["5", "3"], vec![], vec!["5", "x"]] {
        match total(&rows) {
            Ok(n) => println!("  {rows:?} -> {n}"),
            Err(e) => println!("  {rows:?} -> {e}"),
        }
    }

    println!("\nOk(()) — the success that carries nothing:");
    println!("  check(['5','3']) -> {:?}", check(&["5", "3"]));
    println!("  check([])        -> {:?}", check(&[]));

    println!("\nInfallible — an error type with no values:");
    println!("  always_ok(21) -> {:?}", always_ok(21));
    println!("      There is no way to build the Err, so the match arm for it");
    println!("      cannot be reached — which is the compiler's own reason for");
    println!("      letting `let Ok(n) = always_ok(21);` be irrefutable.");

    println!("\nReading the E back is a matter of following one line:");
    println!("  `type Result<T> = std::result::Result<T, RowError>;`");
    println!("  so `-> Result<u32>` fails with RowError, whose variants are the");
    println!("  actual list of things that can go wrong: NoRows, BadRow.");
    println!("  std::io::Result<T>, std::fmt::Result and ParseResult are the same");
    println!("  trick, and the E is the thing you were looking for in each.");
}
