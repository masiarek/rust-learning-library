//! Kata solution: the two decisions a literal makes — which base shows the
//! structure, and which width is narrow enough to be a claim.

fn main() {
    println!("=== part 1: write each field in the base its digits line up with ===");

    // A Unix file mode is nine bits in three groups of three: octal.
    let mode = 0o644;
    println!("  file mode   0o644  = {:>10}  = {:09b}  = rw- r-- r--", mode, mode);
    println!("              same value in hex, 0x1A4, hides the three groups: {:#x}", mode);

    // An RGB colour is three bytes: hex, two digits each.
    let teal: u32 = 0x00_8B_8B;
    println!(
        "  colour      0x008B8B = {:>10}  = r {:3}  g {:3}  b {:3}",
        teal,
        (teal >> 16) & 0xFF,
        (teal >> 8) & 0xFF,
        teal & 0xFF
    );
    println!("              same value in decimal, {}, hides the byte edges", teal);

    // A flag set is one bit per option: binary.
    const READ: u8 = 0b0000_0001;
    const WRITE: u8 = 0b0000_0010;
    const EXEC: u8 = 0b0000_0100;
    let perms = READ | WRITE;
    println!("  flags       0b0000_0011 = {:>4}  = {:08b}   read {} write {} exec {}",
             perms, perms, perms & READ != 0, perms & WRITE != 0, perms & EXEC != 0);

    println!("\n=== part 2: the narrowest type that holds the quantity ===");
    println!("  {:<26} {:>13}  {:<6} {:>21}", "quantity", "worst case", "type", "that type's MAX");
    let rows: [(&str, u128, &str, u128); 5] = [
        ("a STAR score, 0-5", 5, "u8", u8::MAX as u128),
        ("a byte of a ballot scan", 255, "u8", u8::MAX as u128),
        ("ballots in one precinct", 100_000, "u32", u32::MAX as u128),
        ("votes in a US election", 160_000_000, "u32", u32::MAX as u128),
        ("a Unicode code point", 0x10_FFFF, "u32", u32::MAX as u128),
    ];
    for (what, worst, ty, max) in rows {
        println!("  {:<26} {:>13}  {:<6} {:>21}", what, worst, ty, max);
    }

    println!("\n=== the row where the obvious answer is wrong ===");
    println!("  'a precinct is small, u16 is plenty' -- u16::MAX = {}", u16::MAX);
    for ballots in [50_000u32, 100_000u32] {
        match u16::try_from(ballots) {
            Ok(n) => println!("    {ballots:>7} ballots -> u16 holds it: {n}"),
            Err(e) => println!("    {ballots:>7} ballots -> u16 refuses: {e}"),
        }
    }
    println!("  the literal itself is refused at compile time, not at run time --");
    println!("  `let n = 100_000u16;` is rejected by a deny-by-default lint, verbatim:");
    println!("    error: literal out of range for `u16`");
    println!("    = note: the literal `100_000u16` does not fit into the type `u16` whose range is `0..=65535`");
    println!("    = note: `#[deny(overflowing_literals)]` on by default");

    println!("\n=== and the signed row, where the hole is at the bottom ===");
    let margin: i8 = -128;
    println!("  a margin of {margin} fits i8 (MIN is {})", i8::MIN);
    println!("  but its own absolute value does not: {:?}", margin.checked_abs());
    println!("  which is why `i8::MIN.abs()` panics in debug and wraps to {} in release",
             i8::MIN.wrapping_abs());
}
