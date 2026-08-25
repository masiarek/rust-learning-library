//! Three words on the stack, bytes on the heap. len is what you have;
//! capacity is what you paid for.
//!
//!   rustc --edition 2024 anatomy_of_a_string.rs -o /tmp/anat && /tmp/anat

fn report(label: &str, s: &String) {
    println!("   {label:<30} len {:>2}   capacity {:>2}", s.len(), s.capacity());
}

fn main() {
    println!("1. String::new() allocates nothing");
    let mut s = String::new();
    report("String::new()", &s); // len 0, capacity 0 — no heap buffer yet

    println!("\n2. The first push buys a buffer; growth roughly doubles");
    s.push_str("aa");
    report("push_str(\"aa\")", &s);
    s.push_str("bbbbbbb"); // len 9 outgrows the buffer
    report("push_str(\"bbbbbbb\")", &s);
    s.push_str("cccccccccc"); // len 19 outgrows it again
    report("push_str(\"cccccccccc\")", &s);
    println!("   (the exact numbers are the allocator's choice — the SHAPE is the");
    println!("    lesson: capacity jumps ahead of len, so most pushes cost nothing)");

    println!("\n3. Paying up front: with_capacity");
    let mut sized = String::with_capacity(32);
    report("String::with_capacity(32)", &sized);
    sized.push_str("scores: 5, 4, 3");
    report("push_str(\"scores: 5, 4, 3\")", &sized);
    sized.push_str(", 2, 1, 0");
    report("push_str(\", 2, 1, 0\")", &sized);
    println!("   two pushes, zero reallocations — the buffer never moved");

    println!("\n4. Giving it back: shrink_to_fit");
    sized.shrink_to_fit();
    report("shrink_to_fit()", &sized); // capacity falls to len

    println!("\n5. Capacity is bookkeeping, not content");
    let plain = String::from("hi");
    let mut roomy = String::with_capacity(64);
    roomy.push_str("hi");
    println!("   \"hi\" with capacity {:>2} == \"hi\" with capacity {:>2}?  {}",
        plain.capacity(), roomy.capacity(), plain == roomy);

    println!("\n6. A String is a Vec<u8> that promises UTF-8");
    let word = String::from_utf8(vec![83, 84, 65, 82]);
    println!("   from_utf8([83, 84, 65, 82]) = {word:?}");
    let bad = String::from_utf8(vec![83, 84, 0xFF]);
    println!("   from_utf8([83, 84, 0xFF])   = Err: {}", bad.unwrap_err());
    println!("   the promise is checked at the door, so it never needs checking inside");
}
