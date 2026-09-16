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
    println!("Length zero is not the test; capacity is");
    let mut small = String::with_capacity(4);
    let small_capacity = small.capacity();
    "reuse me".clone_into(&mut small);
    println!("   with_capacity(4), 8 bytes in: capacity had to grow: {}", small.capacity() > small_capacity);

    println!();
    println!("Every Clone type gets the reuse too: the blanket impl forwards to clone_from");
    let source = String::from("reuse me");
    let mut owned_buf = String::with_capacity(64);
    let owned_start = owned_buf.as_ptr();
    source.clone_into(&mut owned_buf);
    println!("   String::clone_into  -> same buffer: {}", owned_buf.as_ptr() == owned_start);
    let votes = vec![3, 1, 2];
    let mut vote_buf: Vec<i32> = Vec::with_capacity(16);
    let vote_start = vote_buf.as_ptr();
    votes.clone_into(&mut vote_buf);
    println!("   Vec<i32>::clone_into -> same buffer: {}, {vote_buf:?}", vote_buf.as_ptr() == vote_start);

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
