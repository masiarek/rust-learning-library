//! Kata solution: which fields may legitimately be missing?
//!
//!   rustc --edition 2024 option_fields_kata.rs -o /tmp/ofk && /tmp/ofk

#[derive(Debug, Default)]
struct Voter {
    /// Required. A voter with no name is not a voter with an empty name.
    name: String,
    /// Optional, and the type says so: plenty of people have no middle name.
    middle_name: Option<String>,
    /// NOT an Option. "Registered in no precinct" is not a state that exists;
    /// making it optional would push a check into every reader for nothing.
    precinct: u32,
    /// Optional for a real reason: the ballot may not be back yet. An empty Vec
    /// would say "returned, and scored nobody", which is a different fact.
    scores: Option<Vec<u8>>,
}

fn describe(v: &Voter) {
    let full = match &v.middle_name {
        Some(m) => format!("{} {} ", v.name, m),
        None => format!("{} ", v.name),
    };
    // Two different questions, and the type makes you ask them separately.
    let ballot = match &v.scores {
        None => "no ballot returned".to_string(),
        Some(s) if s.is_empty() => "ballot returned, nobody scored".to_string(),
        Some(s) => format!("ballot returned, {} scores, total {}", s.len(), s.iter().map(|n| *n as u32).sum::<u32>()),
    };
    println!("  {full:<18} precinct {:<4} {ballot}", v.precinct);
}

fn main() {
    let voters = [
        Voter { name: "Ada".into(), middle_name: Some("Q".into()), precinct: 7, scores: Some(vec![5, 2, 0]) },
        Voter { name: "Ben".into(), middle_name: None, precinct: 7, scores: Some(vec![]) },
        Voter { name: "Cara".into(), middle_name: None, precinct: 12, scores: None },
    ];

    println!("Three voters, and the two absences the type keeps apart:");
    for v in &voters {
        describe(v);
    }

    let returned = voters.iter().filter(|v| v.scores.is_some()).count();
    println!("\n  ballots returned: {returned} of {}", voters.len());
    println!("      A Vec<u8> field with no Option could not have told you that.");

    println!("\nDefault gives None for every optional field, for free:");
    println!("  {:?}", Voter::default());
}
