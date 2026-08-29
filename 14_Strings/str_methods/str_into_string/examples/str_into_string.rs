fn main() {
    let s = String::from("hello");
    let boxed: Box<str> = s.into_boxed_str();
    println!("{boxed:?}");

    // And back, without copying.
    let back: String = boxed.into_string();
    println!("{back:?} capacity {}", back.capacity());

    // The size difference that motivates Box<str>.
    println!("String   {} bytes", std::mem::size_of::<String>());
    println!("Box<str> {} bytes", std::mem::size_of::<Box<str>>());

    // A String with slack loses it on the way out and comes back tight.
    let mut roomy = String::with_capacity(64);
    roomy.push_str("small");
    println!("before {} / {}", roomy.len(), roomy.capacity());
    let tight = roomy.into_boxed_str().into_string();
    println!("after  {} / {}", tight.len(), tight.capacity());
}
