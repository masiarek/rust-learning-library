//! Step 4 of the `ToOwned` path: `Clone` hands back `Self`, and every shared
//! reference `&T` is `Copy` — and therefore `Clone` — whether or not `T` is.
//!
//!   rustc --edition 2024 clone_returns_self.rs -o /tmp/crs && /tmp/crs

use std::any::type_name_of_val;

/// Deliberately not `Clone`.
struct Ticket {
    seat: u32,
}

fn main() {
    println!("Clone::clone takes &Self and gives back Self");
    let s = String::from("Ada");
    let copy = Clone::clone(&s);
    println!(
        "   Clone::clone(&s)  s: String -> {}, new buffer: {}",
        type_name_of_val(&copy),
        copy.as_ptr() != s.as_ptr()
    );

    println!();
    println!("Checkpoint. Ticket is not Clone. After `let b = a;` is `a` still usable?");
    let ticket = Ticket { seat: 12 };
    let a = &ticket;
    let b = a;
    println!("   yes: a.seat = {}, b.seat = {}   (&Ticket is Copy, so `let b = a` copied it)", a.seat, b.seat);

    println!();
    println!("And Clone on the reference hands back the reference");
    let c = Clone::clone(&a);
    println!("   Clone::clone(&a)  a: &Ticket -> &Ticket, same address as a: {}", std::ptr::eq(a, c));
}
