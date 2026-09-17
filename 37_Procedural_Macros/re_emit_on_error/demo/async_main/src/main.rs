// What a macro like `#[tokio::main]` would return if it put back the function
// it was given, unchanged, after an error about its arguments.
compile_error!("unknown argument");
async fn main() {
    println!("hello");
}
