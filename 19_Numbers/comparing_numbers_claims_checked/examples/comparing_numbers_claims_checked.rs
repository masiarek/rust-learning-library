//! Rust in Action, section 2.3.3 "Comparing numbers": each claim, run.
//! The book's listings are not reproduced; every probe here is new code.
//!
//!   rustc --edition 2024 comparing_numbers_claims_checked.rs -o /tmp/cncc && /tmp/cncc

use std::any::type_name_of_val;
use std::hint::black_box;
use std::panic;

/// Runs `f`, and returns the panic message instead of printing rustc's report.
fn panic_message<T>(f: impl FnOnce() -> T + panic::UnwindSafe) -> Option<String> {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(hook);
    match result {
        Ok(_) => None,
        Err(payload) => payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string())),
    }
}

fn main() {
    println!("1. THE BOOK: two values of different types cannot be compared");
    println!("   for the number types, yes: std implements i32 < i32 and nothing else.");
    println!("   but == and < are traits with a type parameter for the right side,");
    println!("   and std does implement some cross-type pairs:");
    let owned = String::from("abc");
    println!("     String == &str           : {}", owned == "abc");
    println!("     Vec<i32> == [i32; 3]     : {}", vec![1, 2, 3] == [1, 2, 3]);

    println!();
    println!("2. THE BOOK: casting the smaller type to the larger one is safest");
    let a: i32 = -1;
    println!("     -1i32 as u64 < 1u64      : {}   (u64 is larger, and -1 became {})", (a as u64) < 1, a as u64);
    println!("     i64::from(-1i32) < 1i64  : {}", i64::from(a) < 1);
    println!("   larger is not enough: the target must hold every value, sign included.");

    println!();
    println!("3. THE BOOK: 300_i32 as i8 returns 44");
    let n = black_box(300_i32);
    println!("     300_i32 as i8            : {}", n as i8);
    println!("     300 in hex               : {n:#x}; as keeps the low byte, {:#04x}", n as u8);
    println!("     i8::try_from(300)        : {:?}", i8::try_from(n));

    println!();
    println!("4. THE BOOK: try_into() on the u16 returns a Result, and unwrap() crashes if it failed");
    let b: u16 = 100;
    let widened: Result<i32, _> = b.try_into();
    println!("     u16 -> i32 try_into type : {}", type_name_of_val(&widened));
    println!("   the error type is Infallible: this conversion cannot fail. i32::from(b) says so.");
    println!("   a conversion that can fail, unwrapped:");
    let msg = panic_message(|| i8::try_from(black_box(300_i32)).unwrap());
    println!("     panic                    : {}", msg.unwrap_or_default());

    println!();
    println!("5. THE BOOK: f32 and f64 implement only PartialEq; the other number types also Eq");
    println!("     1_i32.cmp(&2)            : {:?}   (Ord, a total order)", 1_i32.cmp(&2));
    println!("     1.0.partial_cmp(&NAN)    : {:?}   (PartialOrd: some pairs have no answer)", 1.0_f64.partial_cmp(&f64::NAN));

    println!();
    println!("6. THE BOOK: 0.1 + 0.2 == 0.3 fails for f64 and passes for f32");
    let (x32, y32, z32): (f32, f32, f32) = (black_box(0.1), black_box(0.2), black_box(0.3));
    let (x64, y64, z64): (f64, f64, f64) = (black_box(0.1), black_box(0.2), black_box(0.3));
    println!("     f32  0.1 + 0.2 = {:08x}   0.3 = {:08x}   equal: {}", (x32 + y32).to_bits(), z32.to_bits(), x32 + y32 == z32);
    println!("     f64  0.1 + 0.2 = {:016x}   0.3 = {:016x}   equal: {}", (x64 + y64).to_bits(), z64.to_bits(), x64 + y64 == z64);
    println!("   true, and f32 was lucky rather than exact. Try 0.1 + 0.6 instead:");
    let (s32, t32, u32_): (f32, f32, f32) = (black_box(0.1), black_box(0.6), black_box(0.7));
    let (s64, t64, u64_): (f64, f64, f64) = (black_box(0.1), black_box(0.6), black_box(0.7));
    println!("     f32  0.1 + 0.6 == 0.7    : {}", s32 + t32 == u32_);
    println!("     f64  0.1 + 0.6 == 0.7    : {}", s64 + t64 == u64_);

    println!();
    println!("7. THE BOOK: compare within f32::EPSILON, shown on 0.1 + 0.1 against 0.2");
    let sum: f32 = black_box(0.1_f32) + black_box(0.1_f32);
    let want: f32 = 0.2;
    println!("     |0.2 - (0.1 + 0.1)| in f32 : {:e}", (want - sum).abs());
    println!("     0.1 + 0.1 == 0.2 in f32    : {}", sum == want);
    println!("   doubling is exact in binary, so the tolerance was never tested.");
    println!("   Where a tolerance is needed, EPSILON is the wrong size once the numbers grow:");
    let big: f64 = black_box(1000.1) + black_box(1000.2);
    let target: f64 = 2000.3;
    let diff = (target - big).abs();
    println!("     |2000.3 - (1000.1 + 1000.2)| : {diff:e}");
    println!("     <= f64::EPSILON              : {}   ({:e})", diff <= f64::EPSILON, f64::EPSILON);
    println!("     <= 1e-12 * 2000.3 (relative) : {}", diff <= 1e-12 * target.abs());

    println!();
    println!("8. THE BOOK: the square root of a negative number, written -42.0.sqrt(), is NaN");
    let unparenthesised = -42.0_f64.sqrt();
    let parenthesised = (-42.0_f64).sqrt();
    println!("     -42.0_f64.sqrt()         : {unparenthesised}");
    println!("     (-42.0_f64).sqrt()       : {parenthesised}");
    println!("   a method call binds tighter than unary minus: the first is -(42.0.sqrt()).");

    println!();
    println!("9. THE BOOK: almost every operation involving NAN returns NAN");
    let nan = black_box(f64::NAN);
    println!("     NAN + 1.0                : {}", nan + 1.0);
    println!("     NAN.max(1.0)             : {}", nan.max(1.0));
    println!("     NAN.min(1.0)             : {}", nan.min(1.0));
    println!("     NAN.powi(0)              : {}", nan.powi(0));
    println!("     1.0_f64.powf(NAN)        : {}", 1.0_f64.powf(nan));
    println!("     NAN.hypot(INFINITY)      : {}", nan.hypot(f64::INFINITY));
    println!("     NAN as i32               : {}", nan as i32);
    println!("     NAN < 1.0                : {}", nan < 1.0);
    println!("   'almost' is doing work: max, min and three more hand back a real number.");

    println!();
    println!("10. THE BOOK: NAN values are never equal");
    println!("     x == x                   : {}", nan == nan);
    println!("     vec![x] == vec![x]       : {}", vec![nan] == vec![nan]);
    println!("     x.total_cmp(&x)          : {:?}", nan.total_cmp(&nan));
    let msg = panic_message(|| assert_eq!(black_box(f64::NAN), black_box(f64::NAN)));
    println!("     assert_eq!(x, x) panics  : {}", msg.is_some());

    println!();
    println!("11. THE BOOK: use is_nan() and is_finite() to crash near the cause");
    let inf: f32 = black_box(1.0_f32) / black_box(0.0_f32);
    println!("     1.0 / 0.0                : {inf}");
    println!("     inf.is_nan()             : {}", inf.is_nan());
    println!("     inf.is_finite()          : {}", inf.is_finite());
    println!("     NAN.is_finite()          : {}", nan.is_finite());
    println!("   is_finite() rejects both; is_nan() alone lets infinity through.");

    println!();
    println!("12. THE BOOK: illegal or undefined operations trigger a CPU exception");
    println!("   not for floats: the lines above divided by zero and took a square root");
    println!("   of a negative number, and this program is still running.");
    let zero = black_box(0_i32);
    let msg = panic_message(|| 1 / zero);
    println!("     1_i32 / 0                : panic, \"{}\"", msg.unwrap_or_default());
    println!("     1_i32.checked_div(0)     : {:?}", 1_i32.checked_div(zero));
    println!("   Rust checks an integer divisor for zero in debug and release builds");
    println!("   alike, so this panic is Rust's own check, not a trap from the CPU.");
}
