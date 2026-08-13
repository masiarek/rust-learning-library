//! Kata solution: the guard is real, and it is on the wrong condition.
//!
//!   rustc --edition 2024 wrong_guard_kata.rs -o /tmp/wgk && /tmp/wgk

#[derive(Debug)]
enum RateError {
    NoGames,
}

/// The version that looks careful. It returns Result, it has a guard, and the
/// guard is on the input that always has an answer.
fn win_rate_guarded_wrongly(wins: u32, games: u32) -> Result<f64, RateError> {
    if wins == 0 {
        // "No wins? Bail out." — but 0 wins in 10 games is 0.0, a real answer.
        return Err(RateError::NoGames);
    }
    Ok(wins as f64 / games as f64)
}

/// The fix, at the small size: guard the input that genuinely has no answer.
fn win_rate(wins: u32, games: u32) -> Result<f64, RateError> {
    if games == 0 {
        return Err(RateError::NoGames);
    }
    Ok(wins as f64 / games as f64)
}

/// The fix at the larger size: make the unanswerable input unrepresentable.
struct Season {
    wins: u32,
    games: u32, // invariant: > 0
}

impl Season {
    fn new(wins: u32, games: u32) -> Option<Season> {
        (games > 0 && wins <= games).then_some(Season { wins, games })
    }

    /// No Result at all: a Season that exists has a rate.
    fn win_rate(&self) -> f64 {
        self.wins as f64 / self.games as f64
    }
}

fn main() {
    let cases = [(7u32, 10u32), (0, 10), (0, 0)];

    println!("Guarded on the wrong condition:");
    for (w, g) in cases {
        println!("  {w} wins in {g:>2} games -> {:?}", win_rate_guarded_wrongly(w, g));
    }
    println!("      Line 2 rejects a real answer (0 wins in 10 games IS 0.0), and");
    println!("      line 3 — the one input with no answer — never reaches the guard");
    println!("      at all, because 0 wins is caught first. It would have divided");
    println!("      by zero and left through Ok as a plausible-looking number.");

    println!("\nGuarded on the right one:");
    for (w, g) in cases {
        println!("  {w} wins in {g:>2} games -> {:?}", win_rate(w, g));
    }

    println!("\nAnd what the wrong guard would have returned, unguarded:");
    println!("  0 as f64 / 0 as f64 = {}", 0_f64 / 0_f64);
    println!("      NaN. No panic — it sorts oddly, compares false against itself,");
    println!("      and prints in a report as if it were a measurement.");

    println!("\nThe fix that removes the question:");
    println!("  Season::new(0, 0)  -> {:?}", Season::new(0, 0).is_some());
    println!("  Season::new(7, 10) -> rate {:.2}", Season::new(7, 10).unwrap().win_rate());
    println!("      `win_rate` here returns f64, not Result. The unanswerable input");
    println!("      was refused at the door, so there is no failure left to report.");
}
