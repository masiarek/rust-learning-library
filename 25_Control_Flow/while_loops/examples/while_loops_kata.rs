//! Kata solution: Collatz step counts with `while`, the input that never
//! reaches the exit condition, and the width that runs out on the way.
//!
//!   rustc --edition 2024 while_loops_kata.rs -o /tmp/wlk && /tmp/wlk

/// Steps from `start` to 1, or `None` when the sequence cannot get there
/// in a `u32`: 0 never reaches 1, and some starts climb past `u32::MAX`.
fn collatz_steps_u32(start: u32) -> Option<u32> {
    if start == 0 {
        return None; // 0 / 2 == 0: `while n != 1` would never become false
    }
    let mut n = start;
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            n.checked_mul(3)?.checked_add(1)?
        };
        steps += 1;
    }
    Some(steps)
}

fn collatz_steps_u64(start: u64) -> Option<u64> {
    if start == 0 {
        return None;
    }
    let mut n = start;
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            n.checked_mul(3)?.checked_add(1)?
        };
        steps += 1;
    }
    Some(steps)
}

fn peak_u64(start: u64) -> u64 {
    let mut n = start;
    let mut peak = n;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        peak = peak.max(n);
    }
    peak
}

fn main() {
    println!("1. Steps to reach 1");
    for start in [1, 6, 7, 27, 97] {
        println!("   start {start:>3}: {:?}", collatz_steps_u32(start));
    }
    println!("   start 1 is Some(0): the condition is false before the first pass.");

    println!();
    println!("2. The start that never reaches the exit condition");
    println!("   start   0: {:?}", collatz_steps_u32(0));
    println!("   0 is even, 0 / 2 is 0, and `while n != 1` stays true forever.");
    println!("   The early return is the only thing between this call and a hang.");

    println!();
    println!("3. The width that runs out");
    let start = 159_487;
    println!("   u32 start {start}: {:?}", collatz_steps_u32(start));
    println!("   u64 start {start}: {:?}", collatz_steps_u64(start as u64));
    println!("   peak on the way  = {}", peak_u64(start as u64));
    println!("   u32::MAX         = {}", u32::MAX);
    println!("   Nothing in the input is large. The climb is.");

    println!();
    println!("4. The first start whose climb leaves u32");
    let mut candidate: u32 = 1;
    while collatz_steps_u32(candidate).is_some() {
        candidate += 1;
    }
    println!("   {candidate}   (found by a while loop whose count nobody knew in advance)");
}
