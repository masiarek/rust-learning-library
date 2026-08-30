fn main() {
    let mut v: Vec<u32> = Vec::with_capacity(10);
    v.push(1);
    println!("len {} capacity {}", v.len(), v.capacity());

    // The first non-zero capacity depends on the element SIZE:
    // 8 for one-byte elements, 4 up to 1 KiB, 1 above that.
    let mut a: Vec<u8> = Vec::new();          a.push(0);
    let mut b: Vec<u32> = Vec::new();         b.push(0);
    let mut c: Vec<[u8; 2048]> = Vec::new();  c.push([0; 2048]);
    println!("first capacity — u8 {} u32 {} [u8; 2048] {}",
             a.capacity(), b.capacity(), c.capacity());

    // After that it doubles.
    let mut v: Vec<u32> = Vec::new();
    let mut seq = vec![];
    for n in 0..20u32 {
        v.push(n);
        if seq.last() != Some(&v.capacity()) { seq.push(v.capacity()); }
    }
    println!("capacity sequence: {seq:?}");

    // ...but the vec![] macro does not go through that path at all. It builds
    // a boxed array and converts it, so the capacity is EXACT.
    let n = 3;
    println!("vec![1,2,3] {}   new()+3 pushes {}   vec![0u8; n] {}",
             vec![1, 2, 3].capacity(),
             { let mut v = Vec::new(); v.push(1); v.push(2); v.push(3); v.capacity() },
             vec![0u8; n].capacity());

    // A zero-sized type never allocates, so its capacity is the maximum.
    let z: Vec<()> = Vec::new();
    println!("Vec<()> capacity is usize::MAX: {}", z.capacity() == usize::MAX);

    // Capacity never shrinks on its own. Removing everything keeps the buffer.
    let mut v = vec![0u8; 100];
    v.clear();
    println!("cleared: len {} cap {}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("shrunk:  len {} cap {}", v.len(), v.capacity());

    // The exact growth sequence is this std's choice, not a language promise.
    // What IS promised: capacity >= len, always, and reserve gives at least
    // what you ask for.
    let mut v: Vec<u16> = Vec::new();
    v.reserve(100);
    println!("capacity >= len: {}  >= reserved: {}",
             v.capacity() >= v.len(), v.capacity() >= 100);
}
