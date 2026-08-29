fn main() {
    let mut s = String::with_capacity(64);
    s.push_str("small");
    println!("before len {} capacity {}", s.len(), s.capacity());

    s.shrink_to_fit();
    println!("after  len {} capacity {}", s.len(), s.capacity());

    // Contents are untouched; only the bookkeeping changed.
    println!("{s:?}");

    // The stronger move for a finished string.
    let mut done = String::with_capacity(64);
    done.push_str("final");
    let boxed = done.into_boxed_str();
    println!("Box<str> is {} bytes vs String's {}",
             std::mem::size_of_val(&boxed), std::mem::size_of::<String>());

    // Shrinking then growing again undoes the saving and costs two copies.
    let mut churn = String::with_capacity(32);
    churn.push_str("ab");
    churn.shrink_to_fit();
    let low = churn.capacity();
    churn.push_str("cdefgh");
    println!("{low} -> {}", churn.capacity());
}
