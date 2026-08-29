fn main() {
    let s = String::from("héllo");
    let (len, cap) = (s.len(), s.capacity());

    let (ptr, l, c) = s.into_raw_parts();
    println!("len {l} capacity {c} -- the same values the String held: {}", (l, c) == (len, cap));

    // Nothing is freed until it is given back.
    let back = unsafe { String::from_raw_parts(ptr, l, c) };
    println!("{back:?}");

    // Round trip preserves the capacity exactly.
    let mut roomy = String::with_capacity(32);
    roomy.push_str("x");
    let (p2, l2, c2) = roomy.into_raw_parts();
    let restored = unsafe { String::from_raw_parts(p2, l2, c2) };
    println!("len {} capacity {}", restored.len(), restored.capacity());

    // Borrowing, for comparison: as_ptr transfers nothing.
    let borrowed = String::from("kept");
    let _p = borrowed.as_ptr();
    println!("{borrowed:?} is still owned by its binding");
}
