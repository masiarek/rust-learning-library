//! After the ten `ToOwned` steps: `Clone`, `ToOwned` and `From` side by side —
//! what each call takes, what it gives back, and whether the text moved to a
//! new buffer, read off the addresses rather than assumed.
//!
//!   rustc --edition 2024 clone_to_owned_or_from.rs -o /tmp/ctoof && /tmp/ctoof

use std::any::type_name_of_val;
use std::rc::Rc;

fn main() {
    println!("Clone: &T -> T, always the same type");
    let s = String::from("Ada");
    let cloned = s.clone();
    println!("   s.clone()             &String -> {:<24} new buffer: {}", type_name_of_val(&cloned), cloned.as_ptr() != s.as_ptr());
    let n = 42_i32;
    println!("   n.clone()             &i32    -> {:<24} no heap to copy", type_name_of_val(&n.clone()));

    println!();
    println!("ToOwned: &B -> B::Owned, the same type or another one");
    let literal: &str = "Ada";
    let owned = literal.to_owned();
    println!("   literal.to_owned()    &str    -> {:<24} new buffer: {}", type_name_of_val(&owned), owned.as_ptr() != literal.as_ptr());
    println!("   n.to_owned()          &i32    -> {:<24} no heap to copy", type_name_of_val(&n.to_owned()));
    let rc = Rc::new(String::from("Ada"));
    let rc2 = rc.to_owned();
    println!(
        "   rc.to_owned()         &Rc     -> {:<24} text copied: {}, strong count: {}",
        "Rc<String>, the same one",
        !Rc::ptr_eq(&rc, &rc2),
        Rc::strong_count(&rc)
    );

    println!();
    println!("From / Into: T -> U, and the T is gone afterwards");
    let from_str = String::from(literal);
    println!("   String::from(literal) &str    -> {:<24} new buffer: {}", type_name_of_val(&from_str), from_str.as_ptr() != literal.as_ptr());
    let text = String::from("Ada");
    let before = text.as_ptr();
    let bytes: Vec<u8> = Vec::from(text);
    println!("   Vec::from(text)       String  -> {:<24} new buffer: {}", type_name_of_val(&bytes), bytes.as_ptr() != before);
    let again = String::from("Ada");
    let before = again.as_ptr();
    let shared: Rc<str> = again.into();
    println!(
        "   again.into()          String  -> {:<24} new buffer: {}   (the counts need room)",
        type_name_of_val(&shared),
        shared.as_ptr() != before
    );
}
