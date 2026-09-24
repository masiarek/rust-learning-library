// Exercise 2.9: three functions with the signatures the call site asks for.
fn add_3(x: i32) -> i32 {
    x + 3
}

fn add_5(x: i32) -> i32 {
    x + 5
}

fn times(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let x = 3;
    let y = 4;
    println!("The result of x+3 times y+5 is {}", times(add_3(x), add_5(y))); // The result of x+3 times y+5 is 54
}
