//! Kata solution: a smart pointer that counts how often it is dereferenced.
//!
//!   rustc --edition 2024 what_makes_a_pointer_smart_kata.rs -o /tmp/wmpsk && /tmp/wmpsk

use std::cell::Cell;
use std::ops::Deref;

/// Owns a value and counts every `deref`. `deref` takes `&self`, so the
/// counter needs a `Cell`: a write through a shared reference.
struct Counted<T> {
    value: Box<T>,
    reads: Cell<usize>,
}

impl<T> Counted<T> {
    fn new(value: T) -> Self {
        Counted { value: Box::new(value), reads: Cell::new(0) }
    }

    /// An associated function, not a method, so that calling it cannot
    /// itself go through `deref` — the same choice std makes for
    /// `Rc::strong_count(&rc)`.
    fn reads(this: &Self) -> usize {
        this.reads.get()
    }
}

impl<T> Deref for Counted<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.reads.set(self.reads.get() + 1);
        &self.value
    }
}

fn takes_str(text: &str) -> usize {
    text.len()
}

fn main() {
    let name = Counted::new(String::from("ferris"));
    let mut seen = Vec::new();

    let _ = name.len(); //              method search: one deref, Counted -> String
    seen.push(("name.len()", Counted::reads(&name)));

    let _ = name.to_uppercase(); //     Counted -> String (by deref) -> str (by String's own Deref)
    seen.push(("name.to_uppercase()", Counted::reads(&name)));

    let _ = takes_str(&name); //         coercion: &Counted<String> -> &String -> &str
    seen.push(("takes_str(&name)", Counted::reads(&name)));

    let _ = &*name; //                   explicit
    seen.push(("&*name", Counted::reads(&name)));

    let _ = &name; //                    a reference to the pointer itself: no deref
    seen.push(("&name", Counted::reads(&name)));

    println!("Reads counted after each line:");
    for (line, total) in seen {
        println!("  {line:<22} {total}");
    }
    println!();
    println!("Each line that reads through the pointer added one call to deref, and");
    println!("`&name`, which only borrows the pointer, added none. The second hop in");
    println!("to_uppercase and takes_str, String -> str, is String's Deref, not ours.");
}
