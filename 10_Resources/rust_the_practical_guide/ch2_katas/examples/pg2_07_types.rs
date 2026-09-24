// Exercise 2.7: -15 needs a signed type, 170 does not fit in 7 bits, and the
// product -2550 does not fit in 8. i16 is the smallest type that holds all three.
fn main() {
    let a: i16 = -15;
    let b: i16 = 170;
    let name: &str = "Michael";
    println!("name is: {}, and the multiplication result is {}", name, a * b); // name is: Michael, and the multiplication result is -2550
    println!("i16 holds {}..={}", i16::MIN, i16::MAX); // i16 holds -32768..=32767
    let (a, b): (i32, i32) = (-15, 170);
    println!("the same with i32: {}", a * b); // the same with i32: -2550
    let (a, b): (i8, i8) = (-15, 100);
    println!("i8 fits -15 but not 170; even -15 * 100 overflows it: {:?}", a.checked_mul(b)); // None
}
