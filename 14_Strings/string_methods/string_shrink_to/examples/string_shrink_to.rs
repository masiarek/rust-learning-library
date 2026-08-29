fn main() {
    let mut s = String::with_capacity(100);
    s.push_str("hello");
    println!("start        len {} capacity {}", s.len(), s.capacity());

    s.shrink_to(32);
    println!("shrink_to 32 len {} capacity {}", s.len(), s.capacity());

    // Never below len, whatever you ask for.
    s.shrink_to(0);
    println!("shrink_to 0  len {} capacity {}", s.len(), s.capacity());

    // Already small enough: nothing happens.
    let before = s.capacity();
    s.shrink_to(1000);
    println!("unchanged: {}", s.capacity() == before);

    // The reuse pattern: reclaim after a big item, keep a working floor.
    let mut buf = String::with_capacity(16);
    for item in ["short", &"x".repeat(200), "short"] {
        buf.clear();
        buf.push_str(item);
        buf.shrink_to(16);
        println!("len {:>3} capacity {:>3}", buf.len(), buf.capacity());
    }
}
