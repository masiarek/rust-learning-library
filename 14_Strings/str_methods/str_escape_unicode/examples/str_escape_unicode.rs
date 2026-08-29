fn main() {
    println!("{}", "ab".escape_unicode());
    println!("{}", "é".escape_unicode());

    // The three escapes side by side.
    let s = "a\né";
    println!("debug   {}", s.escape_debug());
    println!("default {}", s.escape_default());
    println!("unicode {}", s.escape_unicode());

    // Where it earns its keep: two strings that look identical.
    let precomposed = "é";
    let combining = "e\u{301}";
    println!("{precomposed} vs {combining} -- equal? {}", precomposed == combining);
    println!("{}", precomposed.escape_unicode());
    println!("{}", combining.escape_unicode());
}
