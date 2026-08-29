fn main() {
    let s = "tab\there\n\"quoted\" \\ é 👋";

    println!("{}", s.escape_debug());
    println!("{s:?}");
    println!("same: {}", format!("\"{}\"", s.escape_debug()) == format!("{s:?}"));

    // Printable non-ASCII survives; escape_default would not leave it.
    println!("debug   {}", "café".escape_debug());
    println!("default {}", "café".escape_default());

    // It is an iterator of char.
    println!("{:?}", "a\nb".escape_debug().collect::<Vec<char>>());

    // A leading combining mark is escaped so it cannot merge with the output.
    println!("{}", "\u{301}x".escape_debug());
}
