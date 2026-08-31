//! How a number is written down in Rust: the base prefix, the underscore, the
//! suffix, the byte literal, and the float. Every value here is one literal
//! printed back, so the page cannot claim a spelling the compiler rejects.

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    println!("=== the 0 says 'not decimal', the letter names the base ===");
    println!("  0x2A   hex     = {}", 0x2A);
    println!("  0o77   octal   = {}", 0o77);
    println!("  0b101  binary  = {}", 0b101);
    println!("  42     decimal = {}", 42);
    println!("  and all four spell the same u8 when the number is the same:");
    println!(
        "    0xFF == 0o377 == 0b1111_1111 == 255 : {}",
        0xFF == 0o377 && 0o377 == 0b1111_1111 && 0b1111_1111 == 255
    );
    println!("  case does not matter in hex: 0xff == 0xFF : {}", 0xff == 0xFF);

    println!("\n=== a leading zero on its own means nothing ===");
    println!("  0755  = {}   <- decimal seven hundred fifty-five, NOT octal", 0755);
    println!("  0o755 = {}   <- octal: rwx r-x r-x", 0o755);
    println!("  0000042 = {}  <- leading zeros are legal and inert", 0000042);

    println!("\n=== the underscore is for your eyes, anywhere in the literal ===");
    println!("  1_000_000     = {}", 1_000_000);
    println!("  0xDEAD_BEEFu32 = {}", 0xDEAD_BEEFu32);
    println!("  0b1011_1110   = {}", 0b1011_1110);
    println!("  1_0_0         = {}   <- legal, and nobody should", 1_0_0);

    println!("\n=== the suffix is the type, written on the number ===");
    let a = 57u8;
    let b = 57_u8;
    let c: u8 = 57;
    println!("  57u8  = {} : {}", a, type_of(&a));
    println!("  57_u8 = {} : {}   <- same literal, the _ is optional", b, type_of(&b));
    println!("  let c: u8 = 57 -> {} : {}   <- same instruction, said on the let", c, type_of(&c));
    let d = 57;
    let e = 5.7;
    println!("  57    = {} : {}   <- the integer fallback", d, type_of(&d));
    println!("  5.7   = {} : {}   <- the float fallback", e, type_of(&e));

    println!("\n=== b in front of a quote means bytes, not text ===");
    let byte = b'A';
    let text = 'A';
    let bytes = b"Hi";
    let string = "Hi";
    println!("  b'A'  = {:<3} : {:<10} {} byte", byte, type_of(&byte), size_of_val(&byte));
    println!("  'A'   = {:<3} : {:<10} {} bytes", text as u32, type_of(&text), size_of_val(&text));
    println!("  b\"Hi\" = {:?} : {}", bytes, type_of(&bytes));
    println!("  \"Hi\"  = {:?} : {}", string, type_of(&string));
    println!("  b'A' is a NUMBER you may do arithmetic on: b'A' + 2 = {} = '{}'",
             b'A' + 2, (b'A' + 2) as char);

    println!("\n=== floats: the dot is required, and there is no unsigned one ===");
    let f1 = 2.0;
    let f2 = 2.;
    let f3 = 1e6;
    let f4 = 2.0f32;
    let f5 = 2_f32;
    println!("  2.0   = {} : {}", f1, type_of(&f1));
    println!("  2.    = {} : {}   <- legal; 2 alone would be an integer", f2, type_of(&f2));
    println!("  1e6   = {} : {}", f3, type_of(&f3));
    println!("  2.0f32 = {} : {}", f4, type_of(&f4));
    println!("  2_f32  = {} : {}   <- no dot needed once the suffix says float", f5, type_of(&f5));
    println!("  there is no uf32/uf64: every float carries a sign bit, always");
    println!("    -0.0 == 0.0                    : {}", -0.0 == 0.0);
    println!("    (-0.0f64).is_sign_negative()   : {}   <- the bit is there either way",
             (-0.0f64).is_sign_negative());

    println!("\n=== how much a float remembers ===");
    println!("  f32  {:>2} significand bits, ~{} reliable decimal digits (single precision)",
             f32::MANTISSA_DIGITS, f32::DIGITS);
    println!("  f64  {:>2} significand bits, ~{} reliable decimal digits (double precision)",
             f64::MANTISSA_DIGITS, f64::DIGITS);
    println!("  0.1f32 = {:.20}", 0.1f32);
    println!("  0.1f64 = {:.20}   <- same wrong answer, 29 bits further right", 0.1f64);

    println!("\n=== what the width promises, by formula ===");
    println!("  signed   iN : -2^(N-1)  ..=  2^(N-1) - 1");
    println!("  unsigned uN :         0  ..=  2^N - 1");
    println!("  {:<6} {:>21} {:>21}   formula check", "type", "MIN", "MAX");
    macro_rules! row {
        ($t:ty, $bits:expr, signed) => {
            println!(
                "  {:<6} {:>21} {:>21}   {}",
                stringify!($t),
                <$t>::MIN,
                <$t>::MAX,
                (<$t>::MIN as i128 == -(1i128 << ($bits - 1)))
                    && (<$t>::MAX as i128 == (1i128 << ($bits - 1)) - 1)
            );
        };
        ($t:ty, $bits:expr, unsigned) => {
            println!(
                "  {:<6} {:>21} {:>21}   {}",
                stringify!($t),
                <$t>::MIN,
                <$t>::MAX,
                (<$t>::MIN == 0) && (<$t>::MAX as u128 == (1u128 << $bits) - 1)
            );
        };
    }
    row!(i8, 8, signed);
    row!(i16, 16, signed);
    row!(i32, 32, signed);
    row!(i64, 64, signed);
    row!(u8, 8, unsigned);
    row!(u16, 16, unsigned);
    row!(u32, 32, unsigned);
    row!(u64, 64, unsigned);

    println!("\n=== the asymmetry the formula predicts ===");
    println!("  i8::MIN = {}, i8::MAX = {}   <- one more below zero than above", i8::MIN, i8::MAX);
    println!("  i8::MIN.checked_neg()  = {:?}   <- -(-128) is not an i8", i8::MIN.checked_neg());
    println!("  i8::MIN.checked_abs()  = {:?}   <- neither is its absolute value", i8::MIN.checked_abs());
    println!("  u8 has no such hole: 0 is its own negation, and there is nothing below it");
}
