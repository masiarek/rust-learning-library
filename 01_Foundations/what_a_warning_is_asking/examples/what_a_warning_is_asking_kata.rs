//! Kata solution: four warnings, four different right answers.
//!
//! The point of the exercise is that "add an underscore" is the correct fix
//! for exactly one of these. One was a real bug the compiler found, one is a
//! deliberate discard, one would BREAK if you used a bare `_`, and one wants
//! deleting rather than silencing.

const NAMES: [&str; 4] = ["Ada", "Ben", "Cara", "Dev"];

/// Closes an audit entry when it is dropped — so *when* it drops is visible.
struct AuditEntry(&'static str);

impl Drop for AuditEntry {
    fn drop(&mut self) {
        println!("  [audit] closed: {}", self.0);
    }
}

impl AuditEntry {
    fn open(what: &'static str) -> Self {
        println!("  [audit] opened: {what}");
        AuditEntry(what)
    }
}

/// Split a column of approval marks into (approvals, abstentions).
fn split(marks: &[Option<bool>]) -> (u32, u32) {
    let mut approvals = 0;
    let mut abstentions = 0;
    for m in marks {
        match m {
            Some(true) => approvals += 1,
            Some(false) => {}
            None => abstentions += 1,
        }
    }
    (approvals, abstentions)
}

// Warning 4 was `fn old_margin(...)`, a helper nothing calls any more.
// The fix is deletion, not `#[allow(dead_code)]`: an allow would preserve it
// for ever, and the next reader would have to work out whether it still
// matters. Version control already remembers it. It is simply gone.

fn main() {
    // Four voters, four candidates. `None` is a blank — nobody marked it.
    let ballots: [[Option<bool>; 4]; 4] = [
        [Some(true), Some(false), Some(true), None],
        [Some(true), Some(true), Some(false), Some(false)],
        [None, Some(true), Some(true), Some(false)],
        [Some(false), Some(false), Some(true), Some(true)],
    ];

    // Warning 3: an audit entry must OUTLIVE the work it records.
    // `let _ = AuditEntry::open(..)` compiles, silences the warning, and
    // closes the entry before the tally even starts. The name is load-bearing.
    let _audit = AuditEntry::open("approval tally");

    let mut totals = [0u32; 4];
    for seat in 0..4 {
        let column: Vec<Option<bool>> = ballots.iter().map(|b| b[seat]).collect();

        // Warning 2: this tally does not report abstentions, and that is a
        // decision, not an oversight. `_abstentions` says so out loud.
        let (approvals, _abstentions) = split(&column);
        totals[seat] = approvals;
    }

    for (name, total) in NAMES.iter().zip(totals.iter()) {
        println!("  {name:<5} {total}");
    }

    // Warning 1: this was the real bug. `winner` was computed and never read,
    // so the program did all the work and announced nothing. The compiler was
    // not complaining about style; it had found the missing line.
    let winner = NAMES
        .iter()
        .zip(totals.iter())
        .max_by_key(|(_, total)| **total)
        .map(|(name, _)| *name)
        .unwrap_or("nobody");

    println!("  winner: {winner}");
}
