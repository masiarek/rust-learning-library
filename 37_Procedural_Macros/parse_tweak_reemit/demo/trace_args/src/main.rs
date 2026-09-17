use trace::trace;

#[trace(verbose)] // #[trace] takes no arguments
fn greet(name: &str) -> String {
    format!("hello, {name}")
}

fn main() {
    println!("{}", greet("Ada"));
    println!("{}", greet("Grace"));
}
