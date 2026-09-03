//! Method resolution: `x.f()` derefs and borrows until something named `f` fits.
//!
//!   rustc --edition 2024 method_resolution.rs -o /tmp/mr && /tmp/mr

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

struct Tally(Vec<i32>);

impl Deref for Tally {
    type Target = Vec<i32>;
    fn deref(&self) -> &Vec<i32> {
        &self.0
    }
}

impl Tally {
    /// An INHERENT method added long after the `Deref` impl. It silently takes
    /// over every `.len()` call on a `Tally`, and nothing warns.
    fn len(&self) -> String {
        format!("{} entries", self.0.len())
    }
}

fn main() {
    println!("1. The receiver you did not write: `&` is inserted for you");
    let seats = vec![3, 1, 2];
    println!("   seats.len()                 -> {}", seats.len());
    println!("   the method takes &self, so the receiver was `&seats`");

    println!();
    println!("2. Deref first, then borrow — `String` has no `len`, `str` does");
    let name = String::from("Ada");
    println!("   name.len()                  -> {}", name.len());
    println!("   name.to_uppercase()         -> {}", name.to_uppercase());
    println!("   both live on `str`, one deref down");

    println!();
    println!("3. The ladder is walked as far as it goes: Vec -> [i32]");
    let total: i32 = seats.iter().sum();
    println!("   seats.iter().sum()          -> {total}   (`iter` is a slice method)");
    println!("   seats.first()               -> {:?}", seats.first());

    println!();
    println!("4. Extra references cost nothing — they are peeled off first");
    let r = &&&name;
    println!("   (&&&name).len()             -> {}", r.len());

    println!();
    println!("5. The trap: an inherent method on a Deref wrapper shadows the target's");
    let t = Tally(vec![3, 1, 2]);
    println!("   t.len()      -> {:<12} <- Tally::len, and it returns a String", format!("{:?}", t.len()));
    println!("   (*t).len()   -> {:<12} <- Vec::len, reached by dereferencing first", format!("{:?}", (*t).len()));
    println!("   Vec::len(&t) -> {:<12} <- the same method, named instead of found", format!("{:?}", Vec::len(&t)));
    println!("   t.first()    -> {:<12} <- unshadowed, so it still reaches the slice", format!("{:?}", t.first()));

    println!();
    println!("6. The ladder crosses as many smart pointers as it needs");
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    shared.borrow_mut().push(4);
    println!("   Rc -> RefCell -> Vec        -> {:?}", shared.borrow());
}
