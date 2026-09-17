use describe_facade_derive::Describe; // the derive crate, without the facade

#[derive(Describe)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {}
