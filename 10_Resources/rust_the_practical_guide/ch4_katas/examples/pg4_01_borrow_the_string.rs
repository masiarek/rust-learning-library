//! Exercise 1 of chapter 4: `some_function(s1, s2)` moved `s1` into the call,
//! so the `println!` after it is E0382. Lend it instead — and take `&str`,
//! which a `&String` becomes at the call site by deref coercion.
//!
//!   rustc --edition 2024 pg4_01_borrow_the_string.rs -o /tmp/pg401 && /tmp/pg401

fn main() {
    let s1: String = String::from("this is me, ");
    let s2: &str = "Nouman";
    some_function(&s1, s2);
    println!("{} {}", s1, s2); // this is me,  Nouman
}

fn some_function(a1: &str, a2: &str) {
    println!("{} {}", a1, a2); // this is me,  Nouman
}
