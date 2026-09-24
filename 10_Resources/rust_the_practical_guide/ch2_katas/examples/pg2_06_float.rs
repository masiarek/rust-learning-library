// Exercise 2.6: 3.14159 is a floating-point literal, so the binding needs a
// float type. Either width holds it; an integer type keeps only the 3.
use std::any::type_name_of_val;

fn main() {
    let pi: f32;
    pi = 3.14159; // This value represents pi
    println!("pi is: {}", pi); // pi is: 3.14159
    let pi64: f64 = 3.14159;
    println!("as f64: {}", pi64); // as f64: 3.14159
    println!("what an i32 can keep of it: {}", pi64 as i32); // what an i32 can keep of it: 3
    let unannotated = 3.14159;
    println!("with no annotation the literal is {}", type_name_of_val(&unannotated)); // with no annotation the literal is f64
}
