//! Kata solution: two ways to store one ballot, and the bug each one allows.
//!
//!   rustc --edition 2024 representing_a_ballot_kata.rs -o /tmp/rbk && /tmp/rbk

/// Shape 1: two parallel Vecs. Nothing ties a name to a score.
struct Parallel {
    candidates: Vec<&'static str>,
    scores: Vec<u8>,
}

impl Parallel {
    fn score_for(&self, who: &str) -> Option<u8> {
        let i = self.candidates.iter().position(|c| *c == who)?;
        self.scores.get(i).copied()
    }
}

/// Shape 2: one row per candidate. The pairing is the type.
#[derive(Debug)]
struct Entry {
    candidate: &'static str,
    score: u8,
}

fn score_for(ballot: &[Entry], who: &str) -> Option<u8> {
    ballot.iter().find(|e| e.candidate == who).map(|e| e.score)
}

fn main() {
    let mut p = Parallel {
        candidates: vec!["Ada", "Ben", "Cara"],
        scores: vec![5, 3, 0],
    };
    let rows = vec![
        Entry { candidate: "Ada", score: 5 },
        Entry { candidate: "Ben", score: 3 },
        Entry { candidate: "Cara", score: 0 },
    ];

    println!("Both hold the same ballot:");
    println!("  parallel: Ben -> {:?}", p.score_for("Ben"));
    println!("  rows:     Ben -> {:?}", score_for(&rows, "Ben"));

    println!("\nNow a candidate withdraws, and one line gets forgotten:");
    p.candidates.remove(1); // and the matching scores.remove(1) never happens
    println!("  parallel: candidates {:?} scores {:?}", p.candidates, p.scores);
    println!("  parallel: Cara -> {:?}   <- WRONG, that is Ben's 3", p.score_for("Cara"));
    println!("      The two Vecs disagree and nothing noticed. This code compiles,");
    println!("      runs, and reports a plausible number.");

    println!("\nThe same mistake in the row shape:");
    let mut rows = rows;
    rows.remove(1);
    println!("  rows: {:?}", rows.iter().map(|e| (e.candidate, e.score)).collect::<Vec<_>>());
    println!("  rows: Cara -> {:?}   <- still right", score_for(&rows, "Cara"));
    println!("      There was no second line to forget. The desync is not a bug");
    println!("      you avoided by being careful; it is a bug you cannot write.");

    println!("\nWhat the row shape costs, honestly:");
    println!("  lookup is a scan, not an index — fine for 5 candidates, wrong for");
    println!("  50,000 ballots in a hot loop, which is where the flat matrix and");
    println!("  the index-newtype shapes start to earn their extra ceremony.");
}
