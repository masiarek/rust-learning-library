fn main() {
    let s = "key=value=more";

    println!("{:?}", s.find('='));
    println!("{:?}", s.find("value"));
    println!("{:?}", s.find(char::is_numeric));   // None, not -1

    // Splitting by hand — correct, but the +1 is the pattern's byte length.
    if let Some(i) = s.find('=') {
        println!("{:?} / {:?}", &s[..i], &s[i + 1..]);
    }
    // The same thing, with no arithmetic to get wrong.
    println!("{:?}", s.split_once('='));

    // Byte offsets, so a wide character shifts everything after it.
    let wide = "héllo=x";
    println!("{:?} at byte {:?}", '=', wide.find('='));

    // First character matching a predicate.
    println!("{:?}", "abc123".find(char::is_numeric));
}
