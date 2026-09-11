//! An `Instant` is not a `SystemTime`: which subtractions compile, and what
//! each one hands back when the right-hand side is the later one.
//!
//! Every time here is built from a fixed point plus a `Duration`, so the run
//! is the same on every machine: no reading of the real clock is printed.
//!
//!   rustc --edition 2024 an_instant_is_not_a_system_time.rs -o /tmp/ain && /tmp/ain

use std::time::{Duration, Instant, UNIX_EPOCH};

fn main() {
    println!("1. SystemTime: duration_since is a question with two answers");
    let a = UNIX_EPOCH + Duration::from_secs(100);
    let b = UNIX_EPOCH + Duration::from_secs(160);
    println!("   b.duration_since(a)   = {:?}", b.duration_since(a));
    println!("   a.duration_since(b)   = {:?}", a.duration_since(b));
    let err = a.duration_since(b).unwrap_err();
    println!("   err.duration()        = {:?}", err.duration());
    println!("   err, displayed        = {err}");

    println!();
    println!("2. Instant: `-` compiles, and never goes below zero");
    let t0 = Instant::now();
    let t1 = t0 + Duration::from_secs(2);
    println!("   t1 - t0                        = {:?}", t1 - t0);
    println!("   t0 - t1                        = {:?}", t0 - t1);
    println!("   t0.duration_since(t1)          = {:?}", t0.duration_since(t1));
    println!("   t0.checked_duration_since(t1)  = {:?}", t0.checked_duration_since(t1));
    println!("   t1.checked_duration_since(t0)  = {:?}", t1.checked_duration_since(t0));
}
