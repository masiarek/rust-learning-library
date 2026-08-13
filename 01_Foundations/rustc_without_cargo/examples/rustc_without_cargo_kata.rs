//! Kata solution: one file, three builds.
//!
//!   rustc --edition 2024 rustc_without_cargo_kata.rs -o /tmp/rwck && /tmp/rwck
//!   rustc --edition 2024 --test rustc_without_cargo_kata.rs -o /tmp/rwck_t && /tmp/rwck_t
//!   rustc --edition 2024 -O rustc_without_cargo_kata.rs -o /tmp/rwck_o && /tmp/rwck_o
//!
//! Same source, three different programs: your `main`, a test harness that
//! never calls `main` at all, and an optimized build where one of the answers
//! below changes. No Cargo.toml anywhere.

/// The scoring round of a STAR count, for one candidate.
fn total(scores: &[u8]) -> u32 {
    scores.iter().map(|s| u32::from(*s)).sum()
}

/// A ballot may leave a candidate unscored; a blank is not a zero-score.
fn mean(scores: &[u8]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }
    Some(f64::from(total(scores)) / scores.len() as f64)
}

fn main() {
    let ballots: [u8; 4] = [5, 3, 0, 4];

    println!("total     = {}", total(&ballots));
    println!("mean      = {:?}", mean(&ballots));
    println!("mean([])  = {:?}", mean(&[]));

    println!("\nBuilt with debug_assertions = {}", cfg!(debug_assertions));
    println!("  Plain `rustc` says true, `rustc -O` says false. That is the");
    println!("  same split as `cargo run` versus `cargo run --release`, and it");
    println!("  decides whether an arithmetic overflow panics or wraps.");

    println!("\nThe tests below did not run, and were not even compiled:");
    println!("  #[cfg(test)] is false in this build, so the module does not");
    println!("  exist in this binary. `rustc --test` builds the harness as the");
    println!("  entry point instead — main is the thing that goes unused then.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_a_ballot() {
        assert_eq!(total(&[5, 3, 0, 4]), 12);
    }

    #[test]
    fn empty_has_no_mean() {
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn mean_is_the_total_over_the_count() {
        assert_eq!(mean(&[5, 3, 0, 4]), Some(3.0));
    }
}
