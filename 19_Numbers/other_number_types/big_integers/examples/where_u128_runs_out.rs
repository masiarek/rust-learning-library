// Where the widest built-in integers stop, and how they divide a negative.
fn main() {
    let (mut k, mut n) = (1_u64, 1_u64);
    while let Some(next) = n.checked_mul(k + 1) {
        (k, n) = (k + 1, next);
    }
    println!("u64:  {k}! = {n}"); // u64:  20! = 2432902008176640000

    let (mut k, mut n) = (1_u128, 1_u128);
    while let Some(next) = n.checked_mul(k + 1) {
        (k, n) = (k + 1, next);
    }
    println!("u128: {k}! = {n}"); // u128: 34! = 295232799039604140847618609643520000000
    println!("35!: {:?}", n.checked_mul(35)); // 35!: None
    println!("u128::MAX has {} digits", u128::MAX.to_string().len()); // u128::MAX has 39 digits

    // `/` truncates toward zero, and `%` takes the sign of the left operand.
    println!("{} {}", -7 / 2, -7 % 2); // -3 -1
    println!("{} {}", 7 / -2, 7 % -2); // -3 1

    // Euclidean: the remainder is never negative.
    println!("{} {}", (-7_i32).div_euclid(2), (-7_i32).rem_euclid(2)); // -4 1
    println!("{} {}", 7_i32.div_euclid(-2), 7_i32.rem_euclid(-2)); // -3 1
}
