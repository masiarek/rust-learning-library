//! Step 7 of the `ToOwned` path: on an `Rc` or a `Cow`, `.to_owned()` finds the
//! outer type's `Clone` first — a pointer and a count, or a `Cow` in the same
//! variant — and the calls that do what was meant.
//!
//!   rustc --edition 2024 to_owned_traps.rs -o /tmp/tot && /tmp/tot

use std::any::type_name_of_val;
use std::borrow::Cow;
use std::rc::Rc;

fn variant(cow: &Cow<'_, str>) -> &'static str {
    match cow {
        Cow::Borrowed(_) => "Cow::Borrowed",
        Cow::Owned(_) => "Cow::Owned",
    }
}

fn main() {
    println!("Checkpoint. Strong count after Rc::to_owned? Variant after Cow::to_owned?");
    let shared = Rc::new(String::from("ballot"));
    let again = shared.to_owned();
    println!(
        "   Rc::strong_count = {}, same allocation: {}",
        Rc::strong_count(&shared),
        Rc::ptr_eq(&shared, &again)
    );
    let cow: Cow<'_, str> = Cow::Borrowed("ballot");
    println!("   Cow::Borrowed(..).to_owned() -> {}", variant(&cow.to_owned()));

    println!();
    println!("What was meant: name the inner type, or ask the Cow for its owned form");
    let text: String = (*shared).to_owned();
    println!(
        "   (*shared).to_owned() -> {}, same bytes as the Rc's: {}",
        type_name_of_val(&text),
        text.as_ptr() == shared.as_ptr()
    );
    let owned: String = cow.into_owned();
    println!("   cow.into_owned()     -> {} {owned:?}", type_name_of_val(&owned));
}
