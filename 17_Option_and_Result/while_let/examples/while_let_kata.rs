//! Kata solution: the pattern is the exit condition — and nothing checks it.
//!
//!   rustc --edition 2024 while_let_kata.rs -o /tmp/wlk && /tmp/wlk

fn main() {
    // Drain: each pass shortens the stack, so the pattern eventually fails.
    let mut stack = vec![5u8, 3, 0, 4];
    print!("Draining with `while let Some(x) = stack.pop()`:\n ");
    while let Some(x) = stack.pop() {
        print!(" {x}");
    }
    println!("\n  stack is now {stack:?} — the None ended the loop");

    // The same loop written over an iterator, and then the way you would
    // actually write it.
    let scores = [5u8, 3, 0];
    let mut it = scores.iter();
    print!("\n`while let Some(s) = it.next()`:\n ");
    while let Some(s) = it.next() {
        print!(" {s}");
    }
    print!("\n`for s in scores`               :\n ");
    for s in scores {
        print!(" {s}");
    }
    println!("\n  Identical. `for` is this loop with the advance built in, which");
    println!("  is exactly the line the next demo forgets.");

    // The bug: a body that does not move toward the failing pattern. Bounded
    // here with a counter so the example terminates and can be verified — in
    // real code there is no counter, and the program simply hangs.
    // Not `mut` — and that is the tell: this loop never changes the queue.
    let queue = vec!["late row"];
    let mut passes = 0;
    println!("\nA body that never shortens the queue:");
    while let Some(item) = queue.last() {
        passes += 1;
        println!("  pass {passes}: still holding {item:?}");
        if passes == 3 {
            println!("  (stopped by a counter that only exists in this demo)");
            break;
        }
        // The missing line is `queue.pop();`.
    }
    println!("  `last()` peeks; `pop()` advances. Nothing in the language knows");
    println!("  which one your loop needed — `while let` re-tests the pattern and");
    println!("  is perfectly happy to re-test it forever.");
}
