//! Step 2 of the `ToOwned` path: `str`, `[T]` and `Path` have no size known
//! at compile time, so you only ever hold a pointer to one — and that pointer
//! carries the length as a second word.
//!
//!   rustc --edition 2024 types_with_no_size.rs -o /tmp/twns && /tmp/twns

use std::path::Path;

/// A size in machine words, so the answer is the same on 32- and 64-bit targets.
fn words<T>() -> usize {
    size_of::<T>() / size_of::<usize>()
}

fn main() {
    println!("Checkpoint. Is a &str the same size as a &String?");
    println!("   &String : {} word    an address", words::<&String>());
    println!("   &str    : {} words   an address and a length", words::<&str>());

    println!();
    println!("Every pointer to an unsized type carries the length beside the address");
    println!("   &[i32]   : {} words", words::<&[i32]>());
    println!("   &Path    : {} words", words::<&Path>());
    println!("   Box<str> : {} words", words::<Box<str>>());
    println!("   &i32     : {} word    (i32 is Sized: nothing to carry)", words::<&i32>());

    println!();
    println!("The owned twin is Sized: a fixed-size handle to text on the heap");
    println!("   String   : {} words   address, capacity, length", words::<String>());
    println!("   Vec<i32> : {} words", words::<Vec<i32>>());

    println!();
    println!("The length lives in the pointer, so two &str can share one start");
    let hello: &str = "hello";
    let he: &str = &hello[..2];
    println!(
        "   {hello:?}.len() = {}, {he:?}.len() = {}, same first byte: {}",
        hello.len(),
        he.len(),
        hello.as_ptr() == he.as_ptr()
    );
}
