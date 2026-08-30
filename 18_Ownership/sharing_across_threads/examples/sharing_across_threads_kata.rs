//! Kata solution: three refusals, three fixes, and the one you can delete.
//!
//!   rustc --edition 2024 sharing_across_threads_kata.rs -o /tmp/satk && /tmp/satk
//!
//! Nothing prints from inside a spawned thread, and one number here is
//! deliberately NOT printed — see part 2.

use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::sync::{Arc, Mutex};
use std::thread;

const THREADS: usize = 8;
const PER_THREAD: u32 = 5_000;

fn main() {
    println!("Part 1 — three refusals, three fixes.\n");

    println!("  (a) `Rc` into thread::spawn");
    println!("      error[E0277]: `Rc<Vec<String>>` cannot be sent between threads safely");
    println!("      the trait `Send` is not implemented for `Rc<Vec<String>>`");
    println!("      Fix: Arc. Same API, same size, atomic count.");
    let roster = Arc::new(vec!["Ada".to_string(), "Ben".to_string()]);
    let mine = Arc::clone(&roster);
    let names = thread::spawn(move || mine.len()).join().unwrap();
    println!("      the thread saw {names} names\n");

    println!("  (b) pushing through a shared `Arc<Vec<u32>>`");
    println!("      error[E0596]: cannot borrow data in an `Arc` as mutable");
    println!("      Fix: Arc<Mutex<T>>. Arc grants the ownership, Mutex the write.");
    let tally = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for id in 0..4u32 {
        let mine = Arc::clone(&tally);
        handles.push(thread::spawn(move || mine.lock().unwrap().push(id)));
    }
    for h in handles {
        h.join().unwrap();
    }
    let mut rows = tally.lock().unwrap().clone();
    rows.sort(); // the order they arrived in is the scheduler's business
    println!("      four threads wrote {rows:?}\n");

    println!("  (c) forgetting the per-thread clone");
    println!("      error[E0382]: use of moved value: `shared`");
    println!("      The first `move` closure took the only Arc; the second had none.");
    println!("      Fix: one Arc::clone per spawn, made before the loop body ends.");
    let shared = Arc::new(41u32);
    let mut handles = Vec::new();
    for _ in 0..3 {
        let mine = Arc::clone(&shared); // <- the line that was missing
        handles.push(thread::spawn(move || *mine + 1));
    }
    let got: Vec<u32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("      three threads returned {got:?}");
    println!("      strong_count is back to {} now they have all joined\n", Arc::strong_count(&shared));

    println!("Part 2 — predict the two totals.\n");

    println!("  fetch_add is one indivisible step, so the answer is arithmetic:");
    let atomic = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let mine = Arc::clone(&atomic);
        handles.push(thread::spawn(move || {
            for _ in 0..PER_THREAD {
                mine.fetch_add(1, Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let expected = THREADS as u32 * PER_THREAD;
    println!("    {THREADS} x {PER_THREAD} = {expected}, and it counted {}", atomic.load(Relaxed));

    println!("\n  load-then-store is three steps, so a scheduler can split it.");
    println!("  Scripted on one thread, the interleaving looks like this:");
    let racy = AtomicU32::new(0);
    let a = racy.load(Relaxed);
    let b = racy.load(Relaxed);
    racy.store(a + 1, Relaxed);
    racy.store(b + 1, Relaxed);
    println!("    A reads {a}, B reads {b}, both write 1 -> counter says {}", racy.load(Relaxed));

    println!("\n  Run that across {THREADS} real threads and updates go missing:");
    let racy = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let mine = Arc::clone(&racy);
        handles.push(thread::spawn(move || {
            for _ in 0..PER_THREAD {
                let seen = mine.load(Relaxed);
                mine.store(seen + 1, Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let counted = racy.load(Relaxed);
    println!("    counted <= {expected}? {}", counted <= expected);
    println!("    The actual number is NOT printed here, on purpose: it differs");
    println!("    every run, so no answer key could hold it. That is the same");
    println!("    fact as the bug — a lost update is not reproducible, which is");
    println!("    why you cannot test your way to noticing one.");

    println!("\nPart 3 — the Arc you can delete.\n");
    let precincts = vec![41u32, 17, 88, 5];
    let total: u32 = thread::scope(|s| {
        let workers: Vec<_> = precincts
            .chunks(2)
            .map(|chunk| s.spawn(move || chunk.iter().sum::<u32>()))
            .collect();
        workers.into_iter().map(|w| w.join().unwrap()).sum()
    });
    println!("  thread::scope summed {precincts:?} to {total} with no Arc at all,");
    println!("  and `precincts` is still usable here: {}", precincts.len());
    println!("\n  The rule to carry away: Arc is for threads that OUTLIVE the");
    println!("  borrow. A scoped thread cannot, so it needs no owner and no");
    println!("  count. Reach for the scope first and pay for the Arc second.");
}
