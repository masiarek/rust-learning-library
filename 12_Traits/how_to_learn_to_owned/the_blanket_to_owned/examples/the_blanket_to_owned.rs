//! Step 6 of the `ToOwned` path: `impl<T: Clone> ToOwned for T` gives
//! `to_owned()` to every `Clone` type — and every `&T` is one of them.
//!
//!   rustc --edition 2024 the_blanket_to_owned.rs -o /tmp/tbto && /tmp/tbto

use std::any::{type_name, type_name_of_val};

/// Deliberately not `Clone`.
struct Ticket {
    seat: u32,
}

/// The same struct with `Clone` derived.
#[derive(Clone)]
struct Seat {
    seat: u32,
}

fn main() {
    println!("For a Clone type, Owned is the type itself");
    println!("   <i32 as ToOwned>::Owned      = {}", type_name::<<i32 as ToOwned>::Owned>());
    println!("   <String as ToOwned>::Owned   = {}", type_name::<<String as ToOwned>::Owned>());
    println!("   <Vec<u8> as ToOwned>::Owned  = {}", type_name::<<Vec<u8> as ToOwned>::Owned>());

    println!();
    println!("Checkpoint. What is 42_i32.to_owned()? Is (&ticket).to_owned() a new ticket?");
    let n = 42_i32.to_owned();
    println!("   42_i32.to_owned() -> {} {n}", type_name_of_val(&n));
    let ticket = Ticket { seat: 12 };
    let a = &ticket;
    let same = a.to_owned();
    println!(
        "   a.to_owned() is the same address as a: {}   (it is a &Ticket, seat {})",
        std::ptr::eq(a, same),
        same.seat
    );

    println!();
    println!("Derive Clone, and the same call on the same kind of reference copies the value");
    let seat = Seat { seat: 12 };
    let b = &seat;
    let copy: Seat = b.to_owned();
    println!(
        "   b.to_owned() is the same address as b: {}   (it is a Seat, seat {})",
        std::ptr::eq(b, &copy),
        copy.seat
    );
}
