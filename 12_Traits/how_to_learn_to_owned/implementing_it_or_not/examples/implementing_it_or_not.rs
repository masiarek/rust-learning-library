//! Step 10 of the `ToOwned` path: a `Clone` type already has `ToOwned` from the
//! blanket impl, so a view struct gets an inherent method instead — and which
//! one a call reaches depends on how it is spelled.
//!
//!   rustc --edition 2024 implementing_it_or_not.rs -o /tmp/iion && /tmp/iion

/// A view that borrows its text and derives `Clone`.
#[derive(Clone)]
struct NameRef<'a> {
    first: &'a str,
}

/// Its owned twin.
struct Name {
    first: String,
}

impl NameRef<'_> {
    /// An inherent method named like the trait's. `impl ToOwned for NameRef`
    /// would be E0119: the blanket impl already covers every `Clone` type.
    fn to_owned(&self) -> Name {
        Name { first: self.first.to_owned() }
    }
}

fn main() {
    println!("Checkpoint. NameRef derives Clone and has an inherent to_owned. What comes back?");
    let owner = String::from("Ada");
    let view = NameRef { first: &owner };
    let mine: Name = view.to_owned();
    let blanket: NameRef<'_> = ToOwned::to_owned(&view);
    println!("   view.to_owned()          -> Name    {{ first: {:?} }}   the inherent method", mine.first);
    println!("   ToOwned::to_owned(&view) -> NameRef {{ first: {:?} }}   the blanket impl", blanket.first);

    println!();
    println!("Only the inherent method's result outlives the text it was made from");
    let kept: Name = {
        let temporary = String::from("Grace");
        NameRef { first: &temporary }.to_owned()
    };
    println!("   kept.first = {:?}, its own buffer", kept.first);
}
