//! Step 9 of the `ToOwned` path: `clone_into` writes into an owned value you
//! already have, and saves an allocation only when that value has room.
//!
//!   rustc --edition 2024 clone_into_refills.rs -o /tmp/cir && /tmp/cir

fn main() {
    println!("Checkpoint. buf was made by String::with_capacity(64). Does clone_into change its capacity?");
    let mut buf = String::with_capacity(64);
    let (capacity, start) = (buf.capacity(), buf.as_ptr());
    "reuse me".clone_into(&mut buf);
    println!(
        "   capacity unchanged: {}, same buffer: {}, buf = {buf:?}",
        buf.capacity() == capacity,
        buf.as_ptr() == start
    );

    println!();
    println!("Into an empty String there is no room, so it allocates like to_owned would");
    let mut empty = String::new();
    println!("   capacity before: {}", empty.capacity());
    "reuse me".clone_into(&mut empty);
    println!("   capacity after holds the text: {}", empty.capacity() >= "reuse me".len());

    println!();
    println!("A loop that refills one buffer, instead of making a String per row");
    let rows = ["Ada", "Grace", "Barbara"];
    let mut line = String::with_capacity(16);
    let first = line.as_ptr();
    for row in rows {
        row.clone_into(&mut line);
        println!("   {line:<8} same buffer: {}", line.as_ptr() == first);
    }
}
