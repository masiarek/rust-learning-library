fn main() {
    let text = "let x = 42;";

    let token = &text[8..10];
    println!("{token:?} at {:?}", text.substr_range(token));

    // Identity, not content: an equal string from elsewhere is not part of it.
    let elsewhere = String::from("42");
    println!("{:?}", text.substr_range(elsewhere.as_str()));
    println!("but find() matches by content: {:?}", text.find("42"));

    // Recovering a token's position for an error message.
    for tok in text.split_whitespace() {
        let at = text.substr_range(tok).map(|r| r.start);
        println!("{tok:<4?} column {:?}", at);
    }

    // The whole string is a substring of itself.
    println!("{:?}", text.substr_range(text));
}
