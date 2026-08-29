fn main() {
    println!("{}", String::new().is_empty());
    println!("{}", String::from("x").is_empty());

    // Capacity does not make it non-empty.
    let roomy = String::with_capacity(1024);
    println!("capacity {} but empty: {}", roomy.capacity(), roomy.is_empty());

    // clear() empties it while keeping the buffer.
    let mut used = String::from("hello");
    used.clear();
    println!("empty {} capacity {}", used.is_empty(), used.capacity());

    // Blank is not empty.
    let blank = String::from("   ");
    println!("{} {}", blank.is_empty(), blank.trim().is_empty());

    // Empty is not missing.
    let values: [Option<String>; 3] = [Some("v".into()), Some("".into()), None];
    for v in values {
        let verdict = match &v {
            None => "absent",
            Some(s) if s.is_empty() => "present but empty",
            Some(_) => "present",
        };
        println!("{v:?} -> {verdict}");
    }
}
