//! `spawn` starts a thread; `join` is the only thing that gets a value back.
//!
//!   rustc --edition 2024 spawning_a_thread.rs -o /tmp/spawn && /tmp/spawn

use std::thread;

const BALLOTS: [[u32; 3]; 8] = [
    [5, 3, 0], [4, 4, 1], [0, 5, 2], [3, 3, 3],
    [5, 0, 5], [2, 4, 4], [1, 1, 5], [4, 2, 0],
];

fn main() {
    println!("1. spawn takes a closure and hands back a handle");
    let handle = thread::spawn(|| {
        // Runs on another thread. Whatever it returns comes back through join.
        BALLOTS.iter().map(|b| b[0]).sum::<u32>()
    });
    let total_a = handle.join().unwrap();
    println!("   candidate A total, computed on another thread: {total_a}");
    println!("   `spawn` returns a JoinHandle<T>, and `join` is the ONLY way to");
    println!("   get the T back. It also returns a Result, because the thread may");
    println!("   have panicked instead of finishing.");

    println!();
    println!("2. Four threads, and why the output is still in order");
    let mut handles = Vec::new();
    for (i, name) in ["A", "B", "C"].iter().enumerate() {
        handles.push(thread::spawn(move || {
            let total: u32 = BALLOTS.iter().map(|b| b[i]).sum();
            (*name, total)
        }));
    }
    // The threads finish in whatever order they like; joining in a fixed order
    // is what makes the report deterministic.
    for h in handles {
        let (name, total) = h.join().unwrap();
        println!("   {name}: {total}");
    }
    println!("   Nothing was printed FROM a thread. Interleaved println! output is");
    println!("   the classic non-deterministic test failure — do the work in the");
    println!("   thread, return the value, and print on the joining side.");

    println!();
    println!("3. `move` is usually not optional");
    let threshold = 4u32;
    let counted = thread::spawn(move || BALLOTS.iter().filter(|b| b[0] >= threshold).count());
    println!("   ballots with A >= {threshold}: {}", counted.join().unwrap());
    println!("   Without `move`, the closure borrows `threshold` — and the compiler");
    println!("   refuses, because it cannot prove the borrow outlives the thread:");
    println!("   E0373, \"closure may outlive the current function, but it borrows");
    println!("   `threshold`, which is owned by the current function\". `spawn`");
    println!("   requires a `'static` closure for exactly that reason.");

    println!();
    println!("4. A panicking thread does not take the process with it");
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));   // keep the message off stderr
    let doomed = thread::spawn(|| {
        let empty: [u32; 0] = [];
        empty[std::hint::black_box(0)]
    });
    let outcome = doomed.join();
    std::panic::set_hook(hook);
    match outcome {
        Ok(v) => println!("   returned {v}"),
        Err(_) => println!("   the thread panicked; join() gave Err and main is fine"),
    }
    println!("   The panic message would normally go to stderr — suppressed here");
    println!("   so this example's transcript stays clean. `join` returns");
    println!("   Err(Box<dyn Any + Send>), which is the panic payload — usually a");
    println!("   String you can downcast if you care.");

    println!();
    println!("5. `scope` borrows what `spawn` cannot");
    let local: Vec<u32> = BALLOTS.iter().map(|b| b[2]).collect();
    let (sum, max) = thread::scope(|s| {
        let a = s.spawn(|| local.iter().sum::<u32>());
        let b = s.spawn(|| local.iter().copied().max().unwrap_or(0));
        (a.join().unwrap(), b.join().unwrap())
    });
    println!("   C's scores {local:?} -> sum {sum}, max {max}");
    println!("   Both closures BORROW `local`, with no Arc and no clone. `scope`");
    println!("   guarantees every thread it started has finished before it returns,");
    println!("   so the borrow cannot outlive the data. Stable since 1.63, and the");
    println!("   right default for fan-out over data you already have.");
}
