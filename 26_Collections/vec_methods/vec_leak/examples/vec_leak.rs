fn main() {
    // Consumes the Vec and gives back a mutable slice that outlives it,
    // by never freeing the buffer.
    let leaked: &'static mut [i32] = vec![1, 2, 3].leak();
    leaked[0] = 99;
    println!("{leaked:?}");

    // It is a slice, so the length is fixed and every slice method works.
    leaked.sort();
    println!("sorted in place: {leaked:?} first {:?}", leaked.first());

    // Why it exists: a value the program builds once at startup and needs for
    // the whole run, without a static, a Box::leak dance, or an Rc.
    let table: &'static [u8] = vec![0u8, 1, 4, 9, 16].leak();
    fn lookup(t: &'static [u8], i: usize) -> u8 { t[i] }
    println!("lookup(3) = {}", lookup(table, 3));

    // The lifetime is inferred, and 'static is only the most common choice.
    // A shorter one is legal and just as leaky.
    let short: &mut [u8] = vec![7, 8].leak();
    println!("borrowed for less than 'static: {short:?}");

    // The memory is NOT freed. If you need it back, reconstruct and drop —
    // which is the only way, and it is unsafe.
    let v = vec![1u8, 2, 3];
    let (ptr, len, cap) = v.into_raw_parts();
    let reclaimed = unsafe { Vec::from_raw_parts(ptr, len, cap) };
    println!("reclaimed instead of leaked: {reclaimed:?}");

    // Box::leak is the same idea for a single value.
    let one: &'static mut u32 = Box::leak(Box::new(42));
    *one += 1;
    println!("Box::leak: {one}");

    // And the honest alternative most of the time: keep the Vec alive.
    let owned = vec![1, 2, 3];
    fn borrow(xs: &[i32]) -> usize { xs.len() }
    println!("no leak needed: {}", borrow(&owned));
}
