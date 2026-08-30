use std::collections::TryReserveError;

fn main() {
    // reserve_exact's fallible twin: no speculative slack, and a Result.
    let mut v = vec![1u32, 2, 3];
    v.try_reserve_exact(7).unwrap();
    println!("len {} capacity {}", v.len(), v.capacity());

    // The two axes are independent: exact-or-not, fallible-or-not.
    //   reserve            slack, aborts on failure
    //   reserve_exact      no slack, aborts on failure
    //   try_reserve        slack, returns Err
    //   try_reserve_exact  no slack, returns Err
    let mut a: Vec<u32> = Vec::new();
    let mut b: Vec<u32> = Vec::new();
    a.try_reserve(1).unwrap();
    b.try_reserve_exact(1).unwrap();
    println!("try_reserve(1) -> {}   try_reserve_exact(1) -> {}", a.capacity(), b.capacity());

    // An impossible request is an Err, not an abort.
    let mut v: Vec<u64> = Vec::new();
    println!("huge request is Err: {}", v.try_reserve_exact(usize::MAX / 4).is_err());
    println!("and the vector is untouched: cap {}", v.capacity());

    // The shape it exists for: allocate exactly what a caller asked for,
    // and report failure instead of dying.
    fn exact_copy(src: &[u8]) -> Result<Vec<u8>, TryReserveError> {
        let mut out = Vec::new();
        out.try_reserve_exact(src.len())?;
        out.extend_from_slice(src);
        Ok(out)
    }
    let copy = exact_copy(&[7, 8, 9]).unwrap();
    println!("{copy:?} len {} cap {}", copy.len(), copy.capacity());

    // Already big enough: Ok, and nothing changes.
    let mut v: Vec<u8> = Vec::with_capacity(50);
    let before = v.capacity();
    v.try_reserve_exact(10).unwrap();
    println!("no-op when it already fits: {}", v.capacity() == before);
}
