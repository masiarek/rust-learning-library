use std::mem::MaybeUninit;

fn main() {
    // The gap between len and capacity, typed as uninitialised memory.
    let mut v: Vec<u32> = Vec::with_capacity(10);
    v.push(1);
    println!("len {} cap {} spare {}", v.len(), v.capacity(), v.spare_capacity_mut().len());

    // Writing into it does NOT change the length. That is the whole contract:
    // you fill the memory, then you tell the Vec how much you filled.
    let mut v: Vec<u32> = Vec::with_capacity(4);
    let spare = v.spare_capacity_mut();
    spare[0].write(10);
    spare[1].write(20);
    println!("written but len is still {}", v.len());
    unsafe { v.set_len(2) };
    println!("after set_len(2): {v:?}");

    // Get it wrong and it is undefined behaviour, not a panic — set_len past
    // what you actually initialised reads uninitialised memory. This is the
    // one place a Vec asks you to be right without checking.
    let mut v: Vec<u8> = Vec::with_capacity(8);
    let spare = v.spare_capacity_mut();
    for (i, slot) in spare.iter_mut().enumerate().take(3) { slot.write(i as u8 * 5); }
    unsafe { v.set_len(3) };          // 3, because 3 is what was written
    println!("{v:?}");

    // The safe way to do the same thing, which is what you should reach for
    // unless you are wrapping a C API that writes into a buffer.
    let mut v: Vec<u8> = Vec::with_capacity(8);
    v.extend((0..3).map(|i| i * 5));
    println!("the safe spelling: {v:?}");

    // An empty spare region when the vector is full.
    let mut v = vec![1u8, 2, 3];
    v.shrink_to_fit();
    println!("full vector has {} spare slots", v.spare_capacity_mut().len());

    // MaybeUninit<T> is the same size as T — the region really is the buffer.
    println!("size_of MaybeUninit<u32> == size_of u32: {}",
             size_of::<MaybeUninit<u32>>() == size_of::<u32>());
}
