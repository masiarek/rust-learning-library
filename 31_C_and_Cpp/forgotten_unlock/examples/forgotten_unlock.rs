// The same program that deadlocks in C. There is no unlock() to skip: the
// guard is a value, and the lock is released when that value is dropped.

use std::sync::Mutex;

static TALLY: Mutex<i32> = Mutex::new(0);

fn record(score: i32) -> i32 {
    let mut tally = TALLY.lock().unwrap();      // held while `tally` lives

    if score < 0 {
        return -1;                              // the guard drops on the way out
    }

    *tally += score;
    *tally
}

fn main() {
    println!("record(5)  -> {}", record(5));    // 5
    println!("record(-1) -> {}", record(-1));   // -1
    println!("record(3)  -> {}", record(3));    // 8 — the C version never got here
}
