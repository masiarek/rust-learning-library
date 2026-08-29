fn main() {
    let line = "    value  \r\n";
    println!("{:?}", line.trim_end());
    println!("{:?}", line.trim());

    // Reading a line: drop the newline, keep the indentation.
    for raw in ["    nested\n", "top\r\n"] {
        println!("{:?} -> {:?}", raw, raw.trim_end());
    }

    // The invisible mismatch trailing space causes.
    let stored = "value ";
    println!("{} {}", stored == "value", stored.trim_end() == "value");

    // Trailing-whitespace cleanup across a block.
    let messy = "a   \nb\t\nc\n";
    let clean: Vec<&str> = messy.lines().map(str::trim_end).collect();
    println!("{clean:?}");
}
