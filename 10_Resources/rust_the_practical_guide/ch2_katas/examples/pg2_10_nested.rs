// Exercise 2.10: the two intermediate variables become nested calls.
fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

fn main() {
    println!("Answer: {}", triple(triple(double(5)))); // Answer: 90
}
