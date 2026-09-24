// Exercise 2.2: the two fixed lines assign x2 and print it. Declaring x2 with no
// value makes that assignment the first one, so it needs neither `mut` nor a
// value that is thrown away.
fn main() {
    let x1 = 40;
    let x2;
    x2 = x1 - 2;
    println!("x1 is: {} and x2 is: {}", x1, x2); // x1 is: 40 and x2 is: 38
}
