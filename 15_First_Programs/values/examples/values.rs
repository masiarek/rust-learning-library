//! The built-in scalar types: how you write one down, and how wide it is.
//!
//! Every width below is measured with size_of rather than quoted, so the page
//! cannot drift from the compiler.
//!     rustc --edition 2024 values.rs -o /tmp/values && /tmp/values

use std::mem::size_of;

fn main() {
    println!("1. Signed integers — i8 i16 i32 i64 i128 isize");
    println!("   {:<8} {:>5}  {:>40}  {:>40}", "type", "bytes", "min", "max");
    row_i("i8", size_of::<i8>(), i8::MIN as i128, i8::MAX as i128);
    row_i("i16", size_of::<i16>(), i16::MIN as i128, i16::MAX as i128);
    row_i("i32", size_of::<i32>(), i32::MIN as i128, i32::MAX as i128);
    row_i("i64", size_of::<i64>(), i64::MIN as i128, i64::MAX as i128);
    row_i("i128", size_of::<i128>(), i128::MIN, i128::MAX);
    row_i("isize", size_of::<isize>(), isize::MIN as i128, isize::MAX as i128);
    println!();

    println!("2. Unsigned integers — u8 u16 u32 u64 u128 usize");
    println!("   {:<8} {:>5}  {:>40}  {:>40}", "type", "bytes", "min", "max");
    row_u("u8", size_of::<u8>(), u8::MAX as u128);
    row_u("u16", size_of::<u16>(), u16::MAX as u128);
    row_u("u32", size_of::<u32>(), u32::MAX as u128);
    row_u("u64", size_of::<u64>(), u64::MAX as u128);
    row_u("u128", size_of::<u128>(), u128::MAX);
    row_u("usize", size_of::<usize>(), usize::MAX as u128);
    println!();

    println!("3. isize and usize are the width of a pointer");
    println!("   size_of::<usize>()      = {}", size_of::<usize>());
    println!("   size_of::<*const u8>()  = {}", size_of::<*const u8>());
    println!(
        "   equal? {}   <- that is the definition, not a coincidence of this machine",
        size_of::<usize>() == size_of::<*const u8>()
    );
    println!("   It is the type of a length, an index, and a byte count — which is");
    println!("   why `.len()` gives you a usize and not an i32.");
    println!();

    println!("4. Floats, char and bool");
    println!("   f32     {} bytes   {:>24}", size_of::<f32>(), "3.14, -10.0e20, 2_f32");
    println!("   f64     {} bytes   {:>24}", size_of::<f64>(), "3.14 (the fallback)");
    println!("   char    {} bytes   {:>24}", size_of::<char>(), "'a', 'α', '∞'");
    println!("   bool    {} bytes   {:>24}", size_of::<bool>(), "true, false");
    println!("   char is 32 bits wide because it holds one Unicode scalar value,");
    println!("   and the largest of those is U+10FFFF = {}.", 0x10FFFFu32);
    println!("   bool is 8 bits wide because a byte is the smallest addressable");
    println!("   unit — it carries one bit of information in one byte of space.");
    println!();

    println!("5. Writing one down");
    let plain = 1000;
    let grouped = 1_000;
    let odd = 10_00;
    println!("   1000 == 1_000 == 10_00 ?  {}", plain == grouped && grouped == odd);
    println!("   Underscores are legibility only. The compiler removes them.");
    let suffixed = 123_i64;
    let unspaced = 123i64;
    println!("   123_i64 == 123i64 ?       {}", suffixed == unspaced);
    println!("   The suffix is the type, written on the literal instead of the let.");
    println!("      let a = 123_i64;   is   let a: i64 = 123;");
    println!();

    println!("6. Other bases, and the byte literal");
    println!("   decimal      65        {}", 65);
    println!("   hex          0x41      {}", 0x41);
    println!("   octal        0o101     {}", 0o101);
    println!("   binary       0b100_0001 {}", 0b100_0001);
    println!("   byte         b'A'      {}", b'A');
    println!("   all the same u8: {}", 65 == 0x41 && 0x41 == 0o101 && 0o101 == 0b100_0001 && 0b100_0001 == b'A' as i32);
    println!("   b'A' is a u8, not a char: {} vs {}", size_of::<u8>(), size_of::<char>());
    println!();

    println!("7. The two fallbacks");
    let n = 1;
    let f = 1.0;
    println!("   let n = 1;     {}   <- i32 when nothing else decides", std::any::type_name_of_val(&n));
    println!("   let f = 1.0;   {}   <- f64 when nothing else decides", std::any::type_name_of_val(&f));
    println!("   In an error message these appear as {{integer}} and {{float}}:");
    println!("      let x = 3.14; let y = 20; assert_eq!(x, y);");
    println!("      error[E0277]: can't compare `{{float}}` with `{{integer}}`");
    println!();

    println!("8. The width is a promise, and it is checked");
    println!("      let big: u8 = 1_000_000;");
    println!("      error: literal out of range for `u8`  (range is 0..=255)");
    let almost = u8::MAX;
    let wrapped = almost.wrapping_add(1);
    let checked = almost.checked_add(1);
    let saturated = almost.saturating_add(1);
    println!("   u8::MAX = {almost}, and one more is:");
    println!("      almost + 1              panics in a debug build, wraps in release");
    println!("      wrapping_add(1)  = {wrapped}");
    println!("      checked_add(1)   = {checked:?}");
    println!("      saturating_add(1)= {saturated}");
    println!("   Four answers, and the type does not pick for you — you do.");
}

fn row_i(name: &str, bytes: usize, min: i128, max: i128) {
    println!("   {name:<8} {bytes:>5}  {min:>40}  {max:>40}");
}

fn row_u(name: &str, bytes: usize, max: u128) {
    println!("   {name:<8} {bytes:>5}  {:>40}  {max:>40}", 0);
}
