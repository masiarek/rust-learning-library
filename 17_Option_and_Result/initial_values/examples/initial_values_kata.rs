//! Kata solution: a value that is not ready yet is not the same as a missing one.
//!
//!   rustc --edition 2024 initial_values_kata.rs -o /tmp/ivk && /tmp/ivk

/// Declared, not initialized. Every path assigns before the read, and the
/// compiler checks that — no `Option`, no `mut`, no unreachable arm.
fn quorum_for(voters: usize) -> usize {
    let quorum: usize;
    if voters == 0 {
        quorum = 0;
    } else if voters < 10 {
        quorum = voters; // a tiny board needs everyone
    } else {
        quorum = voters / 2 + 1;
    }
    quorum
}

/// The `Option` version of the same job, for contrast: one more state to read,
/// one more way to be wrong, and an arm that cannot happen.
///
/// The allow is part of the lesson — without it the compiler warns that the
/// `None` is "never read", which is it telling you the initial value was dead
/// on arrival.
#[allow(unused_assignments)]
fn quorum_for_awkward(voters: usize) -> usize {
    let mut quorum: Option<usize> = None;
    if voters == 0 {
        quorum = Some(0);
    } else if voters < 10 {
        quorum = Some(voters);
    } else {
        quorum = Some(voters / 2 + 1);
    }
    quorum.expect("every branch above assigns — but nothing checks that claim")
}

/// Where `Option` is genuinely right: the value may never arrive at all.
struct Election {
    name: &'static str,
    /// None until the count finishes. Not "not ready yet" — "may never happen".
    certified_winner: Option<&'static str>,
}

fn main() {
    println!("Declared without a value, proved assigned before use:");
    for voters in [0, 7, 461] {
        println!("  quorum_for({voters:>3}) -> {}", quorum_for(voters));
    }

    println!("\nThe Option version returns the same numbers…");
    for voters in [0, 7, 461] {
        println!("  quorum_for_awkward({voters:>3}) -> {}", quorum_for_awkward(voters));
    }
    println!("      …and pays for it with a state that cannot occur, an `expect`");
    println!("      whose claim nothing verifies, and a `mut` it did not need. The");
    println!("      compiler even warns that the initial None is never read — the");
    println!("      source has to #[allow] it to compile quietly.");

    println!("\nWhere Option earns its place — absence that outlives the function:");
    let running = Election { name: "Springfield 2026", certified_winner: None };
    let done = Election { name: "Shelbyville 2026", certified_winner: Some("Ada") };
    for e in [&running, &done] {
        match e.certified_winner {
            Some(w) => println!("  {:<18} certified: {w}", e.name),
            None => println!("  {:<18} not certified yet", e.name),
        }
    }
}
