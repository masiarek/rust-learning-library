fn main() {
    // Hands the unused tail of the buffer back to the allocator.
    let mut v: Vec<u32> = Vec::with_capacity(100);
    v.extend_from_slice(&[1, 2, 3]);
    println!("before: len {} cap {}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("after:  len {} cap {}", v.len(), v.capacity());

    // Nothing else does this. clear(), truncate(), pop(), drain() and retain()
    // all leave the buffer at full size on purpose.
    let mut v = vec![0u8; 64];
    v.clear();
    println!("cleared but not shrunk: len {} cap {}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("now: cap {}", v.capacity());

    // It asks the allocator to resize the block. Whether the buffer MOVES is
    // the allocator's call — shrinking in place is allowed, and different
    // allocators answer differently, so the address is not something to
    // assert on. Either way it is not free: call it once, at the final size.
    let mut v: Vec<u16> = Vec::with_capacity(8);
    v.push(1);
    println!("cap 8 -> {} (the buffer may or may not have moved)", {
        v.shrink_to_fit();
        v.capacity()
    });

    // "As much as possible": std may keep more than len, so the promise is
    // capacity() >= len(), not capacity() == len().
    let mut v = vec![1, 2, 3];
    v.shrink_to_fit();
    println!("cap >= len after shrinking: {}", v.capacity() >= v.len());

    // Already exact: a no-op.
    let mut v = vec![1u8, 2, 3];
    let before = v.capacity();
    v.shrink_to_fit();
    println!("already tight, unchanged: {}", v.capacity() == before);

    // The build-then-freeze shape it exists for.
    let mut collected: Vec<u32> = Vec::with_capacity(1000);
    for n in 0..5 { collected.push(n); }
    println!("built: cap {}", collected.capacity());
    collected.shrink_to_fit();
    println!("frozen: cap {}", collected.capacity());

    // into_boxed_slice() does the same shrink and drops the capacity field.
    let boxed = vec![1u8, 2, 3].into_boxed_slice();
    println!("Box<[u8]> of len {} — no spare capacity to track", boxed.len());
}
