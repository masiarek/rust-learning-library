//! Kata solution: the lock returns a Result for exactly one reason.
//!
//!   rustc --edition 2024 mutex_poisoning_kata.rs -o /tmp/mpk && /tmp/mpk

use std::panic;
use std::sync::{Arc, Mutex};
use std::thread;

/// The invariant: `total` is the sum of `counted`. A panic halfway through an
/// update leaves that false — which is the thing poisoning is warning about.
#[derive(Debug, Default)]
struct Tally {
    counted: Vec<u32>,
    total: u32,
}

fn main() {
    let tally = Arc::new(Mutex::new(Tally::default()));

    // A worker that dies holding the guard, mid-update.
    let worker = Arc::clone(&tally);
    let prior = panic::take_hook();
    panic::set_hook(Box::new(|_| {})); // keep this demo's stderr clean
    let handle = thread::spawn(move || {
        let mut t = worker.lock().unwrap();
        t.counted.push(12); // half of the update...
        panic!("row 3 was unreadable"); // ...and the other half never happens
    });
    let died = handle.join().is_err();
    panic::set_hook(prior);

    println!("worker thread panicked while holding the lock: {died}");

    println!("\nAnswer 1 — propagate. The invariant is broken; say so:");
    match tally.lock() {
        Ok(_) => println!("  lock() -> Ok (not what happens here)"),
        Err(e) => println!("  lock() -> Err: {e}"),
    }

    println!("\nAnswer 2 — recover, having looked at the damage:");
    let recovered = match tally.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            let guard = poisoned.into_inner(); // the data is still there
            println!("  into_inner() -> {:?}", *guard);
            println!("      counted has one entry, total is 0 — exactly the broken");
            println!("      state the panic left. Recovering means REPAIRING this,");
            println!("      not merely getting past the Err.");
            guard
        }
    };
    let repaired_total: u32 = recovered.counted.iter().sum();
    drop(recovered);
    {
        let mut t = tally.lock().unwrap_or_else(|p| p.into_inner());
        t.total = repaired_total;
        println!("  repaired -> {:?}", *t);
    }

    println!("\nPoisoning is sticky:");
    println!("  still poisoned? {}", tally.is_poisoned());
    println!("      Repairing the DATA does not clear the flag. Clearing it is");
    println!("      `clear_poison()`, and it is a separate, deliberate step — so");
    println!("      that no other thread quietly gets an Ok it did not earn.");
    tally.clear_poison();
    println!("  after clear_poison() -> {}", tally.is_poisoned());
    println!("  lock() now -> {:?}", tally.lock().map(|g| g.total));

    println!("\nAnswer 3 — `.lock().unwrap()`, which is a decision, not a shortcut:");
    println!("      It says: another thread broke this invariant, so this thread");
    println!("      should die too. Often right. Worth writing as .expect(\"…\")");
    println!("      with the reason, since that is the sentence you will read.");
}
