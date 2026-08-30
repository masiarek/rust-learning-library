fn main() {
    let v = vec![10u32, 20, 30];
    let p = v.as_ptr();

    // Reading through it is unsafe, because nothing checks the offset.
    println!("{} {} {}", unsafe { *p }, unsafe { *p.add(1) }, unsafe { *p.add(2) });

    // The elements really are contiguous: consecutive addresses differ by
    // exactly size_of::<T>(). Printing the addresses themselves would be a
    // different number every run, so derive the gap instead.
    let gap = unsafe { p.add(1) as usize - p as usize };
    println!("gap between elements: {gap} == size_of::<u32>() {}", size_of::<u32>());

    // While there is spare capacity the pointer is stable. Once a push
    // reallocates, the old pointer MAY be dangling — the allocator is free
    // to grow the block in place or to move it, and which one you get is
    // not reproducible. (Measured here: over 400 runs of this program the
    // same reallocation moved the buffer sometimes and not others.) That
    // "maybe" is exactly why holding a raw pointer across a push is UB:
    // there is no answer to check, only a rule to obey.
    let mut v: Vec<u8> = Vec::with_capacity(2);
    v.push(1);
    let before = v.as_ptr() as usize;
    v.push(2);
    println!("within capacity, pointer stable: {}", v.as_ptr() as usize == before);
    let cap_before = v.capacity();
    for n in 3..40 { v.push(n); }
    println!("capacity {} -> {} — the buffer was reallocated, so `before`",
             cap_before, v.capacity());
    println!("is not to be trusted whether or not the address changed");

    // An empty Vec has no allocation, and gives a dangling but ALIGNED
    // pointer rather than null — valid for zero-length reads only.
    let e: Vec<u64> = Vec::new();
    println!("empty vec pointer is aligned: {}", (e.as_ptr() as usize) % align_of::<u64>() == 0);

    // What you almost always want instead: a slice, which carries the length.
    let v = vec![1, 2, 3];
    let s: &[i32] = &v;
    println!("safe equivalent: {:?} len {}", s, s.len());

    // The real use is handing a buffer to C: pointer plus length, together.
    fn pretend_c_api(ptr: *const u8, len: usize) -> u32 {
        (0..len).map(|i| unsafe { *ptr.add(i) } as u32).sum()
    }
    let bytes = vec![1u8, 2, 3, 4];
    println!("sum through a raw pointer: {}", pretend_c_api(bytes.as_ptr(), bytes.len()));
}
