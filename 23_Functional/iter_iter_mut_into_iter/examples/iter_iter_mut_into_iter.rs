//! Three doors onto the same collection: &T, &mut T, and T.
//!
//!   rustc --edition 2024 iter_iter_mut_into_iter.rs -o /tmp/iimii && /tmp/iimii

use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

/// A String wearing a counter, so "this cloned the data" is measured rather
/// than asserted.
#[derive(Debug, PartialEq)]
struct Name(String);

static CLONES: AtomicUsize = AtomicUsize::new(0);

impl Clone for Name {
    fn clone(&self) -> Self {
        CLONES.fetch_add(1, Relaxed);
        Name(self.0.clone())
    }
}

impl Name {
    fn new(s: &str) -> Self {
        Name(s.to_string())
    }
}

fn roster() -> Vec<Name> {
    vec![Name::new("Ada"), Name::new("Ben"), Name::new("Cara")]
}

fn main() {
    println!("1. The same Vec, three ways");
    let mut names = roster();

    for n in names.iter() {
        print!("   iter()      yields &Name     {:?}", n);
        println!();
        break;
    }
    for n in names.iter_mut() {
        n.0.push('!');
    }
    println!("   iter_mut()  yields &mut Name  and the change landed: {:?}", names[0]);
    let owned: Vec<String> = names.into_iter().map(|n| n.0).collect();
    println!("   into_iter() yields Name       taken apart into: {owned:?}");
    println!("   `names` is gone after into_iter — it was consumed, not borrowed.");

    println!();
    println!("2. A `for` loop is one of those three, chosen by what you wrote");
    let mut names = roster();
    println!("   for n in &names        =>  names.iter()       item: &Name");
    for n in &names {
        print!("     {:?}", n.0);
    }
    println!();
    println!("   for n in &mut names    =>  names.iter_mut()   item: &mut Name");
    for n in &mut names {
        n.0 = n.0.to_uppercase();
    }
    println!("     {:?}", names);
    println!("   for n in names         =>  names.into_iter()  item: Name  (moves it)");
    for n in names {
        print!("     {:?}", n.0);
    }
    println!();
    println!("   `for` needs IntoIterator, not Iterator, and Vec implements it three");
    println!("   times: for Vec<T>, for &Vec<T>, and for &mut Vec<T>.");

    println!();
    println!("3. Which is why the beginner's E0382 happens where it does");
    println!("   for n in names {{ .. }}   then using `names` afterwards is:");
    println!("     error[E0382]: borrow of moved value: `names`");
    println!("     `names` moved due to this implicit call to `.into_iter()`");
    println!("     help: consider iterating over a slice of the `Vec<Name>`'s content");
    println!("   The fix rustc suggests is one character: `for n in &names`.");

    println!();
    println!("4. Arrays are the exception worth memorising");
    let scores = [5u8, 3, 0];
    let by_value: Vec<u8> = scores.into_iter().collect();
    let by_ref: Vec<&u8> = scores.iter().collect();
    println!("   [u8; 3].into_iter()  -> items are u8   {by_value:?}");
    println!("   [u8; 3].iter()       -> items are &u8  {by_ref:?}");
    println!("   ...and `scores` is still usable: {scores:?}  (u8 is Copy)");
    println!("   Before edition 2021, array.into_iter() yielded REFERENCES — the");
    println!("   method resolved through the slice, because arrays had no by-value");
    println!("   IntoIterator. Old answers on the internet still assume that.");

    println!();
    println!("5. Getting from &T to T: copied, cloned, and what each costs");
    let names = roster();
    CLONES.store(0, Relaxed);
    let borrowed: Vec<&Name> = names.iter().collect();
    println!("   .iter().collect::<Vec<&Name>>()   {} clone(s)  {:?}",
             CLONES.load(Relaxed), borrowed.len());

    CLONES.store(0, Relaxed);
    let cloned: Vec<Name> = names.iter().cloned().collect();
    println!("   .iter().cloned().collect()        {} clone(s)  {:?}",
             CLONES.load(Relaxed), cloned.len());

    CLONES.store(0, Relaxed);
    let moved: Vec<Name> = names.into_iter().collect();
    println!("   .into_iter().collect()            {} clone(s)  {:?}",
             CLONES.load(Relaxed), moved.len());
    println!("   All three produce a collection of three. Only the middle one paid");
    println!("   for it — and it paid because `iter()` was the wrong door to start");
    println!("   from, not because collecting is expensive.");

    let scores = [5i32, 3, 0];
    let total: i32 = scores.iter().copied().sum();
    println!("   .copied() is the same adapter for Copy types: sum = {total}");

    println!();
    println!("6. Choosing, in one line each");
    println!("   read it            -> iter()       &T");
    println!("   change it in place -> iter_mut()   &mut T");
    println!("   take it apart      -> into_iter()  T   (the collection is spent)");
    println!("   Reaching for .clone() to make a borrow compile usually means the");
    println!("   answer was into_iter() one line earlier.");
}
