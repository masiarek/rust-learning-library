//! Exercise 3 of chapter 4: `str1` was declared inside a block and used after
//! it, E0425. Declare it where it is used, and return the `String` directly
//! rather than through a `let` (clippy's let_and_return, twice in the book).
//!
//!   rustc --edition 2024 pg4_03_let_outside_the_block.rs -o /tmp/pg403 && /tmp/pg403

fn main() {
    let str1 = generate_string();
    let str2 = str1;
    println!("{str2}"); // I will generate a string
}

fn generate_string() -> String {
    String::from("I will generate a string")
}
