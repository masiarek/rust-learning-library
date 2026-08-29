fn main() {
    let path = "/usr/local/bin";

    println!("{}", path.starts_with('/'));
    println!("{}", path.starts_with("/usr"));
    println!("{}", path.starts_with(char::is_alphabetic));

    // A predicate tests the first character only.
    println!("{}", "7up".starts_with(char::is_numeric));

    // The pairing that avoids hand-written byte arithmetic.
    for flag in ["--verbose", "-v", "plain"] {
        match flag.strip_prefix("--") {
            Some(name) => println!("{flag:<10} long option {name:?}"),
            None => println!("{flag:<10} starts_with(\"--\") = {}", flag.starts_with("--")),
        }
    }

    // Everything starts with the empty pattern.
    println!("{}", path.starts_with(""));
}
