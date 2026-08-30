//! `Arc`: the same count, made atomic so it can cross a thread boundary.
//!
//!   rustc --edition 2024 sharing_across_threads.rs -o /tmp/arc && /tmp/arc
//!
//! Nothing is printed from inside a spawned thread. Every thread returns a
//! value, main joins them all, and the reporting happens on one thread in a
//! fixed order — otherwise the transcript would differ from run to run and
//! the recorded answer key would be worthless.

use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::sync::{Arc, Mutex};
use std::thread;

const THREADS: usize = 8;
const PER_THREAD: u32 = 1_000;

fn main() {
    println!("1. The same count as `Rc`, and one owner per thread");
    let roster = Arc::new(vec!["Ada".to_string(), "Ben".to_string(), "Cara".to_string()]);
    println!("   after Arc::new                     {}", Arc::strong_count(&roster));

    // One clone per thread, made BEFORE any thread starts: the closure needs
    // an owner of its own, and `move` takes it. Collecting them first is what
    // makes the count below an observation rather than an assertion.
    let owners: Vec<Arc<Vec<String>>> = (0..4).map(|_| Arc::clone(&roster)).collect();
    println!("   four clones, no thread started yet {}", Arc::strong_count(&roster));

    let handles: Vec<_> = owners
        .into_iter()
        .enumerate()
        .map(|(i, mine)| thread::spawn(move || format!("thread {i} sees {} names", mine.len())))
        .collect();

    let mut seen: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    seen.sort();
    for line in &seen {
        println!("     {line}");
    }
    println!("   after every thread joined          {}", Arc::strong_count(&roster));
    println!("   The count is the same mechanism as `Rc`'s. Only the increment differs.");

    println!();
    println!("2. What `atomic` is paying for");
    println!("   A read-modify-write is three steps, and a scheduler may split it.");
    println!("   Here is that interleaving, executed in order on one thread:");
    let racy = AtomicU32::new(0);
    let read_by_a = racy.load(Relaxed);
    let read_by_b = racy.load(Relaxed);
    racy.store(read_by_a + 1, Relaxed);
    racy.store(read_by_b + 1, Relaxed);
    println!("     A reads {read_by_a}, B reads {read_by_b}, A writes {}, B writes {}",
             read_by_a + 1, read_by_b + 1);
    println!("     two increments, and the counter says {}", racy.load(Relaxed));
    println!("   `fetch_add` is the same three steps made indivisible:");
    let atomic = AtomicU32::new(0);
    atomic.fetch_add(1, Relaxed);
    atomic.fetch_add(1, Relaxed);
    println!("     two increments, and the counter says {}", atomic.load(Relaxed));
    println!("   Lose one of those on a REFERENCE count and you free a live value.");
    println!("   That is the bug `Arc` exists to make impossible, and the reason");
    println!("   `Rc` is not merely discouraged across threads but refused.");

    println!();
    println!("3. `fetch_add` under real contention");
    let hits = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let mine = Arc::clone(&hits);
        handles.push(thread::spawn(move || {
            for _ in 0..PER_THREAD {
                mine.fetch_add(1, Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("   {THREADS} threads x {PER_THREAD} increments = {}", hits.load(Relaxed));
    println!("   Exact, every run. The load/store version above would not be.");

    println!();
    println!("4. `Arc` grants the ownership; `Mutex` grants the write");
    let tally = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for id in 0..THREADS {
        let mine = Arc::clone(&tally);
        handles.push(thread::spawn(move || {
            mine.lock().unwrap().push(id * 10);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let mut rows = tally.lock().unwrap().clone();
    rows.sort(); // arrival order is the scheduler's business, so do not print it
    println!("   {THREADS} threads wrote into one Vec: {rows:?}");
    println!("   `Arc<T>` alone hands out `&T`, so it cannot be pushed to at all.");
    println!("   Neither type substitutes for the other, and the borrow rule");
    println!("   moves to run time: two live `lock()`s on one thread deadlock.");

    println!();
    println!("5. When no counting is needed at all");
    let precincts = vec![41_u32, 17, 88, 5];
    let total: u32 = thread::scope(|s| {
        let workers: Vec<_> = precincts
            .chunks(2)
            .map(|chunk| s.spawn(move || chunk.iter().sum::<u32>()))
            .collect();
        workers.into_iter().map(|w| w.join().unwrap()).sum()
    });
    println!("   thread::scope borrowed {precincts:?} with no Arc: total {total}");
    println!("   A scoped thread cannot outlive the borrow, so the compiler needs");
    println!("   no `'static` and you need no owner per thread. Reach for this");
    println!("   first; `Arc` is for the threads that DO outlive the scope.");

    println!();
    println!("6. Same size, different promise");
    println!("   size_of::<Rc<i32>>()  = {}", size_of::<std::rc::Rc<i32>>());
    println!("   size_of::<Arc<i32>>() = {}", size_of::<Arc<i32>>());
    println!("   Identical values, identical layout. `Send` is the difference,");
    println!("   and it is a promise the compiler tracks, not a byte in the value.");
    println!("   What `Arc` does NOT change: it still hands out `&T`, and two");
    println!("   `Arc`s pointing at each other still leak, exactly as `Rc` does.");
}
