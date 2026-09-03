//! Lock poisoning: why `Mutex::lock()` returns a `Result`, and what your
//! `.unwrap()` is actually deciding.
//!
//! A mutex protects an invariant, not just bytes. If a thread panics while
//! holding the guard, the invariant may be half-restored — so std records that
//! fact in the lock itself and hands the next thread an `Err`. Poisoning is not
//! an error in the usual sense; it is a message from a thread that died.
//!
//!   rustc --edition 2024 mutex_poisoning.rs -o /tmp/mp && /tmp/mp

use std::panic;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

/// Run a thread that panics, without letting its message clutter the demo.
fn quietly(f: impl FnOnce() + Send + 'static) {
    let prior = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let _ = thread::spawn(f).join();
    panic::set_hook(prior);
}

/// The invariant: this list always holds (key, value) as ADJACENT pairs,
/// so its length is always even.
fn is_consistent(v: &[i32]) -> bool {
    v.len() % 2 == 0
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() {
    banner(1, "lock() returns a Result — and normally it is Ok");

    let scores = Mutex::new(vec![1, 5, 2, 4]);
    println!("  lock().is_ok()        -> {}", scores.lock().is_ok());
    println!("  is_poisoned()         -> {}", scores.is_poisoned());
    println!("  data                  -> {:?}", *scores.lock().unwrap());
    println!("      The Result has exactly one possible cause: a previous holder");
    println!("      panicked. Nothing else makes lock() fail.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn poisoned_pair_list() -> Arc<Mutex<Vec<i32>>> {
    let shared = Arc::new(Mutex::new(vec![1, 5, 2, 4]));
    let writer = Arc::clone(&shared);

    quietly(move || {
        let mut guard = writer.lock().unwrap();
        guard.push(3); // the key...
        panic!("failed before writing the score"); // ...and the score never comes
    });

    shared
}

fn step2() {
    banner(2, "A holder panics: the invariant breaks and the lock remembers");

    let shared = poisoned_pair_list();

    println!("  is_poisoned()         -> {}", shared.is_poisoned());
    match shared.lock() {
        Ok(_) => println!("  lock()                -> Ok"),
        Err(e) => {
            let data = e.get_ref();
            println!("  lock()                -> Err(PoisonError)");
            println!("  the data behind it    -> {:?}", *data);
            println!("  invariant holds?      -> {}", is_consistent(&data));
        }
    }
    println!("      The Vec is not corrupt in a memory sense — it is corrupt in the");
    println!("      sense that matters: a key with no value. That is what the");
    println!("      Err is warning the next thread about.");
}

// ─────────────────────────────────────────────────────────── Step 3
/// The third response: hand the poisoning to the caller.
fn total(shared: &Mutex<Vec<i32>>) -> Result<i32, String> {
    let data = shared
        .lock()
        .map_err(|_| "the score list was left inconsistent by a panicking thread".to_string())?;
    Ok(data.iter().sum())
}

fn step3() {
    banner(3, "Three honest responses, and the one you write by accident");

    let shared = poisoned_pair_list();

    // (a) Die with it.
    let prior = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let died = panic::catch_unwind(|| shared.lock().unwrap().len()).is_err();
    panic::set_hook(prior);
    println!("  (a) .unwrap()             -> panicked: {died}");

    // (b) Carry on deliberately.
    let data = shared.lock().unwrap_or_else(|e| e.into_inner());
    println!("  (b) .unwrap_or_else(into_inner) -> {:?}  consistent? {}", *data, is_consistent(&data));
    drop(data);

    // (c) Give the caller the choice.
    println!("  (c) map_err + ?           -> {:?}", total(&shared));

    println!("      (a) is the default everyone types. It is a real decision: this");
    println!("      thread dies because a different one did. Fine in an application,");
    println!("      rude in a library — the caller never got to choose.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "Poisoning is sticky, and clearable");

    let shared = poisoned_pair_list();
    println!("  poisoned now?         -> {}", shared.is_poisoned());
    println!("  every later lock()    -> Err, forever, in every thread");

    // Repair the invariant first, THEN clear the flag.
    {
        let mut data = shared.lock().unwrap_or_else(|e| e.into_inner());
        data.push(0); // the missing score
        println!("  repaired to           -> {:?}  consistent? {}", *data, is_consistent(&data));
    }
    shared.clear_poison();
    println!("  after clear_poison()  -> poisoned? {}, lock() ok? {}",
             shared.is_poisoned(), shared.lock().is_ok());
    println!("      clear_poison() only clears the FLAG. Fixing the data is your job,");
    println!("      and doing it in the other order tells the next thread a lie.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "What actually poisons: the guard's Drop during unwinding");

    // A thread that panics WITHOUT holding the lock.
    let bystander = Arc::new(Mutex::new(0));
    let other = Arc::clone(&bystander);
    quietly(move || {
        let _unrelated = &other;
        panic!("nothing to do with the mutex");
    });
    println!("  panic while NOT holding -> poisoned? {}", bystander.is_poisoned());

    // RwLock: a reader that panics does not poison; a writer does.
    let rw = Arc::new(RwLock::new(vec![1, 5]));
    let reader = Arc::clone(&rw);
    quietly(move || {
        let _guard = reader.read().unwrap();
        panic!("panicking with a READ guard");
    });
    println!("  RwLock, reader panicked -> poisoned? {}", rw.is_poisoned());

    let writer = Arc::clone(&rw);
    quietly(move || {
        let mut guard = writer.write().unwrap();
        guard.push(2);
        panic!("panicking with a WRITE guard");
    });
    println!("  RwLock, writer panicked -> poisoned? {}", rw.is_poisoned());
    println!("      Only an exclusive guard can leave a half-written invariant, so only");
    println!("      an exclusive guard poisons. The rule is mechanical, not a judgement.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn step6() {
    banner(6, "When poisoning is noise");

    let hits = Arc::new(Mutex::new(0u64));
    let other = Arc::clone(&hits);
    quietly(move || {
        let mut n = other.lock().unwrap();
        *n += 1;
        panic!("died after a complete increment");
    });

    let n = hits.lock().unwrap_or_else(|e| e.into_inner());
    println!("  counter after the panic -> {n}, poisoned? {}", hits.is_poisoned());
    println!("      Nothing here can be half-done: one += 1 is complete or it never ran.");
    println!("      The flag is still set, because std cannot know that. This is the case");
    println!("      the parking_lot crate has in mind when it drops poisoning entirely.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    println!();
}
