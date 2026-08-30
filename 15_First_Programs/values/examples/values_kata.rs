//! Kata solution: Fibonacci, and the width that runs out.
//!
//!     rustc --edition 2024 values_kata.rs -o /tmp/vk && /tmp/vk

/// The classic recursive spelling, with an explicit `return`.
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

/// The same function as an expression — no `return`, no semicolon on the last line.
fn fib_expr(n: u32) -> u32 {
    if n < 2 { n } else { fib_expr(n - 1) + fib_expr(n - 2) }
}

/// Iterative, and it never recurses.
fn fib_iter(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let (mut prev, mut cur) = (0u32, 1u32);
    for _ in 2..=n {
        let next = prev + cur;
        prev = cur;
        cur = next;
    }
    cur
}

/// Counts its own calls, so "expensive" is a number rather than an adjective.
fn fib_counted(n: u32, calls: &mut u64) -> u64 {
    *calls += 1;
    if n < 2 {
        return n as u64;
    }
    fib_counted(n - 1, calls) + fib_counted(n - 2, calls)
}

/// The largest n whose fib(n) still fits in `max`, and that value.
fn last_fitting(max: u128) -> (u32, u128) {
    let (mut prev, mut cur) = (0u128, 1u128); // fib(0), fib(1)
    let mut n = 0u32;
    while cur <= max {
        let Some(next) = prev.checked_add(cur) else {
            return (n + 1, cur);
        };
        prev = cur;
        cur = next;
        n += 1;
    }
    (n, prev)
}

fn main() {
    println!("1. fib, three ways, same answers");
    print!("   n        ");
    for n in 0..11 {
        print!("{n:>4}");
    }
    println!();
    print!("   fib      ");
    for n in 0..11 {
        print!("{:>4}", fib(n));
    }
    println!();
    print!("   fib_expr ");
    for n in 0..11 {
        print!("{:>4}", fib_expr(n));
    }
    println!();
    print!("   fib_iter ");
    for n in 0..11 {
        print!("{:>4}", fib_iter(n));
    }
    println!();
    let agree = (0..30).all(|n| fib(n) == fib_expr(n) && fib(n) == fib_iter(n));
    println!("   all three agree for n = 0..30? {agree}");
    println!();

    println!("2. When does it panic?");
    println!("   fib(47) = {}", fib_iter(47));
    println!("   u32::MAX = {}", u32::MAX);
    println!("   fib(48) needs {} , which is {} more than u32 can hold.",
        4_807_526_976u64,
        4_807_526_976u64 - u32::MAX as u64);
    let a: u32 = 2_971_215_073; // fib(47)
    let b: u32 = 1_836_311_903; // fib(46)
    println!("   The addition that dies is fib(47) + fib(46) = {a} + {b}:");
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let boom = std::panic::catch_unwind(|| a + b);
    std::panic::set_hook(hook);
    match boom {
        Ok(v) => println!("      a + b            = {v}   (release build: overflow checks off)"),
        Err(_) => println!("      a + b            panicked: attempt to add with overflow"),
    }
    println!("      a.checked_add(b) = {:?}", a.checked_add(b));
    println!("      a.wrapping_add(b)= {}   <- the wrong answer a release build prints",
        a.wrapping_add(b));
    println!("   So: n = 48 in a debug build, and n = 48 gives a plausible, wrong");
    println!("   number in a release build. The second one is the dangerous half.");
    println!();

    println!("3. The same function, one word wider");
    for (name, max) in [
        ("u8", u8::MAX as u128),
        ("u16", u16::MAX as u128),
        ("u32", u32::MAX as u128),
        ("u64", u64::MAX as u128),
        ("u128", u128::MAX),
    ] {
        let (n, value) = last_fitting(max);
        println!("   {name:<5} holds up to fib({n:>3}) = {value}");
    }
    println!("   Widening buys arithmetic, not safety: u128 dies at 187 instead of");
    println!("   48. Picking a type is picking where the program stops being right.");
    println!();

    println!("4. Recursion is not the slow part — recomputation is");
    for n in [10u32, 20, 30] {
        let mut calls = 0u64;
        let value = fib_counted(n, &mut calls);
        println!("   fib({n:>2}) = {value:<8}  recursive calls: {calls:>9}   iterative steps: {n:>2}");
    }
    println!("   Every call recomputes what the sibling call already worked out.");
    println!("   The loop keeps two numbers and never asks the same question twice.");
}
