//! Kata solution: one test per row.
//!
//!   rustc --edition 2024 a_harness_of_your_own_kata.rs -o /tmp/ahooyk && /tmp/ahooyk

use std::panic;

fn price_in_cents(text: &str) -> Option<u32> {
    let (whole, cents) = text.split_once('.')?;
    if cents.len() != 2 {
        return None;
    }
    Some(whole.parse::<u32>().ok()? * 100 + cents.parse::<u32>().ok()?)
}

/// The table. Two rows expect something the function does not do.
const ROWS: [(&str, Option<u32>); 5] = [
    ("2.50", Some(250)),
    ("0.05", Some(5)),
    ("2.5", Some(250)),   // wrong: one decimal is rejected
    ("two", None),
    ("1,000.00", Some(100_000)), // wrong: the comma is not understood
];

/// Before: one #[test] with a loop. It stops at the first row that fails.
fn one_test_with_a_loop() {
    for (input, expected) in ROWS {
        assert_eq!(price_in_cents(input), expected, "row {input:?}");
    }
}

/// After: a trial per row, each with its own name. A closure captures the row,
/// so a trial holds a `Box<dyn Fn()>` rather than a plain `fn()`.
struct Trial {
    name: String,
    run: Box<dyn Fn() + panic::RefUnwindSafe>,
}

fn trials() -> Vec<Trial> {
    ROWS.iter()
        .map(|&(input, expected)| Trial {
            name: format!("price_in_cents::{input}"),
            run: Box::new(move || assert_eq!(price_in_cents(input), expected)),
        })
        .collect()
}

fn first_line(payload: Box<dyn std::any::Any + Send>) -> String {
    let text = payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default();
    text.lines().map(str::trim).collect::<Vec<_>>().join("; ")
}

fn run(filter: Option<&str>) -> i32 {
    let all = trials();
    let mut failed = 0;
    let mut ran = 0;
    for t in all.iter().filter(|t| filter.is_none_or(|f| t.name.contains(f))) {
        ran += 1;
        match panic::catch_unwind(|| (t.run)()) {
            Ok(()) => println!("   test {} ... ok", t.name),
            Err(p) => {
                failed += 1;
                println!("   test {} ... FAILED  {}", t.name, first_line(p));
            }
        }
    }
    println!("   {} passed; {failed} failed; {} filtered out", ran - failed, all.len() - ran);
    if failed == 0 { 0 } else { 101 }
}

fn main() {
    let quiet = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    println!("1. One #[test] with a loop over the table");
    match panic::catch_unwind(one_test_with_a_loop) {
        Ok(()) => println!("   ok"),
        Err(p) => println!("   FAILED  {}", first_line(p)),
    }
    println!("   One failure reported. The comma row is broken too, and nothing says so");
    println!("   until the first one is fixed.");

    println!();
    println!("2. A trial per row");
    let status = run(None);
    println!("   exit status {status}; both broken rows named, every row run");

    println!();
    println!("3. The filter now selects rows: -- 2.5");
    let status = run(Some("2.5"));
    println!("   exit status {status}; \"2.5\" is a substring of \"2.50\" too, so two rows ran");

    panic::set_hook(quiet);
}
