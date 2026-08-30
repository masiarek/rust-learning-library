//! `move`: where the captured values live, and what that costs.
//!
//!   rustc --edition 2024 the_move_keyword.rs -o /tmp/tmk && /tmp/tmk

use std::mem::size_of_val;
use std::thread;

struct Voter {
    name: String,
    ballot: Vec<u8>,
}

/// Needs `move`: the closure outlives `greeting`, which is a local here.
fn make_greeter(greeting: String) -> impl Fn(&str) -> String {
    move |who| format!("{greeting}, {who}")
}

fn main() {
    println!("1. Without `move`, a closure borrows what it reads");
    let name = String::from("Ada");
    let show = || println!("   borrowed: {name}");
    show();
    show();
    println!("   the original is still ours afterwards: {name:?}");
    println!("   size of that closure: {} bytes (a reference, not a String)", size_of_val(&show));

    println!();
    println!("2. With `move`, the closure takes the value");
    let owner = String::from("Ben");
    let show_owned = move || println!("   owned: {owner}");
    show_owned();
    show_owned();
    println!("   size of that closure: {} bytes (the String itself)", size_of_val(&show_owned));
    println!("   `owner` is no longer usable here — that use is E0382.");

    println!();
    println!("3. The trap: on a Copy type, `move` COPIES");
    let mut total = 10;
    let mut add = move |n: i32| {
        total += n;
        total
    };
    println!("   inside the closure:  {} then {}", add(1), add(1));
    println!("   outside it, total is still {total}");
    println!("   nothing was moved: i32 is Copy, so the closure got its own copy,");
    println!("   the outer `total` was never touched, and no error was raised.");
    println!("   This is the one `move` bug that compiles, runs, and does nothing.");

    println!();
    println!("4. A `move` closure captures the FIELDS it uses, not the whole value");
    let v = Voter {
        name: String::from("Cara"),
        ballot: vec![5, 3, 0],
    };
    let greet = move || println!("   voter: {}", v.name);
    greet();
    println!("   size of Voter:       {} bytes", size_of::<Voter>());
    println!("   size of the closure: {} bytes  (just the String field)", size_of_val(&greet));
    println!("   and the field it did not touch is still ours: {:?}", v.ballot);
    println!("   (edition 2021 changed this: before it, the whole `v` moved in.)");

    println!();
    println!("5. Two places `move` is not optional");
    let greeter = make_greeter(String::from("Hello"));
    println!("   returned closure:  {}", greeter("Ada"));
    println!("   without `move` that is E0373: the closure would outlive `greeting`.");

    let rows = vec![5u32, 3, 0, 4];
    let handle = thread::spawn(move || rows.iter().sum::<u32>());
    println!("   thread closure:    summed to {}", handle.join().unwrap());
    println!("   without `move` that is E0373 too, and the note names the reason:");
    println!("   `function requires argument type to outlive 'static`.");

    println!();
    println!("6. When the original has to survive: clone, then move the clone");
    let roster = vec![String::from("Ada"), String::from("Ben")];
    let for_thread = roster.clone();
    let handle = thread::spawn(move || for_thread.len());
    println!("   thread saw {} rows", handle.join().unwrap());
    println!("   and main still has its own: {roster:?}");
    println!("   the clone exists only to be moved. That is the idiom, and it is");
    println!("   also the first place a `.clone()` is worth questioning: cloning to");
    println!("   satisfy `move` copies the data, cloning an Rc copies a pointer.");
}
