//! Kata: say which impl the hover lands on for each `.clone()`, and what type
//! comes back, before running this.
//!
//!   rustc --edition 2024 reading_the_clone_hover_kata.rs -o /tmp/rtchk && /tmp/rtchk

use std::any::type_name_of_val;
use std::rc::Rc;

// Three of the five calls draw a warning, which is part of the answer: two
// noop_method_call and one suspicious_double_ref_op. std's own doc example
// hides an allow line for the first of them.
#[allow(noop_method_call, suspicious_double_ref_op)]
fn main() {
    let owned = String::from("Hello");
    let text: &str = "Hello";
    let words: &String = &owned;
    let twice: &&String = &words;
    let counted: Rc<str> = Rc::from("Hello");
    let numbers: &[i32] = &[1, 2, 3];

    let a = text.clone();
    let b = words.clone();
    let c = twice.clone();
    let d = counted.clone();
    let e = numbers.clone();

    println!("text.clone()    {:<24} impl Clone for &T, T = str", type_name_of_val(&a));
    println!("words.clone()   {:<24} impl Clone for String", type_name_of_val(&b));
    println!("twice.clone()   {:<24} impl Clone for &T, T = String", type_name_of_val(&c));
    println!("counted.clone() {:<24} impl Clone for Rc<T, A>, strong_count now {}", type_name_of_val(&d), Rc::strong_count(&counted));
    println!("numbers.clone() {:<24} impl Clone for &T, T = [i32]", type_name_of_val(&e));
}
