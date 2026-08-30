fn main() {
    // Copies a range of this vector onto its own end. No second vector,
    // no temporary, no borrow conflict.
    let mut v = vec![0, 1, 2, 3, 4];
    v.extend_from_within(1..3);
    println!("{v:?}");

    // The obvious hand-written version does not compile: extend_from_slice
    // would need &self and &mut self at the same time.
    // v.extend_from_slice(&v[1..3]);   // error[E0502]

    // Full range doubles the vector.
    let mut v = vec!["a", "b", "c"];
    v.extend_from_within(..);
    println!("{v:?}");

    // Any RangeBounds shape works.
    let mut v = vec![1, 2, 3, 4];
    v.extend_from_within(2..);
    println!("2.. {v:?}");
    let mut v = vec![1, 2, 3, 4];
    v.extend_from_within(..=1);
    println!("..=1 {v:?}");

    // T: Clone, and the clone really happens — this is a copy, not an alias.
    let mut v = vec![String::from("x")];
    v.extend_from_within(..);
    v[0].push('!');
    println!("{v:?}");

    // Repeated doubling is how you build a long repeating pattern cheaply.
    let mut pattern = vec![1u8, 2, 3];
    while pattern.len() < 12 { pattern.extend_from_within(..); }
    println!("{pattern:?} len {}", pattern.len());

    // An out-of-range end panics, like any slice index.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| {
        let mut v = vec![1, 2];
        v.extend_from_within(0..9);
    });
    std::panic::set_hook(hook);
    println!("out-of-range range panicked: {}", caught.is_err());
}
