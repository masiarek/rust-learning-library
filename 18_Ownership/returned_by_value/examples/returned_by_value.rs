//! Returned by value: what comes back from a call, and why it needs a size.
//!
//! Every size is printed in words (one word = size_of::<usize>()) or in bytes
//! for types whose byte count is the same on every target, and every address
//! is a comparison -- so the answer key holds on a 32-bit target too.
//!
//!   rustc --edition 2024 returned_by_value.rs -o /tmp/rbv && /tmp/rbv

use std::mem::size_of;
use std::ptr;

const WORD: usize = size_of::<usize>();

fn words<T>() -> usize {
    size_of::<T>() / WORD
}

// Part 1: one function per row. The caller reserves size_of::<return type>()
// before each call, whatever the call will put there.
fn a_number() -> usize {
    7
}
fn a_borrowed_str() -> &'static str {
    "hello"
}
fn a_boxed_str() -> Box<str> {
    "hello".into()
}
fn a_string() -> String {
    String::from("hello")
}
fn a_thousand_bytes() -> [u8; 1000] {
    [7; 1000]
}

// Part 2: the same array handed back two ways.
fn by_value(a: &[i32; 3]) -> [i32; 3] {
    *a
}
fn by_reference(a: &[i32; 3]) -> &[i32; 3] {
    a
}

// Part 3: a String built inside the call, and where its text was built.
fn build(text_was_at: &mut *const u8) -> String {
    let s = String::from("hello");
    *text_was_at = s.as_ptr();
    s
}

fn main() {
    println!("1. What comes back from each signature");
    let n = a_number();
    let b = a_borrowed_str();
    let x = a_boxed_str();
    let s = a_string();
    let t = a_thousand_bytes();
    println!("   -> usize       {} word      the number itself: {n}", words::<usize>());
    println!("   -> &str        {} words     an address and a length: {b:?}", words::<&str>());
    println!("   -> Box<str>    {} words     an address and a length: {x:?}", words::<Box<str>>());
    println!("   -> String      {} words     address, capacity, length: {s:?}", words::<String>());
    println!("   -> [u8; 1000]  {} bytes  every byte comes back: t[999] = {}", size_of::<[u8; 1000]>(), t[999]);
    println!("   -> str         no size     size_of_val(\"hi\") = {}, size_of_val(\"hello\") = {}",
             size_of_val("hi"), size_of_val("hello"));

    println!();
    println!("2. By value hands back a copy; by reference hands back the address");
    let original = [1, 2, 3];
    let mut copy = by_value(&original);
    let same = by_reference(&original);
    copy[0] = 100;
    println!("   by value:     the copy is somewhere else: {}", !ptr::eq(&copy, &original));
    println!("                 copy = {copy:?}, original still {original:?}");
    println!("   by reference: it points at the original: {}", ptr::eq(same, &original));

    println!();
    println!("3. A returned String brings back its header, not its text");
    let mut text_was_at = ptr::null();
    let greeting = build(&mut text_was_at);
    println!("   the text is where build() put it: {}", greeting.as_ptr() == text_was_at);

    println!();
    println!("4. Clone returns Self by value, so a str cannot be Clone");
    let name: &str = "hello";
    #[allow(noop_method_call)]
    let cloned = name.clone();
    let owned = name.to_owned();
    println!("   name.clone()    : a &str, same address and length: {}", ptr::eq(cloned, name));
    println!("   name.to_owned() : a String, {} words, text copied: {}",
             words::<String>(), owned.as_ptr() != name.as_ptr());
}
