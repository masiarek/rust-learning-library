//! Kata solution: predict copy or move, then ask the compiler. Each value is
//! bound at the type written beside it, so the type column is checked too.
//!
//!   rustc --edition 2024 copy_or_move_kata.rs -o /tmp/comk && /tmp/comk

use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::Range;
use std::rc::Rc;

/// The probe from the page: the inherent `verdict` applies only where `T: Copy`.
struct Probe<T>(PhantomData<T>);

trait Moves {
    fn verdict(&self) -> &'static str {
        "move"
    }
}

impl<T> Moves for Probe<T> {}

impl<T: Copy> Probe<T> {
    fn verdict(&self) -> &'static str {
        "copy"
    }
}

fn probe<T>(_: &T) -> Probe<T> {
    Probe(PhantomData)
}

macro_rules! row {
    ($value:expr => $ty:ty) => {{
        let a: $ty = $value;
        println!("  {:<26} {:<16} {}", stringify!($value), stringify!($ty), probe(&a).verdict());
    }};
}

fn main() {
    println!("  {:<26} {:<16} {}", "let a = …;", "type", "let b = a;");
    row!(Some(5) => Option<i32>);
    row!(Some(String::from("hi")) => Option<String>);
    row!(&[1, 2, 3][..] => &[i32]);
    row!(1..3 => Range<i32>);
    row!(Rc::new(5) => Rc<i32>);
    row!(Ordering::Less => Ordering);

    println!();
    println!("Option<T> copies exactly when T does: the enum adds a tag, not an owner.");
    println!("&[i32] is a shared reference, so it copies like every &T.");
    println!("Rc<i32> moves: it has a Drop that lowers the count, and a type with a");
    println!("destructor can never be Copy. A second handle comes from Rc::clone.");

    println!();
    println!("Range<i32> is the surprise: two i32s, and it still moves, because it is");
    println!("an Iterator itself. If it were Copy, it.take(2) would take a silent copy,");
    println!("and `it` would not advance. Written out with clone(), that looks like this:");

    let mut it = 1..6;
    let first: Vec<i32> = it.by_ref().take(2).collect();
    let rest: Vec<i32> = it.collect();
    println!("  it.by_ref().take(2): first = {first:?}, rest = {rest:?}");

    let it = 1..6;
    let first: Vec<i32> = it.clone().take(2).collect();
    let rest: Vec<i32> = it.collect();
    println!("  it.clone().take(2):  first = {first:?}, rest = {rest:?}");

    println!();
    println!("RFC 3550 records that Copy was removed from every iterator in 2015. Its");
    println!("core::range::Range is Copy because it is only IntoIterator, but on 1.98");
    println!("it is nightly-only, and 1..3 still builds std::ops::Range.");
}
