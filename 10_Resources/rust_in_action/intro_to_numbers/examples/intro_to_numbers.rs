// The claims around Rust in Action, listing 2.3 (ch2/ch2-intro-to-numbers.rs)
// and section 2.3.1, checked one at a time.
use std::any::type_name_of_val;

fn main() {
    // 1. The unannotated literal takes its type from the sum it joins.
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;
    let addition = twenty + twenty_one + twenty_two;
    let twenty_beside_i64 = 20;
    let _ = twenty_beside_i64 + 22_i64;
    println!("1. {twenty} + {twenty_one} + {twenty_two} = {addition}"); // 20 + 21 + 22 = 63
    println!(
        "   twenty is {}; the same 20 added to an i64 is {}",
        type_name_of_val(&twenty),
        type_name_of_val(&twenty_beside_i64),
    );

    // 2. The i64 on one_million carries weight: 10^12 does not fit in an i32.
    let one_million: i64 = 1_000_000;
    let as_i32: i32 = 1_000_000;
    println!("2. i64: {}", one_million.pow(2)); // 1000000000000
    println!(
        "   i32: checked_pow(2) is {:?}, because i32::MAX is {}",
        as_i32.checked_pow(2),
        i32::MAX,
    );

    // 3. An array has one element type, and its suffixed members choose it.
    let forty_twos = [42.0, 42f32, 42.0_f32];
    let unsuffixed = [42.0, 42.0];
    println!(
        "3. forty_twos is {}; an unsuffixed pair is {}",
        type_name_of_val(&forty_twos),
        type_name_of_val(&unsuffixed),
    );

    // 4. {:02} pads to at least two characters, and 42 already has two.
    println!("4. {:02} | {:02} | {:05.1}", forty_twos[0], 4.0_f32, forty_twos[0]);

    // 5. A method on a literal float needs the suffix; a path call does not.
    println!(
        "5. 24.5_f32.round() = {}, f32::round(24.5) = {}, 24.5_f32.round_ties_even() = {}",
        24.5_f32.round(),
        f32::round(24.5),
        24.5_f32.round_ties_even(),
    );

    // 6. A type's name counts bits; size_of counts bytes.
    println!(
        "6. i16: {} bits, {} bytes; i32: {} bits, {} bytes; i64: {} bits, {} bytes",
        i16::BITS,
        size_of::<i16>(),
        i32::BITS,
        size_of::<i32>(),
        i64::BITS,
        size_of::<i64>(),
    );

    // 7. No numeric conversion happens unless you write one. Other kinds can.
    let short: i16 = 20;
    let long: i32 = short.into();
    let from = i32::from(short) + 1;
    let owned = String::from("text");
    let borrowed: &str = &owned; // &String becomes &str, and no call is written
    println!("7. {long} {from} {borrowed}");

    // 8. One + token, many types: each pair of operand types is a trait impl.
    println!(
        "8. {} | {} | {}",
        20 + 22,
        20.5 + 21.5,
        String::from("forty") + "-two",
    );

    // 9. Indexing starts at 0; get() asks without panicking.
    println!(
        "9. forty_twos[0] = {}, forty_twos.get(3) = {:?}",
        forty_twos[0],
        forty_twos.get(3),
    );
}
