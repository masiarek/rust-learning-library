//! There is no `Move` trait. Moving is the ground state of every type; `Copy`
//! is the opt-out that stops it. Nothing here opts *in* to being movable,
//! because there is nothing to opt in to.
//!
//!   rustc --edition 2024 no_move_trait.rs -o /tmp/nmt && /tmp/nmt

use std::mem::size_of;

/// One byte of data, and no derives beyond `Debug`.
#[derive(Debug)]
struct Moves(u8);

/// The same one byte, plus the opt-out.
#[derive(Debug, Clone, Copy)]
struct Copies(u8);

fn rule(title: &str) {
    println!("\n──── {title}");
}

fn main() {
    rule("A plain type moves, and no trait was consulted");
    let a = Moves(5);
    let b = a; // `a` is unusable from here on
    println!("  b = {b:?}, and b.0 = {}", b.0);
    println!("  `Moves` implements Debug and nothing else — and it still moved.");
    println!("  Reading `a` now is error[E0382]: borrow of moved value.");

    rule("`Copy` is the opt-out, and it changes what `=` means");
    let c = Copies(5);
    let d = c; // duplicated, not transferred
    println!("  c = {c:?}, d = {d:?}   both still usable (c.0 = {})", c.0);
    println!("  The only difference between the two types is the derive.");

    rule("The opt-out added nothing to the value");
    println!("  size_of::<Moves>()  = {}", size_of::<Moves>());
    println!("  size_of::<Copies>() = {}", size_of::<Copies>());
    println!("  Same bytes, same layout. `Copy` is a marker: it carries no data");
    println!("  and no code, it only tells the compiler what `let d = c;` does.");

    rule("`move` the KEYWORD is a capture mode, not a trait either");
    let owned = String::from("captured by value");
    let show = move || println!("  closure says: {owned}");
    show();
    println!("  `move` here answers 'borrow or take?' for the closure's captures.");
    println!("  It is not a bound, cannot appear after a `:`, and implements nothing.");

    rule("So the question 'is this type movable?' has no answer");
    println!("  Every type is. The real question is whether it is `Copy`,");
    println!("  and the compiler answers that one in the negative:");
    println!("  \"... which does not implement the `Copy` trait\".");
}
