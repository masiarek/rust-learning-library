//! Kata solution: read what the panic left behind, then make the failure a value.
//!
//! A batch job records five ballot rows and the third one never arrived. Part 1
//! unwraps, so you can see exactly how much work is done, what never runs, and
//! what still gets cleaned up. Parts 2 and 3 hand the same fact to the caller as
//! a return value — first as a failure that names the row, then as a result that
//! keeps the four answerable rows instead of spending them.
//!
//!   rustc --edition 2024 what_a_panic_costs_kata.rs -o /tmp/wpck && /tmp/wpck

use std::panic::{self, AssertUnwindSafe};
use std::sync::Mutex;

/// The batch. Row 2 (zero-based) is missing.
const ROWS: [Option<u32>; 5] = [Some(5), Some(3), None, Some(4), Some(2)];

/// Prints when dropped, so the unwind is visible rather than asserted.
struct Ledger;

impl Drop for Ledger {
    fn drop(&mut self) {
        println!("      Ledger closed — Drop ran, even on the way out");
    }
}

// ─────────────────────────────────────────────── Part 1: the unwrapping version
fn total_by_unwrap(log: &Mutex<Vec<u32>>) -> u32 {
    let _ledger = Ledger;
    let mut total = 0;
    for row in ROWS {
        let score = row.unwrap(); // row 2 stops the program here
        total += score;
        log.lock().expect("single-threaded here").push(score);
    }
    total
}

// ───────────────────────────────────── Part 2: the absence as a return value
/// `?` on the `Option` would hand the caller a bare `None`; which row was missing
/// is the useful half, so this is a `Result` carrying the index.
fn total_by_result() -> Result<u32, usize> {
    let mut total = 0;
    for (i, row) in ROWS.iter().enumerate() {
        match row {
            Some(score) => total += score,
            None => return Err(i),
        }
    }
    Ok(total)
}

// ──────────────────────────── Part 3: what the caller usually actually wants
/// One missing row need not cost the other four. Total what is answerable, and
/// report the gaps as data rather than as a stopping condition.
fn total_reporting_gaps() -> (u32, Vec<usize>) {
    let mut total = 0;
    let mut missing = Vec::new();
    for (i, row) in ROWS.iter().enumerate() {
        match row {
            Some(score) => total += score,
            None => missing.push(i),
        }
    }
    (total, missing)
}

fn main() {
    println!("The same batch, three ways of meeting the missing row\n");

    // ── Part 1
    println!("Part 1 — unwrap: the panic, and what it leaves behind");
    let log = Mutex::new(Vec::new());
    let prior = panic::take_hook();
    panic::set_hook(Box::new(|_| {})); // keep the demo's output readable
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| total_by_unwrap(&log)));
    panic::set_hook(prior);

    let recorded = log.lock().expect("single-threaded here").clone();
    match outcome {
        Ok(total) => println!("  returned {total}"),
        Err(_) => println!("  panicked on row 2 — no total was returned at all"),
    }
    println!("  rows recorded before it fired: {recorded:?}");
    println!("      Two rows are in the ledger, rows 3 and 4 were never looked at,");
    println!("      and the caller got no answer — not even a wrong one. What DID");
    println!("      happen is the cleanup: the Ledger's Drop ran on the way out.");

    // ── Part 2
    println!("\nPart 2 — the same fact, as a return value");
    match total_by_result() {
        Ok(total) => println!("  Ok({total})"),
        Err(row) => println!("  Err({row}) — row {row} is missing, and the caller decides"),
    }
    println!("      Nothing was recorded, nothing was half-done, and the caller can");
    println!("      retry, ask for that row, or give up. Same information the panic");
    println!("      had; delivered as a value instead of as an exit.");

    // ── Part 3
    println!("\nPart 3 — keep the rows that are answerable");
    let (total, missing) = total_reporting_gaps();
    println!("  total {total} over the answerable rows, missing {missing:?}");
    println!("      Four rows out of five is usually the right answer, and the gap");
    println!("      is now in the result where a reader can see it. The panic threw");
    println!("      away three rows to report the same one missing.");
}
