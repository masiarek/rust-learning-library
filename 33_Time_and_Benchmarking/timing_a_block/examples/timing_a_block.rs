//! Timing a block: `Instant::now()` before, `elapsed()` after -- and the only
//! verdicts about the result that are safe to print.
//!
//! A measured duration is different on every run, so it is never printed here.
//! What is printed is what `std::thread::sleep` promises: it never sleeps less
//! than it was asked to.
//!
//!   rustc --edition 2024 timing_a_block.rs -o /tmp/timing_a_block && /tmp/timing_a_block

use std::thread;
use std::time::{Duration, Instant};

/// Run `f`; hand back what it returned and how long it took.
fn timed<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let value = f();
    (value, start.elapsed())
}

fn main() {
    println!("1. The whole API is two calls");
    let nap = Duration::from_millis(20);
    let start = Instant::now();
    thread::sleep(nap);
    let took = start.elapsed();
    println!("   slept 20ms; took >= 20ms        {}", took >= nap);

    println!();
    println!("2. Hand the Duration back; let the caller decide what to do with it");
    let (total, _) = timed(|| (1..=1_000_000u64).sum::<u64>());
    println!("   the value comes back too        {total}");
    let ((), took) = timed(|| thread::sleep(nap));
    println!("   a 20ms sleep, timed through it  took >= 20ms: {}", took >= nap);
}
