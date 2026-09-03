//! Kata solution: two ways to store one record, and the bug each one allows.
//!
//!   rustc --edition 2024 representing_a_record_kata.rs -o /tmp/rrk && /tmp/rrk

/// Shape 1: two parallel Vecs. Nothing ties a column name to a value.
struct Parallel {
    questions: Vec<&'static str>,
    ratings: Vec<u8>,
}

impl Parallel {
    fn rating_for(&self, which: &str) -> Option<u8> {
        let i = self.questions.iter().position(|q| *q == which)?;
        self.ratings.get(i).copied()
    }
}

/// Shape 2: one row per question. The pairing is the type.
#[derive(Debug)]
struct Entry {
    question: &'static str,
    rating: u8,
}

fn rating_for(record: &[Entry], which: &str) -> Option<u8> {
    record.iter().find(|e| e.question == which).map(|e| e.rating)
}

fn main() {
    let mut p = Parallel {
        questions: vec!["speed", "price", "support"],
        ratings: vec![5, 3, 0],
    };
    let rows = vec![
        Entry { question: "speed", rating: 5 },
        Entry { question: "price", rating: 3 },
        Entry { question: "support", rating: 0 },
    ];

    println!("Both hold the same record:");
    println!("  parallel: price -> {:?}", p.rating_for("price"));
    println!("  rows:     price -> {:?}", rating_for(&rows, "price"));

    println!("\nNow a question is dropped from the form, and one line gets forgotten:");
    p.questions.remove(1); // and the matching ratings.remove(1) never happens
    println!("  parallel: questions {:?} ratings {:?}", p.questions, p.ratings);
    println!("  parallel: support -> {:?}   <- WRONG, that is price's 3", p.rating_for("support"));
    println!("      The two Vecs disagree and nothing noticed. This code compiles,");
    println!("      runs, and reports a plausible number.");

    println!("\nThe same mistake in the row shape:");
    let mut rows = rows;
    rows.remove(1);
    println!("  rows: {:?}", rows.iter().map(|e| (e.question, e.rating)).collect::<Vec<_>>());
    println!("  rows: support -> {:?}   <- still right", rating_for(&rows, "support"));
    println!("      There was no second line to forget. The desync is not a bug");
    println!("      you avoided by being careful; it is a bug you cannot write.");

    println!("\nWhat the row shape costs, honestly:");
    println!("  lookup is a scan, not an index — fine for 5 questions, wrong for");
    println!("  50,000 rows in a hot loop, which is where the flat matrix and");
    println!("  the index-newtype shapes start to earn their extra ceremony.");
}
