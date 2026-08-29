fn main() {
    let mut s = String::from("hello straße");
    s.make_ascii_uppercase();
    println!("{s:?}");                        // ß untouched

    // Compare with the allocating Unicode version.
    println!("{:?}", "hello straße".to_uppercase());

    // One field of a buffer, in place.
    let mut record = String::from("name:alice");
    if let Some(i) = record.find(':') {
        record.as_mut_str()[i + 1..].make_ascii_uppercase();
    }
    println!("{record:?}");

    // Title-casing the ASCII way: first byte only.
    let mut word = String::from("rust");
    word.as_mut_str()[..1].make_ascii_uppercase();
    println!("{word:?}");
}
