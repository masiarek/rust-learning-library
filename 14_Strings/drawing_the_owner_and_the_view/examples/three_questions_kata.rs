//! Kata solution: three questions answered from the drawing, then checked.
//!
//!   rustc --edition 2024 three_questions_kata.rs -o /tmp/tqk && /tmp/tqk

use std::panic;

/// Where `view` starts, in bytes from the start of `owner`'s buffer.
fn offset_in(owner: &str, view: &str) -> Option<usize> {
    let buffer = owner.as_bytes().as_ptr_range();
    let start = view.as_ptr();
    buffer.contains(&start).then(|| start.addr() - buffer.start.addr())
}

/// The same compiled code for every call: two numbers and a slice.
fn cut(s: &str, start: usize, end: usize) -> &str {
    &s[start..end]
}

fn main() {
    let s = String::from("héllo world");

    println!("1. Why does a &str carry a length?");
    let one = &s[..1];
    let six = &s[..6];
    for (label, view) in [("&s[..1]", one), ("&s[..6]", six)] {
        let shown = format!("{view:?}");
        println!("   {label} = {shown:<8} offset {:?}   len {}", offset_in(&s, view), view.len());
    }
    println!("   Same pointer, different text. The pointer says where a view starts;");
    println!("   only the length says where it stops. A C char * has no length word,");
    println!("   so strlen walks the bytes until it finds a 0.");

    println!();
    println!("2. After `let t = s;`, which bytes moved?");
    let buffer = s.as_ptr();
    let t = s;
    println!("   buffer address changed: {}", t.as_ptr() != buffer);
    println!("   The three words were copied into t and s stopped being usable.");
    println!("   The 12 bytes on the heap stayed where they were.");

    println!();
    println!("3. Why is &s[1..2] on \"héllo\" a panic and not a compile error?");
    panic::set_hook(Box::new(|_| {}));
    for text in ["hello", "héllo"] {
        match panic::catch_unwind(|| cut(text, 1, 2).to_owned()) {
            Ok(piece) => println!("   cut({text:?}, 1, 2) = {piece:?}"),
            Err(payload) => {
                let message = payload.downcast_ref::<String>().map_or("?", |m| m.as_str());
                println!("   cut({text:?}, 1, 2) panicked: {message}");
            }
        }
    }
    let _ = panic::take_hook();
    println!("   One function, compiled once. Its bounds are two usize values, and");
    println!("   whether byte 2 starts a character depends on the bytes, which exist");
    println!("   only when the program runs.");
    println!("   \"héllo\".get(1..2) = {:?}   the same question, without the panic", "héllo".get(1..2));
}
