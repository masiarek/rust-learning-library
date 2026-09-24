// Exercise 3.2: the multiples of 3 or 5 below N, each counted once. A fixed
// table stands in for the book's stdin read.
fn sum_of_multiples(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

fn sum_of_multiples_chain(n: u32) -> u32 {
    (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

fn main() {
    for n in [10, 20, 1000] {
        println!("The sum of the multiples of 3 or 5 below {n} is {}", sum_of_multiples(n)); // 23, 78, 233168
        assert_eq!(sum_of_multiples(n), sum_of_multiples_chain(n));
    }

    // "Counted once": 15 is a multiple of both. Two separate sums count it twice.
    let threes: u32 = (1..20).filter(|i| i % 3 == 0).sum();
    let fives: u32 = (1..20).filter(|i| i % 5 == 0).sum();
    println!("below 20: multiples of 3 sum to {threes}, of 5 to {fives}, together {}", threes + fives); // ... 63, of 5 to 30, together 93
    println!("the || version says {}: 15 was counted once", sum_of_multiples(20)); // the || version says 78: 15 was counted once

    // The template types n as i32, the book's solution as u32. The type decides
    // where a negative input is caught.
    let as_i32: Result<i32, _> = "-5".trim().parse();
    let as_u32: Result<u32, _> = "-5".trim().parse();
    println!("\"-5\" as i32: {as_i32:?}; the loop 1..-5 then runs {} times", (1..-5).count()); // "-5" as i32: Ok(-5); the loop 1..-5 then runs 0 times
    println!("\"-5\" as u32: {as_u32:?}"); // "-5" as u32: Err(ParseIntError { kind: InvalidDigit })
}
