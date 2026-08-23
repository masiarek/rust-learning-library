//! Kata solution: the f-string that isn't.
//!
//! A report written the way a Python programmer would write it, then made to
//! compile. Four braces, four different refusals, and three fixes to choose
//! between — a `let` above the line, a named argument, or a positional one.
//! The last part is the judgement call: only one of the three scales.
//!
//!   rustc --edition 2024 braces_take_a_name_kata.rs -o /tmp/btank && /tmp/btank

fn banner(title: &str) {
    println!("\n──── {title}");
}

struct Ballot {
    voter: &'static str,
    scores: [u32; 3],
}

impl Ballot {
    fn total(&self) -> u32 {
        self.scores.iter().sum()
    }
}

fn main() {
    let ballot = Ballot { voter: "Ada", scores: [5, 3, 4] };

    banner("What a Python programmer writes, and what each brace gets back");

    println!("  println!(\"{{ballot.voter}}\");");
    println!("      error: invalid format string: field access isn't supported");
    println!("      help: consider using a positional formatting argument instead");
    println!();
    println!("  println!(\"{{ballot.scores.len()}} candidates\");");
    println!("      error: invalid format string: expected `}}`, found `.`");
    println!();
    println!("  println!(\"first: {{ballot.scores[0]}}\");");
    println!("      error: invalid format string: expected `}}`, found `.`");
    println!();
    println!("  println!(\"mean: {{ballot.total() / 3}}\");");
    println!("      error: invalid format string: expected `}}`, found `.`");
    println!();
    println!("  Only the first one is diagnosed as what it is. The other three");
    println!("  stop at the first character that cannot continue an identifier,");
    println!("  so the message names the punctuation and not your intent.");

    banner("Fix A: a `let` above the line, then capture the name");

    let voter = ballot.voter;
    let candidates = ballot.scores.len();
    let first = ballot.scores[0];
    let mean = ballot.total() / candidates as u32;
    println!("  {voter}: {candidates} candidates, first {first}, mean {mean}");

    banner("Fix B: a named argument — the name lives in the call");

    println!(
        "  {voter}: {candidates} candidates, first {first}, mean {mean}",
        voter = ballot.voter,
        candidates = ballot.scores.len(),
        first = ballot.scores[0],
        mean = ballot.total() / 3,
    );

    banner("Fix C: positional — no names at all");

    println!(
        "  {}: {} candidates, first {}, mean {}",
        ballot.voter,
        ballot.scores.len(),
        ballot.scores[0],
        ballot.total() / 3,
    );

    banner("Which one to ship");

    println!("  All three print the same line. They are not equally readable.");
    println!();
    println!("  C is the one that rots. With four holes you are already counting");
    println!("  arguments against braces by eye, and inserting a fifth in the");
    println!("  middle silently renumbers everything after it.");
    println!();
    println!("  B keeps the names but spends four lines inventing them at the");
    println!("  call site, where they exist only until the semicolon.");
    println!();
    println!("  A is the default. The names outlive the print, so the next line");
    println!("  can reuse them; the types are visible to the reader and to the");
    println!("  compiler's error messages; and the format string reads as the");
    println!("  sentence it is going to become. Reach for C only when there is");
    println!("  exactly one hole and the expression is short.");

    banner("The one C is genuinely for");

    println!("  {}", ballot.total()); //  one hole, one expression, no ceremony
    println!("  ...and inside a method, where `{{self.voter}}` is refused, a");
    println!("  positional argument is often lighter than binding every field.");
}
