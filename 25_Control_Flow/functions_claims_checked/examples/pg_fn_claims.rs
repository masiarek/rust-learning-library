//! *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) introduces
//! functions in §2.3 and code blocks in §2.4. Each numbered block below runs
//! one sentence of those sections on rustc 1.98.0. The page is
//! 25_Control_Flow/functions_claims_checked/README.md.
//!
//!   rustc --edition 2024 pg_fn_claims.rs -o /tmp/pgf && /tmp/pgf

use std::any::type_name_of_val;

/// The book's first function, printed with an indent so it lines up below.
fn my_fn(s: &str) {
    println!("   {s}");
}

/// Listing 2.5, as printed (plus the indent).
fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("   Computing multiplication");
    num1 * num2
}

/// Listing 2.8, as printed: one value, which is a tuple.
fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}

/// `-> ()` written out, and a `println!` call as the tail expression.
fn shout(s: &str) -> () {
    println!("   {s}!")
}

/// Two arms, and either one is the function's value.
fn sign(n: i32) -> &'static str {
    if n < 0 { "negative" } else { "not negative" }
}

fn takes_i64(n: i64) -> i64 {
    n
}

/// A `return` inside a block leaves the function, not the block.
fn early(flag: bool) -> i32 {
    let v = {
        if flag {
            return 5;
        }
        1
    };
    v + 100
}

fn main() {
    println!("1. Listing 2.5 prints its message, then the product");
    let answer = multiplication(10, 15);
    println!("   answer = {answer}");

    println!();
    println!("2. A println! call is an expression of type (), so it can be a tail");
    let unit: () = println!("   this line is the value of a println! call");
    println!("   the call evaluated to {unit:?}");
    let back = shout("hello");
    println!("   shout(\"hello\") is declared -> () and returned {back:?}");

    println!();
    println!("3. An expression is not a line: if, match and a block have values");
    let n: i32 = -7;
    let label = if n < 0 { "negative" } else { "not negative" };
    let digits = match n.abs() {
        0..=9 => "one digit",
        _ => "more",
    };
    let doubled = {
        let twice = n * 2;
        twice
    };
    println!("   if -> {label}, match -> {digits}, block -> {doubled}");
    let total = n
        + 1
        + 2;
    println!("   and one let statement spread over three lines: total = {total}");

    println!();
    println!("4. A tail can be an if or a match: either arm is the value");
    println!("   sign(-3) = {}, sign(3) = {}", sign(-3), sign(3));

    println!();
    println!("5. return leaves the function, even from inside a block");
    println!("   early(true) = {}, early(false) = {}", early(true), early(false));

    println!();
    println!("6. Listing 2.8 returns one value, a tuple, and a let pattern takes it apart");
    let result = basic_math(10, 15);
    println!("   basic_math(10, 15) = {result:?}, type {}", type_name_of_val(&result));
    let (product, sum, difference) = basic_math(10, 15);
    println!("   destructured: {product} {sum} {difference}");

    println!();
    println!("7. The argument's type may be decided by the parameter, or coerced to it");
    let ten = 10;
    takes_i64(ten);
    println!("   let ten = 10; takes_i64(ten): ten is {}", type_name_of_val(&ten));
    let owned = String::from("Function call with a variable");
    my_fn(&owned);
    println!("   my_fn(&owned) accepted a &String for a &str parameter");

    println!();
    println!("8. Listing 2.9: the block's value is its tail; the names inside are gone after it");
    let full_name = {
        let first_name = "John";
        let last_name = "Archer";
        format!("{first_name} {last_name}")
    };
    println!("   full_name = {full_name}");

    println!();
    println!("9. A block inside a loop runs on every pass, and sees the loop variable");
    let mut squares = Vec::new();
    for i in 1..=3 {
        let square = {
            let s = i * i;
            squares.push(s);
            s
        };
        println!("   pass {i}: the block produced {square}");
    }
    println!("   the block ran {} times: {squares:?}", squares.len());

    println!();
    println!("10. A labelled block can leave early with a value");
    let verdict = 'check: {
        if full_name.is_empty() {
            break 'check "empty";
        }
        if full_name.len() > 8 {
            break 'check "long";
        }
        "short"
    };
    println!("   'check on {full_name:?} -> {verdict}");
}
