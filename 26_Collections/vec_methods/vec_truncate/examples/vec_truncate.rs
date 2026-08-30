fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.truncate(2);
    println!("{v:?}");

    // A len that is >= the current one does nothing. It never grows.
    let mut v = vec![1, 2];
    v.truncate(10);
    println!("truncate(10) on len 2: {v:?}");

    // truncate(0) is clear().
    let mut v = vec![1, 2, 3];
    v.truncate(0);
    println!("truncate(0): {v:?} is_empty {}", v.is_empty());

    // The dropped elements really are dropped, in order from the back.
    struct Noisy(u8);
    impl Drop for Noisy {
        fn drop(&mut self) { println!("  dropping {}", self.0); }
    }
    {
        let mut v = vec![Noisy(1), Noisy(2), Noisy(3), Noisy(4)];
        println!("truncate(1) on four elements:");
        v.truncate(1);
        println!("  survivor: {}", v[0].0);
    }   // the survivor drops here, at the end of the block

    // Capacity is untouched — this frees no memory.
    let mut v: Vec<u8> = Vec::with_capacity(64);
    v.extend_from_slice(&[0; 64]);
    v.truncate(1);
    println!("len {} cap {}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("after shrink_to_fit: len {} cap {}", v.len(), v.capacity());
}
