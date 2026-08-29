fn main() {
    println!("{:?}", "hello".to_ascii_uppercase());

    // ß is left alone, so no expansion.
    println!("{:?} vs {:?}", "straße".to_ascii_uppercase(), "straße".to_uppercase());

    // Round-trips within ASCII.
    let s = "MiXeD case 123!";
    println!("{}", s.to_ascii_uppercase().to_ascii_lowercase() == s.to_ascii_lowercase());

    // Length is always preserved.
    for s in ["ß", "ﬁ", "abc"] {
        println!("{s:<4?} ascii {} bytes, unicode {} bytes",
                 s.to_ascii_uppercase().len(), s.to_uppercase().len());
    }

    // Hex digits: an ASCII-by-definition alphabet.
    println!("{:?}", "1f4a9".to_ascii_uppercase());
}
