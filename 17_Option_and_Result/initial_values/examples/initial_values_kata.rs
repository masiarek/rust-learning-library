//! Kata solution: a value that is not ready yet is not the same as a missing one.
//!
//!   rustc --edition 2024 initial_values_kata.rs -o /tmp/ivk && /tmp/ivk

/// Declared, not initialized. Every path assigns before the read, and the
/// compiler checks that — no `Option`, no `mut`, no unreachable arm.
fn sample_size_for(rows: usize) -> usize {
    let sample: usize;
    if rows == 0 {
        sample = 0;
    } else if rows < 10 {
        sample = rows; // a tiny table: read all of it
    } else {
        sample = rows / 2 + 1;
    }
    sample
}

/// The `Option` version of the same job, for contrast: one more state to read,
/// one more way to be wrong, and an arm that cannot happen.
///
/// The allow is part of the lesson — without it the compiler warns that the
/// `None` is "never read", which is it telling you the initial value was dead
/// on arrival.
#[allow(unused_assignments)]
fn sample_size_awkward(rows: usize) -> usize {
    let mut sample: Option<usize> = None;
    if rows == 0 {
        sample = Some(0);
    } else if rows < 10 {
        sample = Some(rows);
    } else {
        sample = Some(rows / 2 + 1);
    }
    sample.expect("every branch above assigns — but nothing checks that claim")
}

/// Where `Option` is genuinely right: the value may never arrive at all.
struct Job {
    name: &'static str,
    /// None until the job finishes. Not "not ready yet" — "may never happen".
    finished_at: Option<&'static str>,
}

fn main() {
    println!("Declared without a value, proved assigned before use:");
    for rows in [0, 7, 461] {
        println!("  sample_size_for({rows:>3}) -> {}", sample_size_for(rows));
    }

    println!("\nThe Option version returns the same numbers…");
    for rows in [0, 7, 461] {
        println!("  sample_size_awkward({rows:>3}) -> {}", sample_size_awkward(rows));
    }
    println!("      …and pays for it with a state that cannot occur, an `expect`");
    println!("      whose claim nothing verifies, and a `mut` it did not need. The");
    println!("      compiler even warns that the initial None is never read — the");
    println!("      source has to #[allow] it to compile quietly.");

    println!("\nWhere Option earns its place — absence that outlives the function:");
    let running = Job { name: "nightly-reindex", finished_at: None };
    let done = Job { name: "nightly-backup", finished_at: Some("03:12") };
    for j in [&running, &done] {
        match j.finished_at {
            Some(at) => println!("  {:<18} finished at {at}", j.name),
            None => println!("  {:<18} still running", j.name),
        }
    }
}
