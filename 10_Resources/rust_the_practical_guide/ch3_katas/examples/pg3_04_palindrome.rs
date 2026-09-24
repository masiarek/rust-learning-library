// Exercise 3.4: a palindrome reads the same in both directions. Three readings
// of "the same": byte by byte (the book's), char by char, and letters only.
fn is_palindrome_bytes(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.iter().eq(bytes.iter().rev())
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn is_palindrome_sentence(s: &str) -> bool {
    let letters: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect();
    letters.iter().eq(letters.iter().rev())
}

fn main() {
    println!("{:<28} {:>5} {:>5} {:>8}", "input", "bytes", "chars", "letters");
    for input in ["1211", "1881", "12321", "Able was I ere I saw Elba", "été", "éé", "", "a"] {
        println!(
            "{:<28} {:>5} {:>5} {:>8}",
            format!("{input:?}"),
            is_palindrome_bytes(input),
            is_palindrome(input),
            is_palindrome_sentence(input)
        );
    }
    // "1211"                    false false    false
    // "1881"                     true  true     true
    // "12321"                    true  true     true
    // "Able was I ere I saw Elba" false false    true
    // "été"                     false  true     true
    // "éé"                      false  true     true
    // ""                         true  true     true
    // "a"                        true  true     true

    let bytes = "été".as_bytes();
    let reversed: Vec<u8> = bytes.iter().rev().copied().collect();
    println!("\"été\" is the bytes {bytes:02x?}"); // "été" is the bytes [c3, a9, 74, c3, a9]
    println!("read backwards      {reversed:02x?}"); // read backwards      [a9, c3, 74, a9, c3]
    println!("chars: {:?} reversed {:?}", "été".chars().collect::<String>(), "été".chars().rev().collect::<String>()); // chars: "été" reversed "été"
}
