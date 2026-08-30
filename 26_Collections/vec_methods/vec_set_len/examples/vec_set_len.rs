fn main() {
    // set_len writes the length field. It initialises nothing, drops nothing,
    // and checks nothing — every guarantee is yours to keep.
    let mut v: Vec<u8> = Vec::with_capacity(4);
    let spare = v.spare_capacity_mut();
    spare[0].write(7);
    spare[1].write(8);
    unsafe { v.set_len(2) };            // 2 because 2 slots were written
    println!("{v:?}");

    // Shrinking with it LEAKS the elements above the new length: they are
    // never dropped. That is the difference from truncate().
    struct Noisy(u8);
    impl Drop for Noisy {
        fn drop(&mut self) { println!("  dropping {}", self.0); }
    }
    {
        let mut v = vec![Noisy(1), Noisy(2), Noisy(3)];
        println!("truncate(1):");
        v.truncate(1);
        println!("  and now the survivor:");
    }
    {
        let mut v = vec![Noisy(4), Noisy(5), Noisy(6)];
        println!("set_len(1):");
        unsafe { v.set_len(1) };
        println!("  5 and 6 were never dropped — that is a leak");
        println!("  and now the survivor:");
    }

    // The two conditions, both unchecked:
    //   new_len <= capacity()
    //   every element below new_len is initialised
    let mut v: Vec<u32> = Vec::with_capacity(10);
    println!("capacity {} — set_len(11) here would be UB", v.capacity());
    unsafe { v.set_len(0) };            // always sound: 0 <= capacity
    println!("set_len(0) is the one that is always safe: {v:?}");

    // Growing back into a region you HAVE initialised is the legitimate use.
    let mut v: Vec<u8> = Vec::with_capacity(4);
    v.extend_from_slice(&[1, 2, 3, 4]);
    unsafe { v.set_len(2) };            // hide the tail
    println!("hidden: {v:?}");
    unsafe { v.set_len(4) };            // reveal it again — still initialised
    println!("revealed: {v:?}");

    // The safe alternatives, in order of preference:
    //   truncate()      shrink and drop
    //   clear()         truncate(0)
    //   resize()        grow with a value
    //   extend()        grow from an iterator
    let mut v = vec![1, 2, 3, 4];
    v.truncate(2);
    v.resize(4, 0);
    println!("no unsafe needed: {v:?}");
}
