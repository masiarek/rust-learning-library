fn main() {
    let mut s = String::with_capacity(64);
    s.push_str("finished");
    println!("String   len {} capacity {}", s.len(), s.capacity());

    let boxed: Box<str> = s.into_boxed_str();
    println!("Box<str> len {}", boxed.len());

    // Two words instead of three.
    println!("String {} bytes / Box<str> {} bytes",
             std::mem::size_of::<String>(), std::mem::size_of::<Box<str>>());

    // The spare capacity was returned on the way.
    let back = boxed.into_string();
    println!("back    len {} capacity {}", back.len(), back.capacity());

    // Everything a &str can do still works; growing does not.
    let names: Vec<Box<str>> = ["ada", "grace"].iter().map(|s| (*s).into()).collect();
    println!("{:?}", names.iter().map(|n| n.to_uppercase()).collect::<Vec<String>>());
}
