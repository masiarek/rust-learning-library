fn main() {
    let mut s = String::from("café");
    println!("{:?} len {}", s, s.len());
    println!("popped {:?}", s.pop());
    println!("{:?} len {}", s, s.len());

    // Empty gives None, so this loop terminates on its own.
    let mut word = String::from("abc");
    while let Some(c) = word.pop() {
        println!("popped {c:?}, left {word:?}");
    }
    println!("{:?}", word.pop());

    // Capacity is not returned.
    let mut big = String::with_capacity(32);
    big.push_str("hello");
    big.pop();
    println!("len {} capacity {}", big.len(), big.capacity());

    // Reversing by popping.
    let mut src = String::from("héllo");
    let mut out = String::new();
    while let Some(c) = src.pop() { out.push(c); }
    println!("{out:?}");
}
