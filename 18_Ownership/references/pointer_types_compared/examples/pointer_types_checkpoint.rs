//! Checkpoint for the pointer-types table: three predictions, each one a
//! row of the table applied to a type the table does not list.
//!
//! Sizes are in words (units of `size_of::<usize>()`).
//!
//!   rustc --edition 2024 pointer_types_checkpoint.rs -o /tmp/pointer_types_checkpoint && /tmp/pointer_types_checkpoint

use std::mem::size_of;
use std::rc::Rc;

fn words<T>() -> usize {
    size_of::<T>() / size_of::<usize>()
}

fn main() {
    println!("1. Box<[u8; 64]>, in words");
    println!("   {}   (Box<[u8]> is {}: the length 64 is in the type, not in the pointer)", words::<Box<[u8; 64]>>(), words::<Box<[u8]>>());
    println!();

    println!("2. Option<Rc<str>>, in words");
    println!("   {}   (Rc<str> alone is {}: None costs nothing, since an Rc is never null)", words::<Option<Rc<str>>>(), words::<Rc<str>>());
    println!();

    println!("3. let mut a = Rc::new(1); let b = Rc::clone(&a); drop(b); Rc::get_mut(&mut a)?");
    let mut a = Rc::new(1);
    let b = Rc::clone(&a);
    drop(b);
    let count = Rc::strong_count(&a);
    println!("   {:?}   (the count is back to {count}, so a is unique again)", Rc::get_mut(&mut a));
}
