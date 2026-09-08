//! Kata solution: no path can skip a drop.
//!
//!   rustc --edition 2024 forgotten_unlock_kata.rs -o /tmp/fuk && /tmp/fuk

use std::sync::Mutex;

fn total(m: &Mutex<Vec<u32>>, fail_early: bool) -> Result<u32, &'static str> {
    let guard = m.lock().unwrap();
    if fail_early {
        // The early return that causes the C bug. Here it releases the lock:
        // `guard` is dropped on the way out, on this path like any other.
        return Err("refused before summing");
    }
    Ok(guard.iter().sum())
}

fn main() {
    let m = Mutex::new(vec![1u32, 2, 3, 4]);

    println!("THE C SHAPE");
    println!("  pthread_mutex_lock(&m);");
    println!("  if (bad) return -1;            /* <- lock still held */");
    println!("  ...");
    println!("  pthread_mutex_unlock(&m);");
    println!("  Every path out needs its own unlock, including the paths added");
    println!("  next year by somebody who did not read the top of the function.");
    println!("  The failure is a deadlock in an unrelated thread, later.");
    println!();

    println!("THE RUST VERSION, BOTH PATHS");
    println!("  total(&m, false) = {:?}", total(&m, false));
    println!("  total(&m, true)  = {:?}", total(&m, true));
    println!("  ...and the lock is free afterwards: {:?}", total(&m, false));
    println!();
    println!("  `lock()` returns a GUARD, and the guard's Drop releases. There");
    println!("  is no unlock call to forget because there is no unlock call --");
    println!("  the early return drops the guard exactly as the normal return");
    println!("  does, and so does a panic unwinding through the function.");
    println!();

    println!("THE DETAIL THAT MAKES IT MORE THAN A CONVENTION");
    println!("  The data is inside the Mutex, not beside it. `m.lock()` is the");
    println!("  only way to a &mut Vec, so 'accessing without the lock' is not a");
    println!("  mistake you can make -- where C's mutex is a separate object");
    println!("  and nothing connects it to the data it protects.");
    println!();

    println!("AND THE ONE WAY TO STILL GET IT WRONG");
    println!("  let _ = m.lock().unwrap();      <- releases IMMEDIATELY");
    println!("  let _guard = m.lock().unwrap(); <- held to end of scope");
    println!("  `_` is a pattern that binds nothing, so the guard is a temporary");
    println!("  that dies at the end of the statement. That is the one remaining");
    println!("  way to hold a lock for zero instructions and think you held it,");
    println!("  and it is a one-character difference.");
    println!();

    println!("C++ HAS THE SAME ANSWER, WITH A GAP");
    println!("  std::lock_guard is RAII and does the same job. The difference is");
    println!("  that C++ cannot stop you touching the data without it, and");
    println!("  cannot stop a reference to the protected data escaping the");
    println!("  guard's scope. Rust's borrow checker ties the &mut to the");
    println!("  guard's lifetime, so it cannot outlive the lock.");

    assert_eq!(total(&m, false), Ok(10));
    assert!(total(&m, true).is_err());
}
