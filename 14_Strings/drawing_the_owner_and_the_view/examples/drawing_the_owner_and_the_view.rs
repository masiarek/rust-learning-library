//! The drawing on the page, checked against the machine. Sizes are printed in
//! machine words and addresses as offsets into the owner's buffer: words hold
//! on a 32-bit target too, and offsets hold on every run, where the raw
//! addresses change each time the program starts.
//!
//!   rustc --edition 2024 drawing_the_owner_and_the_view.rs -o /tmp/dov && /tmp/dov

use std::mem::size_of;

/// A type's size in machine words.
fn words<T>() -> usize {
    size_of::<T>() / size_of::<usize>()
}

/// Where `view` starts, in bytes from the start of `owner`'s buffer, or
/// `None` when it does not point into that buffer at all.
fn offset_in(owner: &str, view: &str) -> Option<usize> {
    let buffer = owner.as_bytes().as_ptr_range();
    let start = view.as_ptr();
    buffer.contains(&start).then(|| start.addr() - buffer.start.addr())
}

fn main() {
    let s = String::from("héllo world");
    let world: &str = &s[7..];

    println!("1. Two handles, measured in machine words");
    println!("   String  {} words   a pointer, a length and a capacity", words::<String>());
    println!("   &str    {} words   a pointer and a length", words::<&str>());

    println!();
    println!("2. The owner's buffer, one column per byte");
    println!("   s = {s:?}   len {}   capacity {}", s.len(), s.capacity());
    let mut offset_row = String::from("   offset");
    let mut byte_row = String::from("   byte  ");
    let mut char_row = String::from("   char  ");
    for (i, b) in s.bytes().enumerate() {
        offset_row += &format!("{i:>3}");
        byte_row += &format!(" {b:02x}");
        // A char starts only at a boundary; the other bytes continue one.
        let shown = match s.get(i..).and_then(|rest| rest.chars().next()) {
            Some(' ') => '␣',
            Some(c) => c,
            None => '·',
        };
        char_row += &format!("{shown:>3}");
    }
    println!("{offset_row}");
    println!("{byte_row}");
    println!("{char_row}");

    println!();
    println!("3. The view points into that buffer");
    println!("   world = {world:?}   len {}", world.len());
    println!("   starts at offset {:?} of s's buffer", offset_in(&s, world));
    println!("   7, not 6: 'é' is the two bytes c3 a9");

    println!();
    println!("4. Making the view owned makes a second buffer");
    let owned: String = world.to_owned();
    println!("   world.to_owned() == world          {}", owned == world);
    println!("   its pointer, as an offset into s   {:?}", offset_in(&s, &owned));
    println!("   len {}   capacity {}   a buffer of its own", owned.len(), owned.capacity());

    println!();
    println!("5. A move copies the handle and leaves the buffer where it is");
    let before = s.as_ptr();
    let moved = s;
    println!("   let moved = s;   buffer address changed: {}", moved.as_ptr() != before);

    println!();
    println!("6. Even two bytes of text live on the heap, not inside the handle");
    let hi = String::from("hi");
    let handle = (&raw const hi).cast::<u8>();
    let handle_bytes = handle.addr()..handle.addr() + size_of::<String>();
    println!("   String::from(\"hi\")   capacity {}   bytes inside the handle: {}",
        hi.capacity(), handle_bytes.contains(&hi.as_ptr().addr()));
}
