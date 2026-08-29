fn main() {
    println!("{}", "HELLO".eq_ignore_ascii_case("hello"));
    println!("{}", "Content-Type".eq_ignore_ascii_case("content-type"));
    println!("{}", "abc".eq_ignore_ascii_case("abd"));

    // ASCII only: accented pairs are not equal.
    println!("{}", "É".eq_ignore_ascii_case("é"));
    println!("{}", "É".to_lowercase() == "é".to_lowercase());

    // No allocation, unlike the lowercase-both-sides idiom.
    let header = "ACCEPT-ENCODING";
    println!("{}", header.eq_ignore_ascii_case("accept-encoding"));

    // Matching a token against a table.
    let methods = ["GET", "POST", "PUT"];
    for input in ["get", "Post", "patch"] {
        let found = methods.iter().find(|m| m.eq_ignore_ascii_case(input));
        println!("{input:<6} -> {found:?}");
    }
}
