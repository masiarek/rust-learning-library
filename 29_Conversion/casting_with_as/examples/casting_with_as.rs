//! `as` always succeeds, which is the problem.
//!
//!   rustc --edition 2024 casting_with_as.rs -o /tmp/cast && /tmp/cast

fn main() {
    println!("1. Narrowing keeps the low bits and drops the rest");
    println!("   300u32 as u8   = {}   (300 = 0b1_0010_1100; the low 8 bits are 0b0010_1100)",
             300u32 as u8);
    println!("   256u32 as u8   = {}", 256u32 as u8);
    println!("   1000u32 as u8  = {}", 1000u32 as u8);
    println!("   No panic, no warning, no Result. In a debug build too: this is");
    println!("   not an arithmetic overflow, it is a defined truncation.");

    println!();
    println!("2. Signedness is a reinterpretation of the same bits");
    println!("   -1i32 as u32   = {}", -1i32 as u32);
    println!("   -1i8 as u8     = {}", -1i8 as u8);
    println!("   200u8 as i8    = {}", 200u8 as i8);
    println!("   255u8 as i8    = {}", 255u8 as i8);
    println!("   Two's complement, unchanged, read the other way round. A length");
    println!("   that went negative and was cast to usize becomes about 18");
    println!("   quintillion, which is how a bounds check gets passed by accident.");

    println!();
    println!("3. Float to integer saturates, and NaN becomes zero");
    println!("   3.9f64 as i32     = {}   (truncates toward zero, never rounds)", 3.9f64 as i32);
    println!("   -3.9f64 as i32    = {}", -3.9f64 as i32);
    println!("   1e10f64 as i32    = {}   <- saturates at i32::MAX", 1e10f64 as i32);
    println!("   -1e10f64 as i32   = {}   <- and at i32::MIN", -1e10f64 as i32);
    println!("   f64::NAN as i32   = {}", f64::NAN as i32);
    println!("   Saturation has been the defined behaviour since Rust 1.45; before");
    println!("   that this was undefined and produced whatever LLVM felt like.");

    println!();
    println!("4. Integer to float loses precision without saying so");
    let big: i64 = 16_777_217;              // 2^24 + 1
    println!("   16_777_217i64 as f32 = {}", big as f32);
    println!("   back again:            {}", big as f32 as i64);
    println!("   f32 has 24 bits of mantissa, so 2^24 + 1 is not representable and");
    println!("   the nearest value is 2^24. The cast is silent in both directions.");
    let huge: i64 = i64::MAX;
    println!("   i64::MAX as f64 as i64 = {}", huge as f64 as i64);
    println!("   i64::MAX               = {huge}");

    println!();
    println!("5. The non-numeric casts, which are the safe ones");
    println!("   'A' as u32   = {}", 'A' as u32);
    println!("   '€' as u32   = {}", '€' as u32);
    println!("   65u8 as char = {}   <- only u8 may be cast to char, and always fits", 65u8 as char);
    println!("   true as i32  = {}, false as i32 = {}", true as i32, false as i32);
    println!("   A `char` is a Unicode scalar value, so char -> u32 never loses");
    println!("   anything. The reverse needs char::from_u32, which returns Option:");
    println!("   char::from_u32(0xD800) = {:?}   <- a surrogate is not a scalar value",
             char::from_u32(0xD800));

    println!();
    println!("6. What to write instead");
    println!("   widening, cannot fail   u64::from(n)         From");
    println!("   narrowing, might fail   u8::try_from(n)?     TryFrom");
    println!("   truncation is intended  n as u8              as, with a comment");
    println!("   float to int, rounded   n.round() as i64     say which rounding");
    println!("   The rule of thumb: if you cannot say out loud what `as` does to");
    println!("   the out-of-range case, you wanted try_from.");
    let n: i32 = 200;
    println!("   u8::try_from(200i32)  = {:?}", u8::try_from(n));
    println!("   u8::try_from(300i32)  = {:?}", u8::try_from(300i32));
}
