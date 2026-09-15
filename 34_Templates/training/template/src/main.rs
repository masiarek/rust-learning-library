//! A practice program the training template has nothing to say about: a die roll
//! from `rand`, words from `regex`, JSON from `serde_json`, and every error passed
//! up with `?` through `anyhow`.

use anyhow::{Context, Result};
use rand::RngExt;
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Roll {
    sides: u8,
    value: u8,
}

fn main() -> Result<()> {
    let value = rand::rng().random_range(1..=6);
    let roll = Roll { sides: 6, value };
    println!("{}", serde_json::to_string(&roll)?); // {"sides":6,"value":4} -- the value varies

    let words = Regex::new(r"\b[a-z]+ing\b").context("the pattern should compile")?;
    let text = "reading and writing, then testing the thing";
    let found: Vec<&str> = words.find_iter(text).map(|word| word.as_str()).collect();
    println!("{found:?}"); // ["reading", "writing", "testing", "thing"]

    let total: u32 = "42".parse().context("not a whole number")?;
    println!("{}", total + 1); // 43

    Ok(())
}
