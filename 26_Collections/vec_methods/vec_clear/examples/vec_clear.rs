fn main() {
    let mut v = vec![1, 2, 3];
    v.clear();
    println!("{v:?} len {} is_empty {}", v.len(), v.is_empty());

    // It keeps the buffer. That is the reason to prefer it over `v = Vec::new()`
    // when the vector is about to be refilled.
    let mut buf: Vec<u8> = Vec::with_capacity(1024);
    buf.extend_from_slice(&[7; 1024]);
    buf.clear();
    println!("after clear: len {} cap {}", buf.len(), buf.capacity());
    buf.push(1);
    println!("refilled with no allocation: cap still {}", buf.capacity());

    // Reassigning instead throws the buffer away.
    let mut other: Vec<u8> = Vec::with_capacity(1024);
    println!("before reassignment: cap {}", other.capacity());
    other = Vec::new();
    println!("after reassignment:  cap {}", other.capacity());

    // Every element is dropped, front to back.
    struct Noisy(char);
    impl Drop for Noisy {
        fn drop(&mut self) { println!("  dropping {}", self.0); }
    }
    let mut v = vec![Noisy('a'), Noisy('b')];
    println!("clear() on two elements:");
    v.clear();

    // The buffer-reuse loop this enables: one allocation for the whole run.
    let mut line: Vec<u8> = Vec::new();
    let mut caps = vec![];
    for word in ["alpha", "beta", "gamma"] {
        line.clear();
        line.extend_from_slice(word.as_bytes());
        caps.push(line.capacity());
    }
    println!("capacity across three refills: {caps:?}");
}
