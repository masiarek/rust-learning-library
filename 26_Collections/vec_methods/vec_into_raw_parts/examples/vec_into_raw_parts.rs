fn main() {
    let v = vec![10u32, 20, 30];
    println!("before: {v:?} len {} cap {}", v.len(), v.capacity());

    // Hand over the three numbers. The Vec is gone and nothing was freed:
    // from here on the allocation is yours to account for.
    let (ptr, len, cap) = v.into_raw_parts();
    println!("into_raw_parts: len {len} cap {cap} ptr non-null {}", !ptr.is_null());

    // Read through the pointer without going back through a Vec.
    let first = unsafe { *ptr };
    println!("first element read through the raw pointer: {first}");

    // The only safe end for it is handing the same three numbers back.
    let restored = unsafe { Vec::from_raw_parts(ptr, len, cap) };
    println!("restored: {restored:?} len {} cap {}", restored.len(), restored.capacity());

    // Capacity, not length, is what the allocator needs back — so it has to
    // survive the round trip too.
    let mut v: Vec<u8> = Vec::with_capacity(16);
    v.push(1);
    let (p, l, c) = v.into_raw_parts();
    println!("len {l} but cap {c} — the deallocation needs the second number");
    drop(unsafe { Vec::from_raw_parts(p, l, c) });
}
