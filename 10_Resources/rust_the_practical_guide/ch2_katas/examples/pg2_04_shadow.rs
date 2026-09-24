// Exercise 2.4: `a` holds a &str, so `a = 10` is a type error. A second `let`
// makes a new variable with the same name and its own type.
use std::any::type_name_of_val;

fn main() {
    let a = "three"; // don't change this line
    println!("before the second let, a is {:?}, a {}", a, type_name_of_val(&a)); // before the second let, a is "three", a &str
    let a = 10; // don't change the name of this variable
    println!("a is: {}", a); // a is: 10
    println!("after it, a is an {}", type_name_of_val(&a)); // after it, a is an i32
}
