//! `app`: which of the given words match a pattern, and one of them picked at random.
//!
//! ```text
//! cargo run -- --pattern 'ing$' reading writing arithmetic
//! ```

use anyhow::{Context, Result};
use clap::Parser;
use rand::seq::IndexedRandom;
use regex::Regex;
use serde::Serialize;

/// Report which words match a regular expression.
#[derive(Debug, Parser)]
struct Args {
    /// The regular expression each word is matched against.
    #[arg(long, default_value = "ing$")]
    pattern: String,

    /// The words to test.
    #[arg(default_values = ["reading", "writing", "arithmetic"])]
    words: Vec<String>,
}

/// What `app` prints, as JSON.
#[derive(Debug, Serialize)]
struct Report<'a> {
    pattern: &'a str,
    matched: Vec<&'a str>,
    picked: &'a str,
}

/// The one failure this program names itself; `anyhow` carries the rest.
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("no word matches {0:?}")]
    NoMatch(String),
}

fn main() -> Result<()> {
    let args = Args::parse();
    let pattern = Regex::new(&args.pattern)
        .with_context(|| format!("{:?} is not a valid regular expression", args.pattern))?;
    let matched: Vec<&str> = args
        .words
        .iter()
        .map(String::as_str)
        .filter(|word| pattern.is_match(word))
        .collect();
    let Some(&picked) = matched.choose(&mut rand::rng()) else {
        return Err(AppError::NoMatch(args.pattern).into());
    };
    let report = Report {
        pattern: &args.pattern,
        matched,
        picked,
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
