// Exercise 2.3: it compiles. A binding without `mut` may be assigned once, and
// the `let` line does not have to be where that happens.
fn main() {
    let mut x1 = 40;
    let x2;
    x1 = x1 * 3;
    x2 = x1 - 2;
    println!("x1 is: {}, x2 is: {}", x1, x2); // x1 is: 120, x2 is: 118
}
