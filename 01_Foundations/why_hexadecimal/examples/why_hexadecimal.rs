//! Why hexadecimal: the spelling of bits in which the byte boundary never moves.
//!
//!   rustc --edition 2024 why_hexadecimal.rs -o /tmp/whx && /tmp/whx

fn rule(title: &str) {
    println!("\n=== {title} ===");
}

fn main() {
    rule("one number, four spellings");
    let b: u8 = 0xBE;
    println!("  binary   {b:08b}   8 digits, unreadable, but honest about the bits");
    println!("  decimal  {b:>8}   3 digits, and the count changes with the value");
    println!("  octal    {b:>8o}   3 digits, but see below");
    println!("  hex      {b:>8x}   2 digits -- always, for every byte there is");
    println!("  all four are the same u8: {}", b == 0b1011_1110 && b == 190 && b == 0o276);

    rule("the whole reason: one hex digit IS four bits");
    for nibble in [0x0u8, 0x1, 0x7, 0x9, 0xA, 0xF] {
        println!("  0x{nibble:X}  =  {nibble:04b}  =  {nibble:>2}");
    }
    println!("  16 == 2^4, so a digit maps onto a fixed, whole number of bits");

    rule("...so a byte is two digits, and the seam falls between them");
    let byte: u8 = 0xBE;
    println!("  0x{byte:02X}      = {byte:08b}");
    println!("  high nibble = {:04b} = 0x{:X}   <- (byte >> 4)", byte >> 4, byte >> 4);
    println!("  low  nibble = {:04b} = 0x{:X}   <- (byte & 0x0F)", byte & 0x0F, byte & 0x0F);
    println!("  the two hex digits of a byte ARE its two nibbles");

    rule("why not decimal: the boundary wanders");
    for v in [7u8, 42, 100, 255] {
        println!("  {v:>3} decimal is {} digit(s);  0x{v:02X} is always 2", v.to_string().len());
    }
    println!("  10 is not a power of 2, so no decimal digit owns a fixed set of bits");

    rule("why not octal: it fits a 3-bit group, and a byte is not one");
    println!("  one octal digit = 3 bits, and 3 does not divide 8");
    println!("  0xFF as octal   = {:o}  <- 3 digits for 8 bits: the top digit holds only 2", 0xFFu8);
    let perms: u16 = 0o755;
    println!("  where octal still wins: a 9-bit field, which IS three groups of 3");
    println!("    0o755 = {perms:09b} = rwx r-x r-x   (Unix file permissions)");
    println!("    {:03b} {:03b} {:03b}   <- owner / group / other, one digit each",
             (perms >> 6) & 0b111, (perms >> 3) & 0b111, perms & 0b111);

    rule("Rust's literals -- same value, four ways, plus _ anywhere");
    println!("  0xBE == 0b1011_1110 == 0o276 == 190 : {}",
             0xBEu8 == 0b1011_1110u8 && 0o276u8 == 190u8);
    println!("  0xDEAD_BEEFu32      = {}", 0xDEAD_BEEFu32);
    println!("  1_000_000           = {}", 1_000_000);
    println!("  b'A'                = {} = 0x{:X}   <- a byte literal is just a u8", b'A', b'A');

    rule("Rust's formatting");
    let v: u8 = 0x0A;
    println!("  {{:x}}       {:<10}  lowercase, NO padding", format!("{v:x}"));
    println!("  {{:X}}       {:<10}  uppercase", format!("{v:X}"));
    println!("  {{:02x}}     {:<10}  padded to one byte  <- what you almost always want", format!("{v:02x}"));
    println!("  {{:#x}}      {:<10}  with the 0x prefix", format!("{v:#x}"));
    println!("  {{:#04x}}    {:<10}  prefix, and the width COUNTS the prefix", format!("{v:#04x}"));
    println!("  {{:b}}       {:<10}  binary", format!("{v:b}"));
    println!("  {{:#010b}}   {:<10}  prefix + 8 bits = width 10", format!("{v:#010b}"));
    println!("  {{:o}}       {:<10}  octal", format!("{v:o}"));

    rule("TRAP 1: unpadded hex silently loses the byte boundary");
    let a: [u8; 2] = [0x0A, 0xB0];
    let c: [u8; 2] = [0xAB, 0x00];
    let naive = |bs: &[u8]| bs.iter().map(|b| format!("{b:x}")).collect::<String>();
    let good = |bs: &[u8]| bs.iter().map(|b| format!("{b:02x}")).collect::<String>();
    let show = |bs: &[u8]| bs.iter().map(|b| format!("0x{b:02X}")).collect::<Vec<_>>().join(", ");
    println!("  [{}]   naive {:<6} padded {}", show(&a), format!("{:?}", naive(&a)), good(&a));
    println!("  [{}]   naive {:<6} padded {}", show(&c), format!("{:?}", naive(&c)), good(&c));
    println!("  two DIFFERENT arrays, one naive string: {}", naive(&a) == naive(&c));
    println!("  padded, they stay distinct:             {}", good(&a) != good(&c));
    println!("  dropping the 02 throws away the one property you chose hex for");

    rule("TRAP 2: from_str_radix takes a RADIX, not a prefix");
    println!("  u8::from_str_radix(\"ff\", 16)   = {:?}", u8::from_str_radix("ff", 16));
    println!("  u8::from_str_radix(\"0xff\", 16) = {:?}   <- 'x' is not a hex digit",
             u8::from_str_radix("0xff", 16).map_err(|e| e.to_string()));
    println!("  u8::from_str_radix(\"100\", 16)  = {:?}   <- 0x100 does not fit a u8",
             u8::from_str_radix("100", 16).map_err(|e| e.to_string()));
    let printed = format!("{:#x}", 255u8);
    println!("  so {{:#x}} does not round-trip: printed {printed:?}, reads back {:?}",
             u8::from_str_radix(&printed, 16).map_err(|e| e.to_string()));
    let stripped = printed.strip_prefix("0x").unwrap_or(&printed);
    println!("  strip it first:                          {:?}", u8::from_str_radix(stripped, 16));

    rule("TRAP 3: hex of a signed number is two's complement, not a minus sign");
    println!("  format!(\"{{:x}}\", -1i8)  = {:x}         (not \"-1\")", -1i8);
    println!("  format!(\"{{:x}}\", -1i32) = {:x}   (the width shows through)", -1i32);
    println!("  -1i8 as u8             = {}", -1i8 as u8);

    rule("bytes <-> hex, both directions");
    let raw: [u8; 4] = [0x0A, 0xF0, 0x05, 0xBE];
    let text: String = raw.iter().map(|b| format!("{b:02x}")).collect();
    println!("  bytes -> text : {raw:?} -> {text:?}");
    let back: Vec<u8> = (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("two hex digits"))
        .collect();
    println!("  text  -> bytes: {text:?} -> {back:?}");
    println!("  round-trips    : {}", back == raw);
    println!("  and it works BECAUSE every byte spent exactly two characters");
}
