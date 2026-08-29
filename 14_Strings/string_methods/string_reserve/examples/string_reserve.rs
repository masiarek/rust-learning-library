fn main() {
    let mut s = String::from("hello");
    println!("len {} capacity {}", s.len(), s.capacity());

    // Headroom, not a total: 10 MORE than the current length.
    s.reserve(10);
    println!("after reserve(10): len {} capacity {} (>= {})",
             s.len(), s.capacity(), s.len() + 10);

    // Already big enough: nothing happens.
    let before = s.capacity();
    s.reserve(1);
    println!("unchanged: {}", s.capacity() == before);

    // It may round up; reserve_exact does not.
    let mut a = String::from("ab");
    let mut b = String::from("ab");
    a.reserve(5);
    b.reserve_exact(5);
    println!("reserve {} / reserve_exact {}", a.capacity(), b.capacity());

    // The pattern it is for: size once, then push without reallocating.
    let mut out = String::new();
    out.reserve(32);
    let cap = out.capacity();
    for i in 0..8 { out.push_str(&format!("{i} ")); }
    println!("capacity unchanged through the loop: {}", out.capacity() == cap);
}
