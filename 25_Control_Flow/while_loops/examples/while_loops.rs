//! `while`: a `bool` checked before every pass, a `mut` the body has to move,
//! and a value of `()` when it is done.
//!
//!   rustc --edition 2024 while_loops.rs -o /tmp/wl && /tmp/wl

use std::time::{Duration, Instant};

fn count_down_while(start: i32) -> (u32, i32) {
    let mut n = start;
    let mut passes = 0;
    while n > 0 {
        n -= 1;
        passes += 1;
    }
    (passes, n)
}

// Rust has no `do ... while`. The body goes first, and the condition —
// negated — becomes the way out at the bottom.
fn count_down_do_while(start: i32) -> (u32, i32) {
    let mut n = start;
    let mut passes = 0;
    loop {
        n -= 1;
        passes += 1;
        if n <= 0 {
            break;
        }
    }
    (passes, n)
}

fn is_outlier(reading: i32) -> bool {
    !(0..=100).contains(&reading)
}

fn main() {
    println!("1. The condition is checked before every pass");
    let (passes, n) = count_down_while(3);
    println!("   while n > 0, starting at 3   passes = {passes}, n after = {n}");
    let (passes, n) = count_down_while(0);
    println!("   while n > 0, starting at 0   passes = {passes}, n after = {n}   <- the body never ran");
    let (passes, n) = count_down_do_while(0);
    println!("   loop + break at the bottom   passes = {passes}, n after = {n}  <- the do...while shape runs once");

    println!();
    println!("2. The book's sample loop, over a fixed table instead of a sensor");
    let readings = [42, -7, 55, 180, 61, 38, 999, 70, 12];
    let mut samples: Vec<i32> = Vec::new();
    let mut next = 0;
    let mut skipped = 0;
    while samples.len() < 5 && next < readings.len() {
        let sample = readings[next];
        next += 1; // before the `continue`, or an outlier is re-read forever
        if is_outlier(sample) {
            skipped += 1;
            continue;
        }
        samples.push(sample);
    }
    println!("   readings = {readings:?}");
    println!("   samples  = {samples:?}");
    println!("   read {next} of {} readings, skipped {skipped} outliers", readings.len());
    println!("   The second condition, `next < readings.len()`, is the table talking:");
    println!("   a sensor never runs out, a table does.");

    let same: Vec<i32> = readings.iter().copied().filter(|&r| !is_outlier(r)).take(5).collect();
    println!("   filter(..).take(5)  = {same:?}   <- the same answer, no mut, no index");

    println!();
    println!("3. A while loop evaluates to ()");
    let mut k = 0;
    let unit = while k < 3 {
        k += 1;
    };
    println!("   let unit = while k < 3 {{ k += 1; }};   unit = {unit:?}, k = {k}");
    println!("   The answer lives in the mut declared above the loop, never in the loop.");

    println!();
    println!("4. Listing 2.7: a condition that is not a sequence");
    let time_limit = Duration::from_millis(10);
    let mut count: u64 = 0;
    let start = Instant::now();
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    let elapsed = start.elapsed();
    println!("   time_limit              = {time_limit:?}");
    println!("   count > 0               = {}", count > 0);
    println!("   elapsed >= time_limit   = {}", elapsed >= time_limit);
    println!("   The count itself differs on every run and every machine,");
    println!("   so this program checks it and does not print it.");

    println!();
    println!("5. An index and a while is a for loop in disguise");
    let prices = [250, 1_000, 75];
    let mut total_while = 0;
    let mut i = 0;
    while i < prices.len() {
        total_while += prices[i];
        i += 1;
    }
    let mut total_for = 0;
    for price in prices {
        total_for += price;
    }
    println!("   while i < prices.len() {{ .. i += 1; }}   total = {total_while}   (2 muts, 1 index)");
    println!("   for price in prices {{ .. }}              total = {total_for}   (1 mut, no index)");
    println!("   prices.iter().sum()                     total = {}", prices.iter().sum::<i32>());
    println!("   Forget `i += 1` and only the first version can loop forever.");
}
