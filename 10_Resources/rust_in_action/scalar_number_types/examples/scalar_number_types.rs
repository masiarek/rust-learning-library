// The claims in Rust in Action, tables 2.1 and 2.2 and the text between them,
// checked one at a time.
fn main() {
    // 1. Table 2.2: the number 20 in three types.
    println!("1. u32 {:032b}", 20_u32);
    println!("   i8  {:>32}", format!("{:08b}", 20_i8));
    println!("   f32 {:032b}", 20_f32.to_bits());

    // 2. Every fixed-width integer, including the two Table 2.1 leaves out.
    println!("2. type  bits  min ..= max");
    println!("   i8    {:>4}  {} ..= {}", i8::BITS, i8::MIN, i8::MAX);
    println!("   i16   {:>4}  {} ..= {}", i16::BITS, i16::MIN, i16::MAX);
    println!("   i32   {:>4}  {} ..= {}", i32::BITS, i32::MIN, i32::MAX);
    println!("   i64   {:>4}  {} ..= {}", i64::BITS, i64::MIN, i64::MAX);
    println!("   i128  {:>4}  {} ..= {}", i128::BITS, i128::MIN, i128::MAX);
    println!("   u8    {:>4}  {} ..= {}", u8::BITS, u8::MIN, u8::MAX);
    println!("   u16   {:>4}  {} ..= {}", u16::BITS, u16::MIN, u16::MAX);
    println!("   u32   {:>4}  {} ..= {}", u32::BITS, u32::MIN, u32::MAX);
    println!("   u64   {:>4}  {} ..= {}", u64::BITS, u64::MIN, u64::MAX);
    println!("   u128  {:>4}  {} ..= {}", u128::BITS, u128::MIN, u128::MAX);

    // 3. "Twice as high": one more than twice, and unsigned starts at zero.
    println!(
        "3. u8::MAX = {} = 2 * i8::MAX + 1 = {}; u8::MIN = {}",
        u8::MAX,
        2 * i8::MAX as u16 + 1,
        u8::MIN,
    );

    // 4. The same eight bits read as unsigned and as signed.
    let bits = 0b1110_1100_u8;
    println!("4. {bits:08b} as u8 = {bits}, as i8 = {}", bits as i8);

    // 5. The special float patterns: sign, 8 exponent bits, 23 fraction bits.
    println!("5. value sign exponent fraction");
    for (name, x) in [
        ("inf", f32::INFINITY),
        ("-inf", f32::NEG_INFINITY),
        ("0.0", 0.0_f32),
        ("-0.0", -0.0_f32),
    ] {
        let b = x.to_bits();
        println!("   {name:>5} {}    {:08b} {:023b}", b >> 31, (b >> 23) & 0xff, b & 0x7f_ffff);
    }

    // 6. Several patterns for one number, inside a single type.
    println!(
        "6. 0.0 == -0.0: {}, same bits: {}",
        0.0_f32 == -0.0_f32,
        0.0_f32.to_bits() == (-0.0_f32).to_bits(),
    );
    let nan_a = f32::from_bits(0x7fc0_0000);
    let nan_b = f32::from_bits(0x7fc0_0001);
    println!(
        "   0x7fc00000 and 0x7fc00001 are both NaN: {} {}; a NaN equals itself: {}",
        nan_a.is_nan(),
        nan_b.is_nan(),
        nan_a == nan_a,
    );

    // 7. "Real numbers": most decimals have no exact pattern at all.
    println!("7. 0.1_f32 is stored as {:.20}", 0.1_f32);
}
