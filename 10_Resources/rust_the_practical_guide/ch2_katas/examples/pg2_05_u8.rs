// Exercise 2.5: a u8 holds 0..=255, so `-1` is refused before anything runs.
fn main() {
    let x: u8; // Don't change this line!
    x = 1;
    println!("x is: {}", x); // x is: 1
    println!("a u8 holds {}..={}", u8::MIN, u8::MAX); // a u8 holds 0..=255
    println!("the u8 that C stores for -1: {}", -1i32 as u8); // the u8 that C stores for -1: 255
}
