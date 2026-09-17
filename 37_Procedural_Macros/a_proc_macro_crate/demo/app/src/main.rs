use hello_macro::HelloMacro;

/// The trait lives here, not in `hello_macro`: a proc-macro crate cannot export
/// one. The generated `impl HelloMacro for …` finds it because it is in scope.
pub trait HelloMacro {
    fn hello_macro() -> &'static str;
}

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
enum Waffles {}

fn main() {
    println!("{}", Pancakes::hello_macro());
    println!("{}", Waffles::hello_macro());
}
