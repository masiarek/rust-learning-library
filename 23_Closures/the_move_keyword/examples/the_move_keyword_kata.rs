//! Kata solution: `move` answers a lifetime question, not a trait question.
//!
//! Without `move` the compiler captures by reference wherever it can, so the
//! closure borrows your scope and cannot outlive it. With `move` the captured
//! values are stored INSIDE the closure, so it can be returned, sent to another
//! thread, or stored -- and your scope no longer owns them.
//!
//!   rustc --edition 2024 the_move_keyword_kata.rs -o /tmp/tmk && /tmp/tmk

use std::thread;

fn make_greeter(name: String) -> impl Fn() -> String {
    // Without `move` this does not compile: the closure would hold a &String
    // pointing at `name`, which dies when this function returns. The error is
    // "closure may outlive the current function", and it names the fix.
    move || format!("hello, {name}")
}

fn main() {
    println!("1. RETURNING A CLOSURE NEEDS `move`");
    let greet = make_greeter(String::from("ada"));
    println!("  {}", greet());
    println!("  {}", greet());
    println!("  The String lives inside the closure now. That is what let it");
    println!("  outlive make_greeter, and it is a question about WHERE the");
    println!("  value lives -- not about what the body does with it.");
    println!();

    println!("2. SENDING ONE TO A THREAD NEEDS `move` FOR THE SAME REASON");
    let rows = vec![3u32, 1, 4, 1, 5];
    let handle = thread::spawn(move || rows.iter().sum::<u32>());
    println!("  sum from the other thread: {}", handle.join().unwrap());
    println!("  The spawned thread may outlive this function, so a borrow of");
    println!("  `rows` would be unsound and the compiler says so. `move` gives");
    println!("  the Vec to the closure, and naming `rows` below here would now");
    println!("  be a use-after-move error.");
    println!();

    println!("3. `move` DOES NOT DECIDE THE TRAIT");
    let n = 10u32;
    let with_move = move || n * 2;
    let without = || n * 2;
    println!("  with_move() = {}, without() = {}", with_move(), without());
    println!("  Both are `Fn`. `move` copied the u32 into one and borrowed it");
    println!("  in the other, and neither body mutates or consumes anything --");
    println!("  so both sit on the top rung. What the BODY does decides the");
    println!("  trait; `move` decides where the capture lives.");
    println!();

    println!("4. AND ON A Copy TYPE IT IS ALMOST INVISIBLE");
    println!("  n is still usable after the `move` closure: {n}");
    println!("  Because u32 is Copy, `move` copied it rather than moving it --");
    println!("  which is why `move` on a counter you meant to share is a");
    println!("  classic silent bug: the closure gets its own copy and your");
    println!("  original never changes.");

    assert_eq!(make_greeter(String::from("x"))(), "hello, x");
    assert_eq!(with_move(), 20);
}
