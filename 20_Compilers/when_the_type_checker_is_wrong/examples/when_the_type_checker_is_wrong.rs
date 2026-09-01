// Issue 136508, rust-lang/rust: an enum constructor can be cast to an integer.
//
// This program compiles WITH A WARNING on purpose. The warning is the lesson:
// rustc 1.98.0 accepts the cast below and reports it under the lint
// `function_casts_as_integer`, which is warn-by-default.

enum Tag {
    Zero,
    One,
}

enum Wrapped {
    Value(i32),
}

fn main() {
    // A variant carrying no data casts to its discriminant: a compile-time
    // constant, the same number on every machine.
    println!("Tag::Zero as i32               = {}", Tag::Zero as i32);
    println!("Tag::One  as i32               = {}", Tag::One as i32);

    // A variant carrying data is a constructor *function*. Call it for a value.
    let Wrapped::Value(n) = Wrapped::Value(7);
    println!("Wrapped::Value(7) holds        = {n}");

    // `Wrapped::Value(0) as i32` is rejected -- a Wrapped is not an integer.
    // `Wrapped::Value as i32` is accepted, and is not a discriminant.
    let cast = Wrapped::Value as i32;
    let addr = Wrapped::Value as fn(i32) -> Wrapped as usize as i32;

    // The number is a truncated function address, so it differs from run to run
    // and can never be an answer key. What is fixed is what it equals.
    println!("cast is the function's address = {}", cast == addr);
}
