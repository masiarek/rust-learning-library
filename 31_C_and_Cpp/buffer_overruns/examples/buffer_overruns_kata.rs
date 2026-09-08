//! Kata solution: the off-by-one, three ways.
//!
//!   rustc --edition 2024 buffer_overruns_kata.rs -o /tmp/bok && /tmp/bok

fn main() {
    let readings = [12u32, 7, 30];

    println!("THE C SHAPE");
    println!("  for (int i = 0; i <= 3; i++) sum += readings[i];");
    println!("  The fourth read is whatever sits after the array -- a saved");
    println!("  register, a frame pointer, part of another variable -- so the");
    println!("  sum is a different large number on every run, and at -O2 it may");
    println!("  be a different one again.");
    println!();

    println!("1. THE INDEX THAT PANICS");
    println!("  readings[3] -> index out of bounds: the len is 3 but the index is 3");
    println!("  A bounds check, at runtime, on a slice whose length is not known");
    println!("  at compile time. It costs a compare-and-branch, and the branch");
    println!("  predictor makes it very nearly free -- and it turns a silent");
    println!("  wrong answer into a stopped program with a line number.");
    println!();

    println!("2. THE ONE THE COMPILER CATCHES OUTRIGHT");
    println!("  On a fixed-size array with a constant index, rustc rejects it at");
    println!("  compile time: \"this operation will panic at runtime\". No");
    println!("  execution needed -- the length is part of the type [u32; 3].");
    println!();

    println!("3. THE FORM THAT CANNOT BE WRONG");
    let sum: u32 = readings.iter().sum();
    println!("  readings.iter().sum::<u32>() = {sum}");
    println!("  There is no index to get wrong. The iterator knows the length,");
    println!("  and the bounds check is usually optimised away entirely because");
    println!("  the compiler can prove the range -- so the safe form is also the");
    println!("  fast one.");
    println!();

    println!("4. AND THE ONE THAT ASKS INSTEAD OF ASSUMING");
    for i in [2usize, 3] {
        match readings.get(i) {
            Some(v) => println!("  get({i}) -> Some({v})"),
            None => println!("  get({i}) -> None    <- no panic, a value to handle"),
        }
    }
    println!("  .get() returns Option, so 'past the end' becomes a case in the");
    println!("  type rather than a decision about whether to trust the index.");
    println!();

    println!("WHAT C HAS INSTEAD");
    println!("  Nothing in the language. An array decays to a pointer at the");
    println!("  first opportunity and the length is gone -- which is why every");
    println!("  C API takes a separate count, and why every one of those counts");
    println!("  is a chance to disagree with the buffer it describes.");

    assert_eq!(sum, 49);
    assert_eq!(readings.get(3), None);
}
