// Exercise 2.12: quadrupling is doubling twice.
fn double(x: i32) -> i32 {
    x * 2
}

fn quadruple(x: i32) -> i32 {
    double(double(x))
}

fn main() {
    println!("For 1: the expected value is 4 while the output is {}", quadruple(1)); // For 1: the expected value is 4 while the output is 4
    println!("For 2: the expected value is 8 while the output is {}", quadruple(2)); // For 2: the expected value is 8 while the output is 8
    println!("For 3: the expected value is 12 while the output is {}", quadruple(3)); // For 3: the expected value is 12 while the output is 12
    println!("For 4: the expected value is 16 while the output is {}", quadruple(4)); // For 4: the expected value is 16 while the output is 16
}
