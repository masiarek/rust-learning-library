fn main() {
    let mut exact = String::from("ab");
    let mut rounded = String::from("ab");
    exact.reserve_exact(5);
    rounded.reserve(5);
    println!("reserve_exact {} / reserve {}", exact.capacity(), rounded.capacity());

    // Both guarantee at least len + additional.
    println!("{} {}", exact.capacity() >= 7, rounded.capacity() >= 7);

    // The right use: a size you know and will not exceed.
    let payload = "the quick brown fox";
    let mut buf = String::new();
    buf.reserve_exact(payload.len());
    buf.push_str(payload);
    println!("len {} capacity {}", buf.len(), buf.capacity());

    // The wrong use: reserving exactly, repeatedly, in a growth loop.
    let mut bad = String::new();
    let mut reallocs = 0;
    let mut last = bad.capacity();
    for _ in 0..6 {
        bad.reserve_exact(4);
        bad.push_str("abcd");
        if bad.capacity() != last { reallocs += 1; last = bad.capacity(); }
    }
    println!("{reallocs} capacity changes over 6 appends");
}
