fn main() {
    // "additional" is on top of the CURRENT LENGTH, not the current capacity.
    let mut v = vec![1, 2, 3];
    v.reserve(10);
    println!("len {} capacity at least 13: {}", v.len(), v.capacity() >= 13);

    // If there is already room, it does nothing.
    let mut v: Vec<u8> = Vec::with_capacity(100);
    let before = v.capacity();
    v.reserve(10);
    println!("already had room, unchanged: {}", v.capacity() == before);

    // It may over-allocate deliberately, so repeated small reserves stay
    // amortised rather than reallocating every time.
    let mut v: Vec<u32> = Vec::new();
    v.reserve(1);
    println!("reserve(1) on an empty Vec<u32> gave capacity {}", v.capacity());

    // The usual use: you know the size just before you fill it.
    let words = ["alpha", "beta", "gamma"];
    let count = |reserve: bool| {
        let mut out: Vec<char> = Vec::new();
        if reserve { out.reserve(words.iter().map(|w| w.len()).sum()); }
        let mut reallocs = 0;
        for w in words {
            for c in w.chars() {
                let before = out.capacity();
                out.push(c);
                if out.capacity() != before { reallocs += 1; }
            }
        }
        (out.len(), reallocs)
    };
    println!("with reserve:    {:?} (len, reallocations)", count(true));
    println!("without reserve: {:?}", count(false));

    // extend() from an iterator with a known length already reserves for you,
    // which is why this is rarely needed with collect() or extend_from_slice().
    let mut v: Vec<u8> = Vec::new();
    v.extend_from_slice(&[0; 50]);
    println!("extend_from_slice reserved once: len {} cap {}", v.len(), v.capacity());

    // Capacity is measured in ELEMENTS. Reserving 10 for a Vec<u64> asks the
    // allocator for 80 bytes.
    let mut v: Vec<u64> = Vec::new();
    v.reserve(10);
    println!("10 u64 slots = {} bytes of buffer", v.capacity() * size_of::<u64>());
}
