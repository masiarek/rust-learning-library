//! Kata solution: the same fan-out three ways, and the one that needs no Arc.
//!
//!   rustc --edition 2024 spawning_a_thread_kata.rs -o /tmp/spk && /tmp/spk

use std::sync::Arc;
use std::thread;

/// Eight rows, three columns. The job: a per-column total, in parallel.
fn rows() -> Vec<[u32; 3]> {
    vec![
        [5, 3, 0], [4, 4, 1], [0, 5, 2], [3, 3, 3],
        [5, 0, 5], [2, 4, 4], [1, 1, 5], [4, 2, 0],
    ]
}

fn main() {
    let rows = rows();

    println!("1. Sequential, for the answer to check against");
    let expected: Vec<u32> = (0..3).map(|i| rows.iter().map(|b| b[i]).sum()).collect();
    println!("   totals = {expected:?}");

    println!();
    println!("2. spawn + Arc: the version that compiles first for most people");
    let shared = Arc::new(rows.clone());
    let mut handles = Vec::new();
    for i in 0..3 {
        let shared = Arc::clone(&shared);
        handles.push(thread::spawn(move || shared.iter().map(|b| b[i]).sum::<u32>()));
    }
    let with_arc: Vec<u32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("   totals = {with_arc:?}   matches: {}", with_arc == expected);
    println!("   Three Arc clones, one refcount, and a `move` per closure. The Arc");
    println!("   is here for exactly one reason: `spawn` needs a 'static closure,");
    println!("   and a borrow of a local is not 'static.");

    println!();
    println!("3. scope: the same thing with no Arc and no clone");
    let rows_ref = &rows;   // a shared reference is Copy, so `move` copies IT
    let with_scope: Vec<u32> = thread::scope(|s| {
        let handles: Vec<_> = (0..3)
            .map(|i| s.spawn(move || rows_ref.iter().map(|b| b[i]).sum::<u32>()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    println!("   totals = {with_scope:?}   matches: {}", with_scope == expected);
    println!("   The closures BORROW `rows`. `scope` cannot return until every");
    println!("   thread it started has finished, so the borrow provably ends first");
    println!("   — which is the whole reason the lifetime works out.");
    println!("   The `move` on each closure is about `i`, not about the data: a");
    println!("   loop variable is owned by the loop, so it has to be copied in, and");
    println!("   `move` then copies the shared REFERENCE rather than the Vec.");

    println!();
    println!("4. The version that does not compile, and what rustc says");
    println!("   let h = thread::spawn(|| rows.len());   // no `move`");
    println!("   E0373: \"closure may outlive the current function, but it borrows");
    println!("   `rows`, which is owned by the current function\", with a note that");
    println!("   \"function requires argument type to outlive `'static`\" and the");
    println!("   help offering `move`. Adding `move` compiles — and MOVES rows into");
    println!("   the first thread, so a second one cannot have it. That is the");
    println!("   moment most people reach for Arc; scope is the other answer.");

    println!();
    println!("5. Which to reach for");
    println!("   scope        the data is on this stack and the work finishes here.");
    println!("                Cheapest, and the borrow checker still helps.");
    println!("   Arc          the thread outlives this function, or the value has");
    println!("                no single owner. Read-only sharing needs nothing more.");
    println!("   Arc<Mutex>   ...and they also WRITE. Every reader now pays for the");
    println!("                writers, so ask whether a channel would do instead.");
    println!("   The order matters: each rung costs something the one above does");
    println!("   not, and a fan-out over data you already have needs only the first.");
}
