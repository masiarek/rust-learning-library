//! Appending to a `String`: push, push_str, +, format!, and the edits in between.
//!
//!   rustc --edition 2024 building_a_string.rs -o /tmp/bs && /tmp/bs

use std::fmt::Write as _;
use std::panic;

fn main() {
    println!("1. push_str takes a slice; push takes one char");
    let mut s = String::from("Hi");
    s.push_str(" Adam");
    println!("   after push_str(\" Adam\")   {s:?}");
    s.push('!');
    println!("   after push('!')            {s:?}");
    println!("   push_str(\"!\") would work too — the difference is the argument type,");
    println!("   not the effect: 'a' is a char, \"a\" is a &str. One is 4 bytes, the");
    println!("   other is a pointer and a length.");

    println!("\n2. + consumes its left operand");
    let a = String::from("equal ");
    let b = String::from("vote");
    let joined = a + &b;
    println!("   let joined = a + &b;   {joined:?}");
    println!("   `a` is MOVED into the result — the buffer is reused, not copied.");
    println!("   `b` is only borrowed, and is still usable: {b:?}");
    println!("   a + b would be E0308: expected `&str`, found `String`");

    println!("\n3. format! borrows everything");
    let c = String::from("equal ");
    let d = String::from("vote");
    let made = format!("{c}{d}");
    println!("   format!(\"{{c}}{{d}}\")   {made:?}");
    println!("   both still alive: {c:?} {d:?}");
    println!("   Cost: a fresh allocation. `+` reuses the left buffer, so a long chain");
    println!("   of `+` beats format! — and format! beats a chain you cannot read.");

    println!("\n4. write! appends without allocating a second buffer");
    let mut report = String::new();
    for (name, score) in [("Ada", 5), ("Ben", 2), ("Cara", 0)] {
        writeln!(report, "{name:<5} {score}").unwrap();
    }
    print!("{report}");
    println!("   (needs `use std::fmt::Write`; the Result is always Ok for a String)");

    println!("\n5. Editing in the middle");
    let mut e = String::from("hello world");
    e.insert(5, ',');
    println!("   insert(5, ',')       {e:?}");
    e.insert_str(0, ">> ");
    println!("   insert_str(0, \">> \") {e:?}");
    let popped = e.pop();
    println!("   pop()                {e:?}   returned {popped:?}");
    let removed = e.remove(0);
    println!("   remove(0)            {e:?}   returned {removed:?}");
    e.truncate(8);
    println!("   truncate(8)          {e:?}");
    e.clear();
    println!("   clear()              {e:?}   len {} capacity {}", e.len(), e.capacity());
    println!("   clear() keeps the buffer — that is why it is the cheap way to reuse one.");

    println!("\n6. The edits are byte-indexed too, so they can panic");
    let mut f = String::from("bête");
    println!("   {:?} is {} bytes", f, f.len());
    panic::set_hook(Box::new(|_| {}));
    let r = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let mut g = f.clone();
        g.truncate(2);
        g
    }));
    let _ = panic::take_hook();
    match r {
        Ok(g) => println!("   truncate(2) -> {g:?}"),
        Err(_) => println!("   truncate(2) PANICKED — byte 2 is inside 'ê'"),
    }
    f.truncate(3);
    println!("   truncate(3) -> {f:?}   <- 3 is a char boundary");

    println!("\n7. Pre-paying for the growth");
    let mut grown = String::new();
    let mut reallocs = 0;
    let mut last = grown.capacity();
    for _ in 0..64 {
        grown.push('x');
        if grown.capacity() != last {
            reallocs += 1;
            last = grown.capacity();
        }
    }
    let mut prepaid = String::with_capacity(64);
    let mut prepaid_reallocs = 0;
    let mut plast = prepaid.capacity();
    for _ in 0..64 {
        prepaid.push('x');
        if prepaid.capacity() != plast {
            prepaid_reallocs += 1;
            plast = prepaid.capacity();
        }
    }
    println!("   String::new()             64 pushes -> {reallocs} reallocation(s), capacity {last}");
    println!("   String::with_capacity(64) 64 pushes -> {prepaid_reallocs} reallocation(s), capacity {plast}");
}
