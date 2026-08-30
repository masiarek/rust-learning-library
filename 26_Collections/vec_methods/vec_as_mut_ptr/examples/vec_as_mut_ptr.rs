fn main() {
    let mut v = vec![1u32, 2, 3];
    let p = v.as_mut_ptr();
    unsafe { *p = 99; *p.add(2) = 77; }
    println!("{v:?}");

    // Same pointer as as_ptr, with write permission — and the same rule:
    // it is invalidated by anything that can reallocate.
    let mut v: Vec<u8> = Vec::with_capacity(4);
    v.push(1);
    let before = v.as_mut_ptr() as usize;
    v.push(2);
    println!("still within capacity: {}", v.as_mut_ptr() as usize == before);

    // The pattern it exists for: hand a buffer to something that fills it,
    // then set the length to what was actually written.
    fn fill(ptr: *mut u8, cap: usize) -> usize {
        let n = cap.min(4);
        for i in 0..n { unsafe { *ptr.add(i) = (i as u8 + 1) * 11 } }
        n                                  // how many it wrote
    }
    let mut buf: Vec<u8> = Vec::with_capacity(8);
    let written = fill(buf.as_mut_ptr(), buf.capacity());
    unsafe { buf.set_len(written) };
    println!("filled through a raw pointer: {buf:?}");

    // The safe spelling of the same thing, when you control the writer.
    let mut buf: Vec<u8> = Vec::with_capacity(8);
    buf.extend((1..=4).map(|i| i * 11));
    println!("the safe version: {buf:?}");

    // Taking it does not borrow the Vec for any length of time, which is the
    // trap: the compiler will NOT stop you using a stale pointer.
    let mut v = vec![1u8];
    let p = v.as_mut_ptr();
    unsafe { *p = 2 };                 // fine: nothing has reallocated yet
    v.reserve(1000);                   // p may now be dangling
    println!("use `p` after this and it is UB, not a compile error: {v:?}");

    // Which is why the borrow-checked route is the default answer.
    let mut v = vec![1, 2, 3];
    if let Some(first) = v.first_mut() { *first = 9; }
    println!("{v:?}");
}
