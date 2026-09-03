//! Kata solution: which fields may legitimately be missing?
//!
//!   rustc --edition 2024 option_fields_kata.rs -o /tmp/ofk && /tmp/ofk

#[derive(Debug, Default)]
struct Respondent {
    /// Required. A person with no name is not a person with an empty name.
    name: String,
    /// Optional, and the type says so: plenty of people have no middle name.
    middle_name: Option<String>,
    /// NOT an Option. "Belongs to no account" is not a state that exists here;
    /// making it optional would push a check into every reader for nothing.
    account_id: u32,
    /// Optional for a real reason: the form may not be back yet. An empty Vec
    /// would say "returned, and rated nothing", which is a different fact.
    ratings: Option<Vec<u8>>,
}

fn describe(r: &Respondent) {
    let full = match &r.middle_name {
        Some(m) => format!("{} {} ", r.name, m),
        None => format!("{} ", r.name),
    };
    // Two different questions, and the type makes you ask them separately.
    let form = match &r.ratings {
        None => "no form returned".to_string(),
        Some(s) if s.is_empty() => "form returned, nothing rated".to_string(),
        Some(s) => format!("form returned, {} ratings, total {}", s.len(), s.iter().map(|n| *n as u32).sum::<u32>()),
    };
    println!("  {full:<18} account {:<4} {form}", r.account_id);
}

fn main() {
    let people = [
        Respondent { name: "Ada".into(), middle_name: Some("Q".into()), account_id: 7, ratings: Some(vec![5, 2, 0]) },
        Respondent { name: "Ben".into(), middle_name: None, account_id: 7, ratings: Some(vec![]) },
        Respondent { name: "Cara".into(), middle_name: None, account_id: 12, ratings: None },
    ];

    println!("Three respondents, and the two absences the type keeps apart:");
    for r in &people {
        describe(r);
    }

    let returned = people.iter().filter(|r| r.ratings.is_some()).count();
    println!("\n  forms returned: {returned} of {}", people.len());
    println!("      A Vec<u8> field with no Option could not have told you that.");

    println!("\nDefault gives None for every optional field, for free:");
    println!("  {:?}", Respondent::default());
}
