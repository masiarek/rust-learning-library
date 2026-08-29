fn main() {
    let mut s = String::from("hello world");
    s.truncate(5);
    println!("{s:?}");

    // Beyond the length is a no-op, not a panic.
    s.truncate(100);
    println!("{s:?}");

    // The one panic: an offset inside a character.
    let wide = String::from("héllo wörld");
    println!("boundary at 8? {}", wide.is_char_boundary(8));

    // Truncating to a byte budget safely.
    for budget in [2, 8] {
        let mut copy = wide.clone();
        copy.truncate(copy.floor_char_boundary(budget));
        println!("budget {budget:>2} -> {copy:?} ({} bytes)", copy.len());
    }

    // Capacity is not released.
    let mut roomy = String::with_capacity(32);
    roomy.push_str("hello world");
    roomy.truncate(5);
    println!("len {} capacity {}", roomy.len(), roomy.capacity());
}
