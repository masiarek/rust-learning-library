//! Comparing two numbers of different types: `<` wants one type on both
//! sides, so the question is which type can hold both values.
//!
//!   rustc --edition 2024 comparing_two_number_types.rs -o /tmp/ctnt && /tmp/ctnt

use std::any::type_name_of_val;
use std::cmp::Ordering;

/// Compares an `i64` with an `f64` exactly, the way Python compares an int
/// with a float. Neither type holds every value of the other, so there is no
/// common type to convert into; split the float into whole part and fraction
/// instead.
fn cmp_i64_f64(i: i64, f: f64) -> Option<Ordering> {
    if f.is_nan() {
        return None;
    }
    // 2^63 is exactly representable; every finite f64 at or beyond it is out
    // of i64's range, and so is everything below -2^63.
    if f >= 9_223_372_036_854_775_808.0 {
        return Some(Ordering::Less);
    }
    if f < -9_223_372_036_854_775_808.0 {
        return Some(Ordering::Greater);
    }
    let whole = f.trunc() as i64; // exact: f.trunc() is a whole number in range
    match i.cmp(&whole) {
        Ordering::Equal => 0.0_f64.partial_cmp(&f.fract()),
        other => Some(other),
    }
}

fn main() {
    println!("1. THE WORKING ANSWER: CONVERT ONE SIDE, LOSSLESSLY");
    let a: i32 = 10;
    let b: u16 = 100;
    println!("   a: i32 = {a}, b: u16 = {b}");
    println!("   a < i32::from(b)        : {}", a < i32::from(b));
    println!("   a < b.into()            : {}", a < b.into());
    println!("   a.cmp(&i32::from(b))    : {:?}", a.cmp(&i32::from(b)));
    println!("   i32::from(u16) compiles only because every u16 fits in an i32.");

    println!();
    println!("2. PICK THE TYPE THAT HOLDS BOTH RANGES");
    let small: u16 = 65_535;
    let neg: i32 = -1;
    let big32: u32 = 3_000_000_000;
    let neg64: i64 = -1;
    let big64: u64 = u64::MAX;
    let row = |pair: &str, via: &str, claim: String, answer: bool| {
        println!("   {pair:<14}{via:<8}{claim:<28}: {answer}");
    };
    row("u16 and i32", "i32", format!("{neg} < {small}"), neg < i32::from(small));
    row("i32 and u32", "i64", format!("{neg} < {big32}"), i64::from(neg) < i64::from(big32));
    row("i64 and u64", "i128", format!("{neg64} < {big64}"), i128::from(neg64) < i128::from(big64));
    let len = vec![0u8; 3].len();
    let limit: u64 = 2;
    row("usize and u64", "u64", format!("{len} > {limit}"), u64::try_from(len).is_ok_and(|n| n > limit));
    println!("   {:<22}no From either way, so u64::try_from(len) = {:?}", "", u64::try_from(len));
    let i: i128 = -1;
    let u: u128 = 1;
    row("i128 and u128", "none", format!("{i} < {u}"), i < 0 || i.cast_unsigned() < u);
    println!("   {:<22}no wider type exists: test the sign, then compare as u128", "");

    println!();
    println!("3. THE SAME COMPARISONS WITH `as`, AND WHERE THEY LIE");
    println!("   -1i32 as u32 < 1u32          : {:<6} -1 became {}", (neg as u32) < 1u32, neg as u32);
    println!("   -1i32 as u64 < 1u64          : {:<6} a WIDER type, and -1 became {}", (neg as u64) < 1u64, neg as u64);
    println!("   -1i32 < 3_000_000_000 as i32 : {:<6} 3000000000 became {}", neg < big32 as i32, big32 as i32);
    println!("   widening is safe when the TARGET holds every value of the source:");
    println!("   width is not enough, the sign has to fit too. From only exists");
    println!("   where it does, so i32::from(b) cannot make this mistake.");

    println!();
    println!("4. INTEGER AND FLOAT: f64 HOLDS EVERY i32, NOT EVERY i64");
    println!("   f64::from(i32::MAX) = {}   exact", f64::from(i32::MAX));
    let n: i64 = 9_007_199_254_740_993; // 2^53 + 1
    let f: f64 = 9_007_199_254_740_992.0; // 2^53
    println!("   n = {n} (2^53 + 1), f = {f:.1} (2^53)");
    println!("   n as f64 == f              : {}   <- two different numbers, 'equal'", n as f64 == f);
    println!("   n as f64 > f               : {}", n as f64 > f);
    println!("   cmp_i64_f64(n, f)          : {:?}   <- exact", cmp_i64_f64(n, f));
    println!("   cmp_i64_f64(3, 3.5)        : {:?}", cmp_i64_f64(3, 3.5));
    println!("   cmp_i64_f64(-3, -3.5)      : {:?}", cmp_i64_f64(-3, -3.5));
    println!("   cmp_i64_f64(i64::MAX, 2^63): {:?}", cmp_i64_f64(i64::MAX, 9_223_372_036_854_775_808.0));
    println!("   cmp_i64_f64(0, NAN)        : {:?}", cmp_i64_f64(0, f64::NAN));

    println!();
    println!("5. try_into() FOR A CONVERSION THAT CANNOT FAIL");
    let r: Result<i32, _> = b.try_into();
    println!("   b.try_into() as i32 has type {}", type_name_of_val(&r));
    println!("   Infallible has no values, so that unwrap() can never panic.");
    println!("   the conversion that CAN fail is the other direction:");
    println!("   u16::try_from(70_000i32)   = {:?}", u16::try_from(70_000_i32));
    println!("   u16::try_from(-1i32)       = {:?}", u16::try_from(-1_i32));
}
