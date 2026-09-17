use show_tokens::swallow;

#[swallow] // swallow returns an empty TokenStream
fn greet() -> &'static str {
    "hello"
}

fn main() {
    println!("{}", greet());
}
