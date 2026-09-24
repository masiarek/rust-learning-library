//! Exercise 6 (§5.7): the square of a number, or an error.
//!
//! The function is the book's with the missing return type written in:
//! `Result<i32, String>`, because the body returns `Ok(result)` where
//! `result: i32` and `Err(...)` with a `String`. Then a version that also
//! refuses a square that does not fit.
//!
//!   rustc --edition 2024 pg5_06_calculate_square.rs -o /tmp/pg506 && /tmp/pg506

fn calculate_square(num: i32) -> Result<i32, String> {
    if num >= 0 {
        let result = num * num;
        println!("The square of {} is: {}", num, result);
        Ok(result)
    } else {
        Err("Negative number provided".to_string())
    }
}

/// `checked_mul` returns `None` instead of overflowing; `ok_or_else` turns
/// that into the `Err` this function promises.
fn checked_square(num: i32) -> Result<i32, String> {
    if num < 0 {
        return Err("Negative number provided".to_string());
    }
    num.checked_mul(num)
        .ok_or_else(|| format!("{num} squared does not fit in an i32"))
}

fn main() {
    let number = 7;
    if let Err(e) = calculate_square(number) {
        println!("Error: {e}");
    }

    // The `if let Err` only speaks up on the error path.
    if let Err(e) = calculate_square(-3) {
        println!("Error: {e}");
    }

    // A `match` sees both paths, and the checked version has one more way to fail.
    for n in [46_340, 46_341] {
        match checked_square(n) {
            Ok(square) => println!("{n} squared is {square}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}
