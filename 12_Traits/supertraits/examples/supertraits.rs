//! A supertrait is a REQUIREMENT on the implementor, not inheritance.
//!
//!   rustc --edition 2024 supertraits.rs -o /tmp/st && /tmp/st

use std::fmt;

// `: fmt::Display` reads "anything implementing Shout must also implement
// Display". In exchange, Shout's own default body may USE Display.
trait Shout: fmt::Display {
    fn shout(&self) -> String {
        format!("{}!!!", self.to_string().to_uppercase())
    }
}

// Two supertraits, joined with `+`.
trait Broadcast: Shout + Clone {
    fn twice(&self) -> String {
        let echo = self.clone();
        format!("{} ... {}", self.shout(), echo.shout())
    }
}

#[derive(Clone)]
struct Dog;

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "woof")
    }
}

// Both impls are EMPTY: every method already has a default body, and the
// defaults are only allowed to exist because the supertraits guarantee
// `to_string` and `clone`.
impl Shout for Dog {}
impl Broadcast for Dog {}

fn main() {
    let dog = Dog;

    println!("1. The default body could call to_string() because Display is required");
    println!("   dog.shout()  = {}", dog.shout());
    println!("   dog.twice()  = {}", dog.twice());

    println!();
    println!("2. Both impl blocks are empty — `impl Shout for Dog {{}}`");
    println!("   The work was done by `impl Display for Dog`, which the supertrait");
    println!("   demanded. Leave that out and the impl is E0277, not E0599:");
    println!("   the trait bound `Dog: Display` is not satisfied.");

    println!();
    println!("3. A trait object carries the supertrait's methods too");
    let obj: &dyn Shout = &dog;
    println!("   as &dyn Shout, Display still works: {}", obj);
    println!("   ...and so does the trait's own method: {}", obj.shout());

    println!();
    println!("4. It is not inheritance: no fields, no override, two separate impls");
    println!("   size_of::<Dog>() = {}   Shout and Display added nothing to the value",
        std::mem::size_of::<Dog>());
    let as_display: &dyn fmt::Display = &dog;
    println!("   and the same value viewed as &dyn Display is a different vtable: {}", as_display);
}
