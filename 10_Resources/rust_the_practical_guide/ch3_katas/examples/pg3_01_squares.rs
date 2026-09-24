// Exercise 3.1: (1 + 2 + ... + n)^2 - (1^2 + 2^2 + ... + n^2). The book reads n
// from stdin; here a fixed table stands in for the keyboard so the output is an
// answer key. The second half measures where an i32 stops being enough.
fn difference(n: i32) -> i32 {
    let mut sum = 0;
    let mut sum_of_squares = 0;
    for i in 1..=n {
        sum += i;
        sum_of_squares += i.pow(2);
    }
    sum.pow(2) - sum_of_squares
}

/// The smallest n whose square of the sum is larger than `max`.
fn first_n_whose_square_exceeds(max: u128) -> u128 {
    let mut sum: u128 = 0;
    for n in 1.. {
        sum += n;
        if sum * sum > max {
            return n;
        }
    }
    unreachable!()
}

fn main() {
    for n in [5, 10, 100] {
        println!("The difference of the square of the sum and the sum of squares for N = {n} is {}", difference(n));
    }
    // The difference ... for N = 5 is 170
    // The difference ... for N = 10 is 2640
    // The difference ... for N = 100 is 25164150

    let n32 = first_n_whose_square_exceeds(i32::MAX as u128);
    let n64 = first_n_whose_square_exceeds(i64::MAX as u128);
    println!("the square of the sum outgrows an i32 at N = {n32}, an i64 at N = {n64}"); // ... an i32 at N = 304, an i64 at N = 77936

    let sum: u128 = (1..=n32).sum();
    let sum_of_squares: u128 = (1..=n32).map(|i| i * i).sum();
    println!("at N = {n32}: the square {} > i32::MAX {}", sum * sum, i32::MAX); // at N = 304: the square 2149249600 > i32::MAX 2147483647
    println!("but the difference {} would still fit", sum * sum - sum_of_squares); // but the difference 2139838520 would still fit
    println!("N = {}: an i32 keeps the answer {}", n32 - 1, difference(n32 as i32 - 1)); // N = 303: an i32 keeps the answer 2111836472
}
