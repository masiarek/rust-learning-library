use show_tokens::{ShowTokens, show_attr, show_tokens};

/// A point on a plane.
#[derive(Debug, ShowTokens)]
struct Point {
    x: i32,
    y: i32,
}

#[show_attr(times = 3)]
fn greet(name: &str) -> String {
    format!("hello, {name}")
}

/// Every line of `tokens`, indented under its label.
fn show(label: &str, tokens: &str) {
    println!("   {label}");
    for line in tokens.lines() {
        println!("      {line}");
    }
}

fn main() {
    println!("1. A derive receives the item, and adds to it");
    show("received", Point::RECEIVED);
    let p = Point { x: 1, y: 2 };
    println!("   the struct is still there: {p:?}, x + y = {}", p.x + p.y);
    println!();
    println!("2. A function-like macro receives what is between its delimiters");
    show("received", show_tokens!(GET /users/{id} => list_users));
    println!();
    println!("3. An attribute receives its arguments and the item, and replaces the item");
    show("arguments", GREET_ARGS);
    show("item", GREET_ITEM);
    println!("   the replacement put greet back: {}", greet("Ada"));
}
