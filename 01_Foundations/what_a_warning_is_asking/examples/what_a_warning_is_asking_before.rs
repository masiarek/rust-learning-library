//! Kata starting point: an approval tally that compiles, runs, exits 0 —
//! and is wrong. Four warnings say so. Each one wants a different answer.
//!
//! Compile it and read the four warnings before you change anything:
//!
//!   rustc --edition 2024 what_a_warning_is_asking_before.rs -o /tmp/before

const NAMES: [&str; 4] = ["Ada", "Ben", "Cara", "Dev"];

/// Closes an audit entry when it is dropped.
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

/// The margin between the top two — from an earlier version of this program.
fn old_margin(totals: &[u32; 4]) -> u32 {
    let mut sorted = *totals;
    sorted.sort_unstable();
    sorted[3] - sorted[2]
}

fn main() {
    let ballots: [[Option<bool>; 4]; 4] = [
        [Some(true), Some(false), Some(true), None],
        [Some(true), Some(true), Some(false), Some(false)],
        [None, Some(true), Some(true), Some(false)],
        [Some(false), Some(false), Some(true), Some(true)],
    ];

    let audit = AuditEntry::open("approval tally");

    let mut totals = [0u32; 4];
    for seat in 0..4 {
        let column: Vec<Option<bool>> = ballots.iter().map(|b| b[seat]).collect();
        let (approvals, abstentions) = split(&column);
        totals[seat] = approvals;
    }

    for (name, total) in NAMES.iter().zip(totals.iter()) {
        println!("  {name:<5} {total}");
    }

    let winner = NAMES
        .iter()
        .zip(totals.iter())
        .max_by_key(|(_, total)| **total)
        .map(|(name, _)| *name)
        .unwrap_or("nobody");
}
