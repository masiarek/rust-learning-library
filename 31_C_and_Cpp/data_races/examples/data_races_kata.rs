//! Kata solution: the race you cannot write.
//!
//!   rustc --edition 2024 data_races_kata.rs -o /tmp/drk && /tmp/drk

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

const THREADS: u32 = 8;
const EACH: u32 = 10_000;

fn main() {
    println!("THE C SHAPE");
    println!("  static int counter;   ...   counter++;   /* in 8 threads */");
    println!("  counter++ is load, add, store. Two threads can load the same");
    println!("  value and both store one more than it, so increments vanish --");
    println!("  and the DEBUG build loses them while -O2 may keep the value in a");
    println!("  register and produce the right answer, which is the worst");
    println!("  possible way to learn about a race.");
    println!();

    println!("WRITING IT IN RUST: THE PROGRAM DOES NOT EXIST");
    println!("  let mut counter = 0u32;");
    println!("  thread::spawn(|| counter += 1);   <- E0373 / E0499");
    println!("  A closure sent to another thread may outlive this frame, so it");
    println!("  cannot borrow `counter`; and two of them cannot hold `&mut` to");
    println!("  one value at all. The check is the ordinary borrow checker --");
    println!("  there is no threading special case.");
    println!();

    println!("1. THE MUTEX VERSION");
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..EACH { *c.lock().unwrap() += 1; }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("  {THREADS} threads x {EACH} increments = {}", *counter.lock().unwrap());
    println!("  Exact, every run. The data is INSIDE the mutex, so there is no");
    println!("  way to reach it without locking -- unlike a C mutex, which is a");
    println!("  separate object you have to remember to take.");
    println!();

    println!("2. THE ATOMIC VERSION");
    let atomic = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let a = Arc::clone(&atomic);
        handles.push(thread::spawn(move || {
            for _ in 0..EACH { a.fetch_add(1, Ordering::Relaxed); }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("  same total, no lock: {}", atomic.load(Ordering::SeqCst));
    println!("  fetch_add is one indivisible operation, so the load-add-store");
    println!("  window that the C bug lives in does not exist.");
    println!();

    println!("WHAT MAKES THIS CHECKABLE AT ALL");
    println!("  Two marker traits. `Send` means a value may move to another");
    println!("  thread; `Sync` means &T may be shared between them. u32 is both,");
    println!("  &mut u32 is not Sync, Rc is neither -- so `thread::spawn`");
    println!("  requiring Send is what rejects the bad programs, at the type");
    println!("  level, with no runtime cost and no sanitizer.");
    println!();
    println!("  Note what is NOT promised: Rust prevents data races, not race");
    println!("  CONDITIONS. Two correctly-locked threads can still interleave");
    println!("  in an order your logic did not expect. The guarantee is about");
    println!("  memory, and it is the half that produces impossible values.");

    assert_eq!(*counter.lock().unwrap(), THREADS * EACH);
    assert_eq!(atomic.load(Ordering::SeqCst), THREADS * EACH);
}
