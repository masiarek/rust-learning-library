fn main() {
    // Grow: the new slots are clones of the value you pass.
    let mut v = vec![1, 2];
    v.resize(5, 0);
    println!("{v:?}");

    // Shrink: the value is ignored and the tail is dropped.
    let mut v = vec![1, 2, 3, 4, 5];
    v.resize(2, 99);
    println!("{v:?}");

    // Same length: nothing happens at all.
    let mut v = vec![1, 2, 3];
    v.resize(3, 0);
    println!("{v:?}");

    // T: Clone, and it really clones — one value in, n copies out.
    let mut v: Vec<String> = Vec::new();
    v.resize(3, String::from("x"));
    v[0].push('!');
    println!("{v:?}");

    // Which is why a non-Clone default needs resize_with instead.
    let mut v: Vec<Vec<u8>> = vec![vec![1]];
    v.resize_with(3, Vec::new);
    println!("resize_with for a fresh value each time: {v:?}");

    // Padding a buffer to a fixed width.
    let mut record = b"abc".to_vec();
    record.resize(8, b'.');
    println!("{}", String::from_utf8(record).unwrap());

    // The tail really is dropped when shrinking.
    struct Noisy(u8);
    impl Drop for Noisy {
        fn drop(&mut self) { println!("  dropping {}", self.0); }
    }
    {
        let mut v = vec![Noisy(1), Noisy(2), Noisy(3)];
        println!("resize(1, ..):");
        v.resize_with(1, || Noisy(0));
        println!("  survivor {}", v[0].0);
    }
}
