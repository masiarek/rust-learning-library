//! Kata solution: one function that measures BOTH halves of a reference —
//! the handle, and the value at the far end — for sized and unsized targets
//! alike. `&T` is always sized even when `T` is not, so `size_of::<&T>()`
//! is legal inside the generic and is what separates fat from thin.
//!
//! Run:  rustc --edition 2024 str_is_unsized_kata.rs && ./str_is_unsized_kata

use std::fmt::Display;

fn describe<T: ?Sized>(label: &str, value: &T) {
    let words = size_of::<&T>() / size_of::<usize>();
    let shape = if words == 2 { "fat " } else { "thin" };
    let unit = if words == 1 { "word" } else { "words" };
    println!("   {label:<13} {shape} pointer, {words} {unit} — the value is {} bytes",
             size_of_val(value));
}

fn main() {
    println!("Four references, one function:");
    describe("&str", "hello");
    describe("&[i32]", &[1, 2, 3][..]);
    describe("&dyn Display", &7i32 as &dyn Display);
    describe("&i32", &7i32);

    println!();
    println!("Remove the `?Sized` and only the last call still compiles:");
    println!("`i32` is the only one of the four targets whose size is in its type.");
}
