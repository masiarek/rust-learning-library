fn main() {
    // The only reliably correct source of the three numbers is a Vec that
    // produced them, which is what into_raw_parts is for.
    let original = vec![1u16, 2, 3];
    let (ptr, len, cap) = original.into_raw_parts();

    // Same pointer, same length, same capacity, same T. All four must match.
    let v = unsafe { Vec::from_raw_parts(ptr, len, cap) };
    println!("round trip: {v:?}");

    // Length may be lowered on the way back in — the elements above it are
    // then leaked rather than dropped, but the buffer is still freed correctly
    // because the capacity is right.
    let (ptr, _len, cap) = v.into_raw_parts();
    let shorter = unsafe { Vec::from_raw_parts(ptr, 2, cap) };
    println!("rebuilt with len 2 of 3: {shorter:?} cap {}", shorter.capacity());
    drop(shorter);

    // Building one from a Box<[T]> is the safe way to do the same thing, and
    // is what you almost always actually want.
    let boxed: Box<[u16]> = Box::new([7, 8, 9]);
    let from_box: Vec<u16> = boxed.into();
    println!("Vec::from(Box<[T]>): {from_box:?} cap {}", from_box.capacity());

    // A Vec's own buffer, borrowed rather than taken — no unsafe needed.
    let borrow = vec![4u16, 5];
    println!("as_slice instead: {:?}", borrow.as_slice());
}
