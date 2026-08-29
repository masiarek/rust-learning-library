//! Sum an array of ten numbers, three ways, and check they agree.
//!
//! `calc_accum` is the function whose assembly the lesson quotes at two
//! optimization levels. `black_box` hides its argument from the optimizer, so
//! the second sum is really computed at run time — the answer is identical
//! either way, which is the whole point.

/// The function the lesson disassembles. `no_mangle` keeps the symbol name
/// readable in the assembly output.
#[unsafe(no_mangle)]
pub extern "C" fn calc_accum() -> i32 {
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    numbers.iter().sum()
}

fn main() {
    // Optimized, this whole function is one instruction. Its answer is the
    // same as if every one of the ten additions had been performed.
    println!("calc_accum()      = {}", calc_accum());

    // Same arithmetic, hidden from the optimizer: the loop survives into the
    // binary and the additions really happen.
    let numbers = std::hint::black_box([1, 2, 3, 4, 5, 6, 7, 8, 9, 10i32]);
    let summed: i32 = numbers.iter().sum();
    println!("black_box version = {summed}");

    // An optimization may change how long a program takes, how big it is, and
    // what its assembly looks like. It may not change this line.
    println!("same answer       = {}", calc_accum() == summed);
}
