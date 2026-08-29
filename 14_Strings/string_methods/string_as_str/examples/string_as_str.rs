fn shout(s: &str) -> String { s.to_uppercase() }

fn main() {
    let s = String::from("hello");

    // All the same borrow.
    println!("{:?} {:?} {:?}", s.as_str(), &s[..], &*s);

    // Deref coercion means the call needs none of them.
    println!("{:?}", shout(&s));

    // Where it earns its place: as a function reference.
    let maybe = Some(String::from("x"));
    println!("{:?}", maybe.as_deref());
    let names = vec![String::from("a"), String::from("b")];
    let views: Vec<&str> = names.iter().map(String::as_str).collect();
    println!("{views:?}");

    // Matching against string literals.
    let cmd = String::from("stop");
    match cmd.as_str() {
        "go" => println!("going"),
        "stop" => println!("stopping"),
        other => println!("unknown {other:?}"),
    }
}
