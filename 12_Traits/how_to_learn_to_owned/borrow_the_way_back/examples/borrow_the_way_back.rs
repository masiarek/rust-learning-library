//! Step 8 of the `ToOwned` path: `type Owned: Borrow<Self>` promises the owned
//! value can lend out its borrowed form — which is what lets a
//! `HashMap<String, _>` be searched with a `&str`.
//!
//!   rustc --edition 2024 borrow_the_way_back.rs -o /tmp/btwb && /tmp/btwb

use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::ffi::CStr;
use std::path::{Path, PathBuf};

fn main() {
    println!("Checkpoint. Does seats.get(\"Ada\") compile on a HashMap<String, u32>?");
    let mut seats: HashMap<String, u32> = HashMap::new();
    seats.insert(String::from("Ada"), 3);
    println!("   yes, because String: Borrow<str> -> {:?}", seats.get("Ada"));

    println!();
    println!("There and back: to_owned goes out, borrow comes home");
    let name: String = "Ada".to_owned();
    let back: &str = name.borrow();
    let votes: Vec<i32> = [3, 1][..].to_owned();
    let back_votes: &[i32] = votes.borrow();
    let file: PathBuf = Path::new("notes.txt").to_owned();
    let back_file: &Path = file.borrow();
    println!("   \"Ada\"     -> String  -> &str   {back:?}");
    println!("   [3, 1]    -> Vec     -> &[i32] {back_votes:?}");
    println!("   notes.txt -> PathBuf -> &Path  {back_file:?}");

    println!();
    println!("A Cow<str> lends a &str from either arm");
    let borrowed: Cow<'_, str> = Cow::Borrowed("ballot");
    let owned: Cow<'_, str> = Cow::Owned(String::from("ballot"));
    let (x, y): (&str, &str) = (&borrowed, &owned);
    println!("   Cow::Borrowed -> {x:?}, Cow::Owned -> {y:?}");

    println!();
    println!("std hands you a Cow when it may have to repair the text");
    let clean = c"thanksfish".to_string_lossy();
    let broken = CStr::from_bytes_with_nul(b"caf\xe9\0").unwrap().to_string_lossy();
    println!("   CStr::to_string_lossy(\"thanksfish\")  -> {}, {clean:?}", variant(&clean));
    println!("   CStr::to_string_lossy(b\"caf\\xe9\")    -> {}, {broken:?}", variant(&broken));
    let bytes = String::from_utf8_lossy(b"caf\xc3\xa9");
    println!("   String::from_utf8_lossy(valid UTF-8) -> {}, {bytes:?}", variant(&bytes));

    println!();
    println!("The write: to_mut calls to_owned only while the Cow is still Borrowed");
    let literal: &'static str = "Hello World";
    let mut cow: Cow<'_, str> = Cow::Borrowed(literal);
    println!("   before to_mut: {}, points at the literal: {}", variant(&cow), cow.as_ptr() == literal.as_ptr());
    let (start, capacity, len) = {
        let text: &mut String = cow.to_mut();
        (text.as_ptr(), text.capacity(), text.len())
    };
    println!(
        "   after to_mut:  {}, new buffer: {}, capacity == length: {}",
        variant(&cow),
        start != literal.as_ptr(),
        capacity == len
    );
    let same = cow.to_mut().as_ptr() == start;
    println!("   to_mut again:  {}, same buffer, nothing copied: {same}", variant(&cow));
    let grew = {
        let text = cow.to_mut();
        text.push('!');
        text.capacity() > capacity
    };
    println!("   push('!'): no spare capacity, so the String had to reallocate: {grew}");
    println!("   the literal is untouched: {literal:?}, the Cow reads {:?}", &*cow);
}

fn variant(cow: &Cow<'_, str>) -> &'static str {
    match cow {
        Cow::Borrowed(_) => "Cow::Borrowed",
        Cow::Owned(_) => "Cow::Owned",
    }
}
