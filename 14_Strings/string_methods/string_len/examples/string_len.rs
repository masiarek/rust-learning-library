fn main() {
    let s = String::from("héllo");
    println!("len {} chars {}", s.len(), s.chars().count());

    // len is what is written; capacity is what is available.
    let mut roomy = String::with_capacity(64);
    roomy.push_str("héllo");
    println!("len {} capacity {}", roomy.len(), roomy.capacity());

    // The same count as the str method.
    println!("{}", s.len() == s.as_str().len());

    // Bytes grow by more than one per character.
    let mut built = String::new();
    for c in ['a', 'é', '👋'] {
        built.push(c);
        println!("after {c:?}: len {}", built.len());
    }
}
