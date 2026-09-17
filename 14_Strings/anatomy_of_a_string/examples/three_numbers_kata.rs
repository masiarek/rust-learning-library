//! Kata solution: size_of, len and capacity for five Strings, then the one
//! String that clear() and shrink_to_fit() change one number at a time.
//!
//!   rustc --edition 2024 three_numbers_kata.rs -o /tmp/tnk && /tmp/tnk

fn row(label: &str, s: &String, note: &str) {
    println!(
        "   {label:<24} {:>11} {:>5} {:>8}   {note}",
        size_of_val(s),
        s.len(),
        s.capacity()
    );
}

fn header() {
    println!("   {:<24} {:>11} {:>5} {:>8}", "", "size_of_val", "len", "capacity");
}

fn main() {
    let a = String::new();
    let b = String::from("Hello");
    let c = String::from("zażółć");
    let d = "ab".repeat(500);
    let mut e = String::with_capacity(64);
    e.push_str("Hi");

    println!("1. Five Strings, three numbers each (bytes, 64-bit target)");
    header();
    row("String::new()", &a, "no text, no buffer");
    row("String::from(\"Hello\")", &b, "a buffer the size of the text");
    row("String::from(\"zażółć\")", &c, "six letters, ten bytes");
    row("\"ab\".repeat(500)", &d, "a thousand bytes, same 24");
    row("with_capacity(64) + \"Hi\"", &e, "room bought ahead of the text");

    for s in [&a, &b, &c, &d, &e] {
        assert_eq!(size_of_val(s), size_of::<String>());
        assert!(s.capacity() >= s.len());
    }
    assert_eq!((c.len(), c.chars().count()), (10, 6));

    println!();
    println!("2. One String, two calls, one number each");
    header();
    row("before", &e, "two bytes of text in a 64-byte buffer");
    e.clear();
    row("after clear()", &e, "the text is gone, the buffer stays");
    e.shrink_to_fit();
    row("after shrink_to_fit()", &e, "now the buffer is gone too");

    println!();
    println!("3. What the columns say");
    println!("   size_of_val is 24 on every row: it measures the String, never the text.");
    println!("   len counts the text in bytes, so \"zażółć\" is 10, not 6.");
    println!("   capacity is the buffer the String asked for: never below len, and");
    println!("   the only one of the three that sees room the text is not using.");
}
