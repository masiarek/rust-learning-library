fn main() {
    // Shrink, but not below the floor you name.
    let mut v: Vec<u32> = Vec::with_capacity(100);
    v.extend_from_slice(&[1, 2, 3]);
    v.shrink_to(10);
    println!("len {} cap {}", v.len(), v.capacity());

    // The floor never wins against the length: capacity stays >= len.
    let mut v = vec![1u32, 2, 3, 4, 5];
    v.shrink_to(2);
    println!("asked for 2 with len 5: cap {}", v.capacity());

    // A floor above the current capacity does nothing — it never grows.
    let mut v: Vec<u8> = Vec::with_capacity(4);
    v.shrink_to(100);
    println!("shrink_to(100) on cap 4: cap {}", v.capacity());

    // shrink_to(0) is shrink_to_fit().
    let mut a: Vec<u16> = Vec::with_capacity(50);
    a.push(1);
    let mut b = a.clone();
    a.shrink_to(0);
    b.shrink_to_fit();
    println!("shrink_to(0) {} == shrink_to_fit {}", a.capacity(), b.capacity());

    // Where the floor earns its keep: a reused buffer that should give memory
    // back after a spike, but keep enough for the ordinary case.
    let mut buf: Vec<u8> = Vec::new();
    let mut caps = vec![];
    for size in [4, 4096, 4, 4] {
        buf.clear();
        buf.resize(size, 0);
        buf.shrink_to(64);            // never below 64, never hoard 4 KiB
        caps.push(buf.capacity());
    }
    println!("capacity across a spike: {caps:?}");

    // Same thing with shrink_to_fit would reallocate back up every round.
    let mut buf: Vec<u8> = Vec::new();
    let mut caps = vec![];
    for size in [4, 4096, 4, 4] {
        buf.clear();
        buf.resize(size, 0);
        buf.shrink_to_fit();
        caps.push(buf.capacity());
    }
    println!("with shrink_to_fit:      {caps:?}");
}
