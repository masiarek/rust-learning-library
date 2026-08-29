fn main() {
    let s = "héllo";

    println!("{:?}", s.chars().collect::<Vec<char>>());
    println!("{} chars from {} bytes", s.chars().count(), s.len());

    // The whole iterator toolkit applies.
    println!("{}", s.chars().filter(|c| c.is_alphabetic()).count());
    println!("{}", s.chars().rev().collect::<String>());
    println!("{}", s.chars().map(|c| c.to_ascii_uppercase()).collect::<String>());

    // There is no s[i]; nth() decodes its way there, and says so by being O(n).
    println!("{:?}", s.chars().nth(1));

    // A scalar is not a grapheme: same visible text, two different counts.
    let precomposed = "é";           // U+00E9
    let combining = "e\u{301}";      // 'e' + COMBINING ACUTE ACCENT
    println!("{} vs {} chars, printed {} and {}",
             precomposed.chars().count(), combining.chars().count(),
             precomposed, combining);
}
