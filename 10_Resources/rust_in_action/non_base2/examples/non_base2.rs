// The claims around Rust in Action, listing 2.4 (ch2/ch2-non-base2.rs),
// checked one at a time.
fn main() {
    // 1. Three prefixes, three values. The prefix is spelling, not type.
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;
    println!("1. {three} {thirty} {three_hundred}"); // 1. 3 30 300
    println!(
        "   0x12C == 0x12c: {}; their types: {} {} {}",
        0x12C == 0x12c,
        std::any::type_name_of_val(&three),
        std::any::type_name_of_val(&thirty),
        std::any::type_name_of_val(&three_hundred),
    );

    // 2. The book's positional arithmetic, digit by digit.
    println!(
        "2. 2*1 + 1*1 = {}, 8*3 + 1*6 = {}, 256*1 + 16*2 + 1*12 = {}",
        2 * 1 + 1 * 1,
        8 * 3 + 1 * 6,
        256 * 1 + 16 * 2 + 1 * 12,
    );

    // 3. A leading zero alone is not octal: 036 is thirty-six.
    println!("3. 036 = {}, 0o36 = {}", 036, 0o36);

    // 4. A hex literal swallows a suffix made of hex digits.
    let looks_like_f32 = 0x1f32;
    println!(
        "4. 0x1f32 = {looks_like_f32}, an {}; 0x1_u32 = {}",
        std::any::type_name_of_val(&looks_like_f32),
        0x1_u32,
    );

    // 5. Printing in a base: the letter picks it, # adds the prefix, 0N pads.
    println!(
        "5. {three_hundred:b} {three_hundred:#b} | {three_hundred:o} {three_hundred:#o} | {three_hundred:x} {three_hundred:X} {three_hundred:#x} {three_hundred:#06x}"
    );

    // 6. Reading a base back: from_str_radix takes the base, not the prefix.
    println!(
        "6. {:?} | {:?}",
        i32::from_str_radix("12C", 16),
        i32::from_str_radix("0x12C", 16),
    );

    // 7. A negative number prints its two's-complement bits, with no minus sign.
    println!("7. -3_i8: {:b} {:x}", -3_i8, -3_i8);
}
