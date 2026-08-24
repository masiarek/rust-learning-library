//! Kata solution: delete four unwraps from a config parser, one technique each.
//!
//! The starting version is the shape you get by copying a crate README: it
//! works on the happy path and panics on all four of the ways the input can be
//! wrong. Each rewrite below removes exactly one unwrap, and the last one is
//! the whole function as a pipeline.
//!
//!   rustc --edition 2024 unwrap_is_a_todo_kata.rs -o /tmp/uiatk && /tmp/uiatk

use std::num::ParseIntError;

/// What a bad config line can be, named rather than panicked.
#[derive(Debug)]
enum ConfigError {
    NoEquals,
    UnknownKey(String),
    BadNumber(ParseIntError),
    OutOfRange { value: u32, max: u32 },
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

/// The version that panics — kept only so the page can show what it replaces.
///
/// ```ignore
/// fn parse_quorum_panicking(line: &str) -> u32 {
///     let (key, value) = line.split_once('=').unwrap();   // 1. no '='
///     assert!(key == "quorum");                           // 2. wrong key
///     let n: u32 = value.parse().unwrap();                // 3. not a number
///     assert!(n <= 100);                                  // 4. out of range
///     n
/// }
/// ```
///
/// Every one of those four lines is a `todo!` somebody forgot to remove.
fn parse_quorum(line: &str) -> Result<u32, ConfigError> {
    // 1. `ok_or` turns the Option from split_once into a Result.
    let (key, value) = line.split_once('=').ok_or(ConfigError::NoEquals)?;

    // 2. A guard that returns instead of asserting.
    if key.trim() != "quorum" {
        return Err(ConfigError::UnknownKey(key.trim().to_string()));
    }

    // 3. `?` plus the From impl above: ParseIntError becomes ConfigError.
    let n: u32 = value.trim().parse()?;

    // 4. The range check as a value, carrying both numbers.
    if n > 100 {
        return Err(ConfigError::OutOfRange { value: n, max: 100 });
    }
    Ok(n)
}

/// The same thing as one pipeline, for when every step is an expression.
fn parse_quorum_pipeline(line: &str) -> Result<u32, ConfigError> {
    line.split_once('=')
        .ok_or(ConfigError::NoEquals)
        .and_then(|(key, value)| {
            if key.trim() == "quorum" {
                Ok(value)
            } else {
                Err(ConfigError::UnknownKey(key.trim().to_string()))
            }
        })
        .and_then(|value| value.trim().parse::<u32>().map_err(ConfigError::BadNumber))
        .and_then(|n| {
            if n > 100 {
                Err(ConfigError::OutOfRange { value: n, max: 100 })
            } else {
                Ok(n)
            }
        })
}

/// And the caller that does not care why, only what to use instead.
fn quorum_or_default(line: &str) -> u32 {
    parse_quorum(line).unwrap_or(51)
}

fn main() {
    let lines = [
        "quorum=60",
        "quorum = 7 ",
        "quorum",
        "seats=4",
        "quorum=lots",
        "quorum=900",
    ];

    println!("{:<14} {:<44} {}", "input", "parse_quorum", "or_default");
    println!("{}", "-".repeat(74));
    for line in lines {
        let got = parse_quorum(line);
        // The two implementations must agree on every input; that is the test.
        let pipeline = parse_quorum_pipeline(line);
        assert_eq!(format!("{got:?}"), format!("{pipeline:?}"));
        println!(
            "{:<14} {:<44} {}",
            format!("{line:?}"),
            format!("{got:?}"),
            quorum_or_default(line)
        );
    }
    println!("\nboth implementations agreed on all {} inputs", lines.len());
}
