// Two threads, one counter, three runs, one answer.

use std::sync::{Arc, Mutex};
use std::thread;

fn count_twice() -> i64 {
    let tally = Arc::new(Mutex::new(0i64));

    let workers: Vec<_> = (0..2)
        .map(|_| {
            let tally = Arc::clone(&tally);     // one handle per thread
            thread::spawn(move || {
                for _ in 0..100_000 {
                    *tally.lock().unwrap() += 1;
                }
            })
        })
        .collect();

    for w in workers {
        w.join().unwrap();
    }

    *tally.lock().unwrap()
}

fn main() {
    for run in 1..=3 {
        println!("run {run} -> {}", count_twice());   // 200000, every time
    }

    // Drop the Mutex and let both closures touch the i64 directly and it does
    // not build: E0499, two mutable borrows of `tally` at once.
}
