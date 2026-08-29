fn main() {
    let mut s = String::from("programming");
    s.retain(|c| !"aeiou".contains(c));
    println!("{s:?}");

    // Works on non-ASCII, because it visits chars rather than bytes.
    let mut mixed = String::from("héllo wörld");
    mixed.retain(|c| c.is_alphabetic());
    println!("{mixed:?}");

    // FnMut: the predicate can carry state.
    let mut seen = String::new();
    let mut dedup = String::from("aabbccdd");
    dedup.retain(|c| if seen.contains(c) { false } else { seen.push(c); true });
    println!("{dedup:?}");

    // In place: the capacity survives.
    let mut roomy = String::with_capacity(32);
    roomy.push_str("a1b2c3");
    roomy.retain(char::is_alphabetic);
    println!("{roomy:?} capacity {}", roomy.capacity());

    // The allocating alternative.
    println!("{:?}", "a1b2c3".chars().filter(|c| c.is_numeric()).collect::<String>());
}
