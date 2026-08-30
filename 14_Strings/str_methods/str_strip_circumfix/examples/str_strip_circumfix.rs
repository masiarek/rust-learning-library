fn main() {
    println!("{:?}", "\"hello\"".strip_circumfix('"', '"'));
    println!("{:?}", "<tag>".strip_circumfix('<', '>'));
    println!("{:?}", "/* note */".strip_circumfix("/*", "*/"));

    // Both, or nothing.
    for q in ["\"hi\"", "\"hi", "hi\"", "hi"] {
        println!("{:<8} -> {:?}", format!("{q:?}"), q.strip_circumfix('"', '"'));
    }

    // No double-counting when the string is too short to hold both.
    println!("{:?}", "\"".strip_circumfix('"', '"'));
    println!("{:?}", "".strip_circumfix('"', '"'));

    // The portable two-step spelling.
    let q = "\"hi\"";
    println!("{:?}", q.strip_prefix('"').and_then(|s| s.strip_suffix('"')));
}
