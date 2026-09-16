//! The `ToOwned` docs page for 1.98.0, one block at a time, each claim run.
//!
//!   rustc --edition 2024 reading_the_to_owned_docs.rs -o /tmp/rtod && /tmp/rtod

use std::any::{type_name, type_name_of_val};
use std::borrow::Borrow;
use std::cell::Cell;
use std::ffi::OsStr;
use std::path::Path;

/// Reads the associated type off the trait for any `T`, sized or not.
fn owned_type<T: ToOwned + ?Sized>() -> &'static str {
    type_name::<T::Owned>()
}

/// Implements only the required method, so `clone_into` runs the provided
/// body. Deliberately not `Clone`: the blanket impl would already cover it,
/// and this impl would be E0119.
struct Ticket<'a> {
    seat: u32,
    copies_made: &'a Cell<u32>,
}

impl<'a> ToOwned for Ticket<'a> {
    type Owned = Ticket<'a>;

    fn to_owned(&self) -> Ticket<'a> {
        self.copies_made.set(self.copies_made.get() + 1);
        Ticket { seat: self.seat, copies_made: self.copies_made }
    }
}

fn main() {
    println!("1. The header: std::borrow, and no `use` line needed");
    let owned = "Ada".to_owned();
    println!("   \"Ada\".to_owned() = {owned:?}, with ToOwned never imported: it is in the prelude");

    println!();
    println!("2. Required associated type: `type Owned: Borrow<Self>`");
    println!("   <str  as ToOwned>::Owned = {}   <- a different type", owned_type::<str>());
    println!("   <u8   as ToOwned>::Owned = {}                      <- the blanket impl: Self", owned_type::<u8>());
    println!("   <&str as ToOwned>::Owned = {}                    <- a reference is Clone, so also Self", owned_type::<&str>());

    println!();
    println!("3. Required method: the page's own example, run");
    let s: &str = "a";
    let ss: String = s.to_owned();
    let v: &[i32] = &[1, 2];
    let vv: Vec<i32> = v.to_owned();
    println!("   ss = {ss:?}, vv = {vv:?}");

    println!();
    println!("4. Provided method: the page's own example, run");
    let mut s: String = String::new();
    "hello".clone_into(&mut s);
    let mut v: Vec<i32> = Vec::new();
    [1, 2][..].clone_into(&mut v);
    println!("   s = {s:?}, v = {v:?}");
    let copies_made = Cell::new(0);
    let front = Ticket { seat: 1, copies_made: &copies_made };
    let mut back = Ticket { seat: 40, copies_made: &copies_made };
    front.clone_into(&mut back);
    println!("   an impl that wrote only to_owned: clone_into called to_owned {} time, back.seat = {}", copies_made.get(), back.seat);

    println!();
    println!("5. Dyn compatibility: Borrow is, ToOwned is not");
    let name = String::from("Ada");
    let lenders: [&dyn Borrow<str>; 2] = [&name, &"Ben"];
    for lender in lenders {
        let lent: &str = lender.borrow();
        println!("   &dyn Borrow<str> lends {lent:?}");
    }
    println!("   &dyn ToOwned<Owned = String> is E0038");

    println!();
    println!("6. Implementors: each stable impl, called");
    println!("   str            {:<25} -> {}", "\"notes\".to_owned()", type_name_of_val(&"notes".to_owned()));
    println!("   [T] (T: Clone) {:<25} -> {}", "[1, 2][..].to_owned()", type_name_of_val(&[1, 2][..].to_owned()));
    println!("   Path           {:<25} -> {}", "Path::new(..).to_owned()", type_name_of_val(&Path::new("notes.txt").to_owned()));
    println!("   OsStr          {:<25} -> {}", "OsStr::new(..).to_owned()", type_name_of_val(&OsStr::new("notes.txt").to_owned()));
    println!("   CStr           {:<25} -> {}", "c\"notes\".to_owned()", type_name_of_val(&c"notes".to_owned()));
    println!("   T (T: Clone)   {:<25} -> {}", "42_u8.to_owned()", type_name_of_val(&42_u8.to_owned()));
}
