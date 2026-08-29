fn main() {
    let ascii = "  the   quick \t brown \n fox  ";
    println!("{:?}", ascii.split_ascii_whitespace().collect::<Vec<&str>>());
    println!("same as unicode: {}",
             ascii.split_ascii_whitespace().eq(ascii.split_whitespace()));

    // Where they part company: a non-breaking space.
    let nbsp = "a\u{00A0}b";
    println!("{:?}", nbsp.split_whitespace().collect::<Vec<&str>>());
    println!("{:?}", nbsp.split_ascii_whitespace().collect::<Vec<&str>>());

    // The five ASCII separators.
    let all = "a b\tc\nd\re\u{000C}f";
    println!("{:?}", all.split_ascii_whitespace().collect::<Vec<&str>>());

    // A log line: machine-generated ASCII, which is what this is for.
    let line = "2026-08-29  INFO   started";
    println!("{:?}", line.split_ascii_whitespace().collect::<Vec<&str>>());
}
