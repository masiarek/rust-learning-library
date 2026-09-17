//! The picture at the top of the page, measured: the String's own size
//! against the length and capacity of the text it owns.
//!
//!   rustc --edition 2024 measuring_the_three_words.rs -o /tmp/mttw && /tmp/mttw

use std::mem::{size_of, size_of_val};

fn main() {
    let s1 = String::from("Hello");

    // Size of the String struct itself (pointer + length + capacity)
    // = 24 bytes on a 64-bit system
    println!("size_of::<String>()    = {}", size_of::<String>());
    println!("size_of_val(&s1)       = {}", size_of_val(&s1));

    // Length of the string *contents* ("Hello")
    println!("s1.len()               = {}", s1.len());       // 5
    println!("s1.capacity()          = {}", s1.capacity());  // >= 5
}
