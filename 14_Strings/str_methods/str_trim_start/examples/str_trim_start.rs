fn main() {
    let s = "   indented   ";
    println!("{:?}", s.trim_start());
    println!("{:?}", s.trim_end());
    println!("{:?}", s.trim());

    // Stripping indentation while keeping the line ending.
    let block = "    alpha\n    beta\n";
    let stripped: String = block.lines().map(|l| format!("{}\n", l.trim_start())).collect();
    print!("{stripped}");

    // "start" is memory order, not the left of the screen — which is why
    // trim_left was renamed. Here the leading char is Hebrew, not whitespace.
    let rtl = "  \u{05D0}\u{05D1}  ";
    println!("{:?}", rtl.trim_start());
}
