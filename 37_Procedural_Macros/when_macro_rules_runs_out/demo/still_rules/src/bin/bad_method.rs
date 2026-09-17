use still_rules::method;

fn main() {
    println!("{}", method!(PATCH));
    println!("{}", method!("GET"));
}
