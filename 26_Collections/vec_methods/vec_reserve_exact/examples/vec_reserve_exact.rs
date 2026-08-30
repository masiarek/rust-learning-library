fn main() {
    // Asks for exactly len + additional, with no speculative slack.
    let mut v = vec![1, 2, 3];
    v.reserve_exact(7);
    println!("len {} capacity {}", v.len(), v.capacity());

    // reserve() on the same vector is allowed to round up.
    let mut a: Vec<u32> = Vec::new();
    let mut b: Vec<u32> = Vec::new();
    a.reserve(1);
    b.reserve_exact(1);
    println!("reserve(1) -> {}   reserve_exact(1) -> {}", a.capacity(), b.capacity());

    // "Exact" is a request, not a guarantee — the allocator may still hand
    // back more, so the contract is still `capacity() >= len + additional`.
    let mut v: Vec<u8> = Vec::new();
    v.reserve_exact(5);
    println!("asked exactly 5, got at least 5: {}", v.capacity() >= 5);

    // Where it belongs: a vector that is filled once and then kept. Slack
    // that will never be used is just wasted memory.
    let source = [1u8, 2, 3, 4, 5];
    let mut snapshot: Vec<u8> = Vec::new();
    snapshot.reserve_exact(source.len());
    snapshot.extend_from_slice(&source);
    println!("snapshot len {} cap {}", snapshot.len(), snapshot.capacity());

    // Where it does NOT belong: a vector still being pushed into. Exact
    // reservations before each push give back the growth-by-doubling that
    // makes pushing amortised O(1).
    let mut v: Vec<u32> = Vec::new();
    let mut reallocs = 0;
    for n in 0..8 {
        let before = v.capacity();
        v.reserve_exact(1);
        v.push(n);
        if v.capacity() != before { reallocs += 1; }
    }
    println!("reserve_exact before every push: {reallocs} reallocations for 8 pushes");

    let mut v: Vec<u32> = Vec::new();
    let mut reallocs = 0;
    for n in 0..8 {
        let before = v.capacity();
        v.push(n);
        if v.capacity() != before { reallocs += 1; }
    }
    println!("plain push:                      {reallocs} reallocations for 8 pushes");
}
