//! Kata solution: an average that has an answer for every input.
//!
//!   rustc --edition 2024 partial_functions_kata.rs -o /tmp/pfk && /tmp/pfk

/// The partial version: undefined on an empty slice, and it does not say so.
fn mean_naive(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

/// The total version: "no answer" is now one of the answers.
fn mean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        None
    } else {
        Some(xs.iter().sum::<f64>() / xs.len() as f64)
    }
}

fn report(label: &str, xs: &[f64]) {
    match mean(xs) {
        Some(m) => println!("  {label:<14} -> average score {m}"),
        None => println!("  {label:<14} -> nobody scored this candidate"),
    }
}

fn main() {
    let scored = [5.0, 3.0, 4.0];
    let unscored: [f64; 0] = [];

    println!("The naive version, on the input it has no answer for:");
    println!("  mean_naive(scored)   -> {}", mean_naive(&scored));
    println!("  mean_naive(unscored) -> {}", mean_naive(&unscored));
    println!("      0.0 / 0.0 is NaN. No panic, no warning — and NaN is not even");
    println!("      equal to itself, so a later == or sort quietly misbehaves.");

    println!("\nThe total version says so instead:");
    report("scored", &scored);
    report("unscored", &unscored);

    println!("\nWhat None means here is a decision, so write it down:");
    println!("  mean(unscored) is None because there is no such average —");
    println!("  NOT because the average is 0, which is a score a voter can give.");
    println!("  mean(&[0.0, 0.0]) -> {:?}", mean(&[0.0, 0.0]));
}
