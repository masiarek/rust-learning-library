//! A struct with a field that borrows from another field of the same struct.
//! Safe Rust builds one in three ways, and none of them can be moved, returned
//! or mutated afterwards. A range instead of a reference has none of those
//! limits, so it comes first.
//!
//!   rustc --edition 2024 self_referential_structs.rs -o /tmp/srs && /tmp/srs

use std::cell::Cell;
use std::ops::Range;

/// The working answer: keep WHERE the word is, not a reference to it.
struct Sentence {
    text: String,
    first_word: Range<usize>,
}

impl Sentence {
    fn new(text: String) -> Sentence {
        let end = text.find(' ').unwrap_or(text.len());
        Sentence { text, first_word: 0..end }
    }

    fn first_word(&self) -> &str {
        &self.text[self.first_word.clone()]
    }
}

/// The shape from quinedot's guide: `borrowed` is meant to point into `owned`.
struct Snek<'a> {
    owned: String,
    borrowed: &'a str,
}

impl<'a> Snek<'a> {
    /// Ties the borrow to the struct's own lifetime. What it hands back is the
    /// only way left to reach the struct, and it has to be shared.
    fn bite(&'a mut self) -> &'a Snek<'a> {
        self.borrowed = &self.owned;
        self
    }

    fn points_into_itself(&self) -> bool {
        self.borrowed.as_ptr() == self.owned.as_ptr()
    }
}

/// The shared route: a `Cell` lets `&'a self` store the reference.
struct CellSnek<'a> {
    owned: String,
    borrowed: Cell<&'a str>,
}

impl<'a> CellSnek<'a> {
    fn bite(&'a self) {
        self.borrowed.set(&self.owned);
    }
}

fn make(text: &str) -> Sentence {
    Sentence::new(text.to_owned())
}

fn main() {
    println!("1. A range instead of a reference: returned, moved, mutated");
    let sentence = make("borrow it forever");
    let heap = sentence.text.as_ptr();
    let mut moved = vec![sentence];
    println!("   first word after a return and a move into a Vec: {:?}", moved[0].first_word());
    println!("   the text's heap bytes stayed put through the move: {}", moved[0].text.as_ptr() == heap);
    moved[0] = Sentence::new(String::from("pin it down"));
    println!("   replaced, and the range was recomputed: {:?}", moved[0].first_word());

    println!();
    println!("2. Built in place, with no &'a mut anywhere");
    let mut snek = Snek { owned: String::from("hiss"), borrowed: "" };
    snek.borrowed = &snek.owned;
    println!("   snek.borrowed = {:?}, snek.owned = {:?}", snek.borrowed, snek.owned);
    let by_ref = &snek;
    println!("   through & and a &self method, points into itself: {}", by_ref.points_into_itself());
    println!("   refused from here on: moving snek, &mut snek, snek.owned.push_str,");
    println!("   returning it from a function, and a Drop impl on Snek.");

    println!();
    println!("3. Built through bite(&'a mut self)");
    let mut bitten = Snek { owned: String::from("rattle"), borrowed: "" };
    let handle = bitten.bite();
    println!("   through the returned handle: {:?}, points into itself: {}", handle.borrowed, handle.points_into_itself());
    println!("   `bitten` itself may not be named again, not even to read a field.");

    println!();
    println!("4. Built through &'a self and a Cell");
    let cell_snek = CellSnek { owned: String::from("coil"), borrowed: Cell::new("") };
    cell_snek.bite();
    cell_snek.bite();
    println!("   bitten twice, still readable: owned {:?}, borrowed {:?}", cell_snek.owned, cell_snek.borrowed.get());
    println!("   shared access survives; moving it or taking &mut does not.");
}
