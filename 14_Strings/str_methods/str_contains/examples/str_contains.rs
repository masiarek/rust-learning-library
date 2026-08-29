fn main() {
    let s = "the quick brown fox";

    println!("{}", s.contains("quick"));          // &str
    println!("{}", s.contains('q'));              // char
    println!("{}", s.contains(&['x', 'z'][..]));  // any of these chars
    println!("{}", s.contains(char::is_numeric)); // a predicate

    // Byte-level, not linguistic.
    println!("{}", "Hello".contains("hello"));
    println!("{}", "Hello".to_lowercase().contains("hello"));

    // Two spellings of the same visible text do not match each other.
    println!("{}", "café".contains("é"));
    println!("{}", "cafe\u{301}".contains("é"));

    // The empty pattern is everywhere.
    println!("{} {}", s.contains(""), "".contains(""));
}
