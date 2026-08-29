fn main() {
    let mut s = String::new();
    println!("new         len {:>2} capacity {:>2}", s.len(), s.capacity());

    for word in ["aa", "bbbbbbb", "cccccccccc"] {
        s.push_str(word);
        println!("+{word:<11} len {:>2} capacity {:>2}", s.len(), s.capacity());
    }

    // At least what you asked for.
    let planned = String::with_capacity(10);
    println!("asked 10, got {}", planned.capacity());

    // Not part of the value: same text, different capacity, still equal.
    let tight = String::from("abc");
    let mut roomy = String::with_capacity(64);
    roomy.push_str("abc");
    println!("equal {} / capacities {} and {}",
             tight == roomy, tight.capacity(), roomy.capacity());

    // Bytes, not characters.
    let wide = "é".repeat(3);
    println!("{} chars, len {}", wide.chars().count(), wide.len());
}
