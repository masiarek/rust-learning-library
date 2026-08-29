fn main() {
    let s = "  hello \t\n";
    println!("{:?}", s.trim_ascii());
    println!("{}", s.trim_ascii() == s.trim());

    // const: a literal trimmed at compile time.
    const RAW: &str = "  compiled  ";
    const TRIMMED: &str = RAW.trim_ascii();
    println!("{TRIMMED:?}");

    // Where they differ: a non-breaking space is not ASCII whitespace.
    let nbsp = "\u{00A0}hi\u{00A0}";
    println!("{:?} vs {:?}", nbsp.trim_ascii(), nbsp.trim());

    // The five bytes it does remove.
    println!("{:?}", " \t\n\r\u{000C}x \t\n\r\u{000C}".trim_ascii());
}
