//! Exercise 1 (§5.7): one `Vec` that holds two kinds of number.
//!
//! The enum is the answer. The `main` is the book's, with the trailing space
//! in "Integer: {} " dropped. The second half is why the payload is `f64`
//! and not the `f32` the book's solution writes.
//!
//!   rustc --edition 2024 pg5_01_value_enum.rs -o /tmp/pg501 && /tmp/pg501

#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f64),
}

fn main() {
    let some_val = vec![Value::Integer(12), Value::Float(15.5)];

    // The derive the exercise asks for, put to use: variant name plus payload.
    println!("{some_val:?}");

    for i in some_val {
        match i {
            Value::Integer(num) => println!("Integer: {num}"),
            Value::Float(num) => println!("Float: {num}"),
        }
    }

    // Why f64: an f32 keeps 24 significant bits, so 16 777 217 is the first
    // whole number it cannot hold, and 0.1 + 0.2 looks exact only because
    // seven digits are all an f32 can show.
    let big = Value::Float(16_777_217.0);
    let big_f32 = 16_777_217.0_f32;
    println!("{big:?} as f32 -> {big_f32}");
    let sum = Value::Float(0.1 + 0.2);
    let sum_f32 = 0.1_f32 + 0.2_f32;
    println!("{sum:?} as f32 -> {sum_f32}");
}
