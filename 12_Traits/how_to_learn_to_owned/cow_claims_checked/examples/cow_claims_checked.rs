//! Claims that explanations of `Cow` repeat, each one run. The two that do not
//! compile at all (`into_borrowed`, and a `Cow<dyn Display>`) are on the page
//! with rustc's own words, since a program that does not build prints nothing.
//!
//!   rustc --edition 2024 cow_claims_checked.rs -o /tmp/ccc && /tmp/ccc

use std::any::type_name_of_val;
use std::borrow::Cow;
use std::rc::Rc;

fn variant<B: ?Sized + ToOwned>(cow: &Cow<'_, B>) -> &'static str {
    match cow {
        Cow::Borrowed(_) => "Borrowed",
        Cow::Owned(_) => "Owned",
    }
}

/// The version that circulates: slices the first byte, and always formats.
fn capitalize_common(msg: &str) -> Cow<'_, str> {
    if let Some(letter) = msg.get(0..1) {
        format!("{}{}", letter.to_uppercase(), &msg[1..]).into()
    } else {
        msg.into()
    }
}

/// Allocates only when the first character really is lowercase, whatever its length in bytes.
fn capitalize(msg: &str) -> Cow<'_, str> {
    match msg.chars().next() {
        Some(first) if first.is_lowercase() => {
            let rest = &msg[first.len_utf8()..];
            Cow::Owned(first.to_uppercase().chain(rest.chars()).collect())
        }
        _ => Cow::Borrowed(msg),
    }
}

fn main() {
    println!("1. \"to_mut() gives you a &mut str\"");
    let mut cow: Cow<'_, str> = Cow::Borrowed("ballot");
    println!("   to_mut() returns {}: the Owned type, not B", type_name_of_val(&cow.to_mut()));

    println!();
    println!("2. \"calling replace or to_uppercase on a borrowed Cow clones it into Owned\"");
    let borrowed: Cow<'_, str> = Cow::Borrowed("Hello, Rust!");
    let replaced = borrowed.replace("Rust", "World");
    println!("   replace returned {}, and the Cow is still {}", type_name_of_val(&replaced), variant(&borrowed));

    println!();
    println!("3. \"as_ref() returns a Cow\"");
    let as_ref: &str = borrowed.as_ref();
    println!("   as_ref() returns {}", type_name_of_val(&as_ref));

    println!();
    println!("4. \"Owned means the data was cloned\"");
    let made: Cow<'static, str> = format!("file {} not found", 7).into();
    let fixed: Cow<'static, str> = "out of memory".into();
    println!("   String.into() -> {}, nothing cloned; &'static str.into() -> {}", variant(&made), variant(&fixed));

    println!();
    println!("5. \"?Sized means B has to be unsized\" / \"Cow has a size cost\"");
    let number: Cow<'_, i32> = Cow::Borrowed(&42);
    println!("   Cow<i32> compiles: {} {}", variant(&number), *number);
    let word = size_of::<usize>();
    println!(
        "   in words: Cow<str> {}, String {}, &str {}; Cow<i32> {}, &i32 {}",
        size_of::<Cow<'_, str>>() / word,
        size_of::<String>() / word,
        size_of::<&str>() / word,
        size_of::<Cow<'_, i32>>() / word,
        size_of::<&i32>() / word
    );

    println!();
    println!("6. \"+= on a Cow<str> appends without allocating\"");
    let mut name: Cow<'static, str> = "jimb".into();
    let before = variant(&name);
    name += ", Esq.";
    println!("   before: {before}, after +=: {} {name:?}", variant(&name));

    println!();
    println!("7. \"a capitalize that returns Cow only allocates when it has to\"");
    for msg in ["ada", "Ada", "élan", ""] {
        let common = capitalize_common(msg);
        let fixed = capitalize(msg);
        println!(
            "   {msg:<7} common: {:<8} {:<9} fixed: {:<8} {:?}",
            variant(&common),
            format!("{common:?}"),
            variant(&fixed),
            fixed
        );
    }

    println!();
    println!("8. Rc::make_mut is clone-on-write too: it copies only when the value is shared");
    let mut shared = Rc::new(String::from("roster"));
    let other = Rc::clone(&shared);
    Rc::make_mut(&mut shared).push('!');
    println!("   shared: now separate allocations: {}, {shared:?} and {other:?}", !Rc::ptr_eq(&shared, &other));
    let mut alone = Rc::new(String::from("roster"));
    let start = Rc::as_ptr(&alone);
    Rc::make_mut(&mut alone).push('!');
    println!("   alone:  same allocation, nothing copied: {}", Rc::as_ptr(&alone) == start);
}
