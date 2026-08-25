//! Guarding the wrong condition: how a fabricated answer walks out through `Ok`.
//!
//! A `Result` return type proves the author knew the function could fail. It does
//! not prove they guarded the case that actually has no answer — and when they
//! guarded a different one, the undefined input leaves through the happy arm as a
//! plausible number. Zero is the number it usually leaves as.
//!
//!   rustc --edition 2024 wrong_guard.rs -o /tmp/wg && /tmp/wg

use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

/// Wins and losses. "Expansion" joined this season and has not played yet.
static TEAMS: LazyLock<HashMap<&str, (u32, u32)>> = LazyLock::new(|| {
    HashMap::from([("Bears", (9, 8)), ("Lions", (0, 17)), ("Expansion", (0, 0))])
});

const ROSTER: [&str; 4] = ["Bears", "Lions", "Expansion", "Sharks"];

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
/// The listing as it is usually written. Two bugs; neither is a type error.
fn winning_ratio_as_written(team: &String) -> Result<f32, &str> {
    if let Some(&(wins, losses)) = TEAMS.get(team.as_str()) {
        if wins == 0 {
            Ok(wins as f32)
        } else {
            Ok(((wins + losses) as f32) / wins as f32)
        }
    } else {
        Err("Invalid team")
    }
}

fn step1() {
    banner(1, "It compiles, it returns Result, and it is wrong twice");

    for name in ROSTER {
        println!("  {name:>10} -> {:?}", winning_ratio_as_written(&name.to_string()));
    }
    println!("      Bears are 9-8. A winning ratio of 1.89 is not a ratio at all:");
    println!("      (wins + losses) / wins is games PER WIN. Both sides are f32, so");
    println!("      the compiler has nothing to object to. Arithmetic is on you.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "The guard is real — it is just on the wrong condition");

    let (w, l) = TEAMS["Lions"];
    println!("  Lions     {w}-{l}  wins == 0, and 0/17 = 0.000 is a TRUE answer");
    let (w, l) = TEAMS["Expansion"];
    println!("  Expansion {w}-{l}   no games played, so there is NO answer to give");
    println!("      The author guarded `wins == 0`, which is unusual but answerable,");
    println!("      and left `wins + losses == 0`, which is undefined, unguarded.");
    println!("      So the one input with no answer is the one that got a number.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn winning_ratio_min(team: &str) -> Result<f32, &'static str> {
    let &(wins, losses) = TEAMS.get(team).ok_or("no such team")?;
    let played = wins + losses;
    if played == 0 {
        return Err("that team has not played yet");
    }
    Ok(wins as f32 / played as f32)
}

fn step3() {
    banner(3, "Move the guard to the undefined case");

    for name in ROSTER {
        println!("  {name:>10} -> {:?}", winning_ratio_min(name));
    }
    println!("      Lions keep their 0.0, because 0 wins in 17 games IS zero.");
    println!("      Expansion now fails, because 0 wins in 0 games is not a number.");
}

// ─────────────────────────────────────────────────────────── Step 4
#[derive(Debug, PartialEq)]
enum RatioError {
    NoSuchTeam(String),
    NoGamesPlayed,
}

impl fmt::Display for RatioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RatioError::NoSuchTeam(name) => write!(f, "{name:?} is not a team we track"),
            RatioError::NoGamesPlayed => write!(f, "the season has not started for that team"),
        }
    }
}

impl std::error::Error for RatioError {}

fn winning_ratio(team: &str) -> Result<f32, RatioError> {
    let &(wins, losses) = TEAMS
        .get(team)
        .ok_or_else(|| RatioError::NoSuchTeam(team.to_string()))?;
    let played = wins + losses;
    if played == 0 {
        return Err(RatioError::NoGamesPlayed);
    }
    Ok(wins as f32 / played as f32)
}

fn step4() {
    banner(4, "Two failures the caller treats differently");

    for name in ["Bears", "Expansion", "Sharks"] {
        match winning_ratio(name) {
            Ok(r) => println!("  {name:>10} -> {r:.3}"),
            Err(e @ RatioError::NoSuchTeam(_)) => {
                println!("  {name:>10} -> typo, ask again: {e}")
            }
            Err(e @ RatioError::NoGamesPlayed) => {
                println!("  {name:>10} -> print a dash, not a 0: {e}")
            }
        }
    }
    println!("      A bad lookup is the caller's mistake; an unplayed season is the");
    println!("      world's. Only a named E lets the caller tell them apart.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "What would have caught it");

    let cases = [("Bears", 9.0 / 17.0), ("Lions", 0.0)];
    for (name, expected) in cases {
        let got = winning_ratio(name).unwrap();
        println!("  {name:>10} expected {expected:.4}  got {got:.4}  {}",
                 if (got - expected).abs() < 1e-6 { "ok" } else { "MISMATCH" });
    }
    println!("  Expansion is_err -> {}", winning_ratio("Expansion").is_err());
    println!("      Not the compiler: every version above type-checks. What catches");
    println!("      an inverted formula is one hand-computed expectation, and what");
    println!("      catches a missing guard is a test at the boundary — 0 games.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    println!();
}
