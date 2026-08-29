fn main() {
    let s = "tab\there é 👋";

    println!("{}", s.escape_default());
    println!("{}", s.escape_debug());

    // The output is always pure ASCII.
    let escaped = s.escape_default().to_string();
    println!("ascii: {}", escaped.is_ascii());

    // Valid Rust literal syntax, which is what makes it usable for codegen.
    println!("let s = \"{}\";", "a\"b\\c\né".escape_default());

    // Every non-ASCII character becomes \u{...}.
    println!("{}", "αβγ".escape_default());
}
