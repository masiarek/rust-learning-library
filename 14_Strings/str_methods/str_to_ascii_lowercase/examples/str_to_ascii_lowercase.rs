fn main() {
    println!("{:?}", "Hello World".to_ascii_lowercase());

    // Non-ASCII is left exactly as it is.
    let mixed = "CAFÉ Straße";
    println!("{:?}", mixed.to_ascii_lowercase());
    println!("{:?}", mixed.to_lowercase());

    // Length-preserving, which the Unicode version is not.
    for s in ["İ", "ß", "ABC"] {
        println!("{s:<4?} ascii {} bytes, unicode {} bytes",
                 s.to_ascii_lowercase().len(), s.to_lowercase().len());
    }

    // Where it is exactly right: an ASCII-by-specification alphabet.
    println!("{:?}", "Content-Type".to_ascii_lowercase());
    println!("{:?}", "IMAGE.PNG".to_ascii_lowercase());
}
