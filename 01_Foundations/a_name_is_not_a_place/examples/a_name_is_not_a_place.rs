//! A name is not a place.
//!
//! `mut` writes into a place. A shadow builds a *second* place and moves the
//! name onto it, leaving the first one exactly as it was. That distinction is
//! not bookkeeping: a reference taken before the shadow keeps reading the old
//! place, so both values are live at once under one name — and the borrow
//! checker rejects the `mut` spelling of the same four lines.
//!
//!   rustc --edition 2024 a_name_is_not_a_place.rs -o /tmp/anip && /tmp/anip

struct Tracked(&'static str);

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("    drop: {}", self.0);
    }
}

fn banner(title: &str) {
    println!("\n──── {title}");
}

fn main() {
    banner("Two places, one name, both alive");

    let x = 5;
    let y = &x; //          y borrows the FIRST x
    let x = 6; //           a second place; the name moves onto it
    println!("  y = {y}   <- the place y borrowed, untouched");
    println!("  x = {x}   <- the place the name means now");

    banner("Not a `Copy` trick — the same thing with a String");

    let name = String::from("Ada");
    let seen = &name; //    borrows the first String
    let name = name.to_uppercase(); // a second String, in a second place
    println!("  seen = {seen}   <- still the first String, still owned");
    println!("  name = {name}   <- the name now means the new one");

    banner("The `mut` spelling of those four lines does not compile");

    println!("  let mut x = 5;");
    println!("  let y = &x;");
    println!("  x = 6;        <- error[E0506]: cannot assign to `x`");
    println!("                                 because it is borrowed");
    println!("  One place, and `y` is watching it. There is no second place");
    println!("  to put the 6 in, so the write has to be refused.");

    banner("So `mut` does not 'edit the value in place' either");

    {
        let mut slot = Tracked("first");
        println!("  mut:    slot holds {}", slot.0);
        slot = Tracked("second"); // the old value is dropped right here
        println!("  mut:    assigned; the drop above already happened");
        println!("  mut:    slot holds {}", slot.0);
    }

    println!();

    {
        let slot = Tracked("first");
        println!("  shadow: slot holds {}", slot.0);
        let slot = Tracked("second"); // nothing is dropped here
        println!("  shadow: shadowed; nothing has dropped");
        println!("  shadow: slot holds {}", slot.0);
    } // both die here, in reverse declaration order

    banner("Mutability belongs to the binding, not to the value");

    let s = String::from("Hello"); //   an immutable binding
    let mut s = s; //                   the SAME String, moved to a mutable one
    s.push_str(", world"); //           so the very same value is now mutable
    let s = s; //                       and frozen again, still the same value
    println!("  s = {s}");
    println!("  One String, three bindings, two answers to \"is it mutable?\"");
    println!("  The value never had an opinion; only the binding does.");

    banner("They are not opposites, so they combine");

    let weight = 60; //                     immutable i32
    let mut weight = weight as f32; //      new place, new type, and mutable
    weight += 1.5; //                       a write into that new place
    println!("  weight = {weight} kg");
}
