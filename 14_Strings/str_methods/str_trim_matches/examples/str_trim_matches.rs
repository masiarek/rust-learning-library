fn main() {
    println!("{:?}", "xxhixx".trim_matches('x'));
    println!("{:?}", "--a-b--".trim_matches('-'));
    println!("{:?}", "1a2".trim_matches(char::is_numeric));
    println!("{:?}", "xy hi yx".trim_matches(&['x', 'y', ' '][..]));

    // A &str pattern is not accepted; chain the one-sided methods instead.
    let s = "abcXabc";
    println!("{:?}", s.trim_start_matches("abc").trim_end_matches("abc"));

    // The quote trap: it repeats, and it does not require a pair.
    println!("{:?}", "\"\"\"".trim_matches('"'));
    println!("{:?}", "hi\"".trim_matches('"'));

    // Balanced unquoting, which is usually what was meant.
    let quoted = "\"hi\"";
    let unquoted = quoted.strip_prefix('"').and_then(|s| s.strip_suffix('"'));
    println!("{unquoted:?}");
    println!("{:?}", "hi\"".strip_prefix('"').and_then(|s| s.strip_suffix('"')));
}
