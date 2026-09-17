//! The claims on "ToOwned in real code" that a program can check without
//! pulling in any crate: what the std excerpts do, run on the pinned compiler.
//!
//!   rustc --edition 2024 to_owned_in_real_code.rs -o /tmp/toirc && /tmp/toirc

use std::borrow::{Borrow, Cow};
use std::ffi::{CStr, CString};

fn variant(cow: &Cow<'_, str>) -> &'static str {
    match cow {
        Cow::Borrowed(_) => "Borrowed",
        Cow::Owned(_) => "Owned",
    }
}

fn main() {
    println!("§3  [T]::clone_into reuses the elements too, not just the Vec");
    let source = [String::from("ab"), String::from("cd")];
    let mut target = vec![String::with_capacity(64), String::with_capacity(64), String::with_capacity(64)];
    let first = target[0].as_ptr();
    source.as_slice().clone_into(&mut target);
    println!("    target {target:?}, first String kept its own buffer: {}", target[0].as_ptr() == first);

    println!();
    println!("§5  CStr::clone_into into a CString of the same length reuses its buffer");
    let mut name = CString::new("Ada").unwrap();
    let start = name.as_ptr();
    let other: &CStr = c"Bob";
    other.clone_into(&mut name);
    println!("    name {name:?}, same buffer: {}", name.as_ptr() == start);

    println!();
    println!("§7  += on an empty Cow<str> borrows the right-hand side instead of allocating");
    let mut empty_borrowed: Cow<'_, str> = Cow::Borrowed("");
    empty_borrowed += "x";
    let mut empty_owned: Cow<'_, str> = Cow::Owned(String::with_capacity(64));
    empty_owned += "x";
    let mut full: Cow<'_, str> = Cow::Borrowed("a");
    full += "x";
    println!("    Borrowed(\"\") += \"x\"             -> {}", variant(&empty_borrowed));
    println!("    Owned(capacity 64, empty) += \"x\" -> {}   (the buffer is dropped)", variant(&empty_owned));
    println!("    Borrowed(\"a\") += \"x\"            -> {} {full:?}", variant(&full));

    println!();
    println!("§8  from_utf8_lossy borrows unless a byte needs replacing");
    for bytes in [&b""[..], b"valid", b"caf\xe9"] {
        let text = String::from_utf8_lossy(bytes);
        let shown = format!("{bytes:?}");
        println!("    {shown:<24} -> {} {text:?}", variant(&text));
    }

    println!();
    println!("§9  a Box lends what it holds, which is what lets `type Owned = Box<RawValue>` work");
    let boxed: Box<str> = Box::from("{\"raw\": true}");
    let lent: &str = boxed.borrow();
    println!("    Box<str>: Borrow<str> -> {lent:?}");

    println!();
    println!("§14 std's Cow<str>, measured in words");
    println!("    size_of::<Cow<str>>() = {} words", size_of::<Cow<'_, str>>() / size_of::<usize>());
}
