//! Kata solution: fall back without forgetting why you had to.
//!
//!   rustc --edition 2024 unwrap_or_else_kata.rs -o /tmp/uoek && /tmp/uoek

/// A tally that must produce a number for the report no matter what, but must
/// not pretend the bad row never happened.
fn counted(raw: &str, notes: &mut Vec<String>) -> u32 {
    raw.trim().parse::<u32>().unwrap_or_else(|e| {
        // On a Result, the closure is HANDED the error. This is the thing
        // `unwrap_or` and `expect` cannot do.
        notes.push(format!("row {raw:?} counted as 0 — {e}"));
        0
    })
}

fn main() {
    let rows = ["12", "7", "twelve", "", "3"];
    let mut notes = Vec::new();

    let total: u32 = rows.iter().map(|r| counted(r, &mut notes)).sum();

    println!("Total across {} rows -> {total}", rows.len());
    println!("\nAnd the report can still say what it did with the rest:");
    for n in &notes {
        println!("  {n}");
    }

    println!("\nThe same fallback written three ways, and what each forgets:");
    let bad = "twelve".parse::<u32>();
    println!("  unwrap_or(0)            -> {}   (the reason is gone)", bad.clone().unwrap_or(0));
    println!("  unwrap_or_default()     -> {}   (the reason is gone)", bad.clone().unwrap_or_default());
    println!(
        "  unwrap_or_else(|e| …)   -> {}   (kept: {})",
        bad.clone().unwrap_or_else(|_| 0),
        bad.unwrap_err()
    );
}
