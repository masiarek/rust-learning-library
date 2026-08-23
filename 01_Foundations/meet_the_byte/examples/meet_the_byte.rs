//! Meet the byte: what `u8` is, what fits in exactly one, and what a width costs.

use std::mem::size_of;
use std::num::NonZeroU8;

fn rule(title: &str) {
    println!("\n=== {title} ===");
}

fn bits(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:08b}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    rule("one byte, read two ways");
    for b in [0u8, 1, 2, 89, 255] {
        println!("  {b:08b}  =  {b:3}  =  0x{b:02x}");
    }
    println!("  u8::MAX = {}, u8::BITS = {}", u8::MAX, u8::BITS);
    println!("  0b0101_1001 == 89 == 0x59 : {}", 0b0101_1001u8 == 89 && 89u8 == 0x59);

    rule("memory is addressed one byte at a time");
    let cells: [u8; 3] = [0b0101_0011, 0b0101_0101, 0b1011_0111];
    println!("  three bytes            : {}", bits(&cells));
    let first = &cells[0] as *const u8 as usize;
    let second = &cells[1] as *const u8 as usize;
    println!("  addr(cells[1]) - addr(cells[0]) = {}   <- one, always", second - first);
    println!("  size_of::<[u8; 3]>()   = {}", size_of::<[u8; 3]>());
    println!("  the same three as ints : {cells:?}");
    let as_text = cells
        .iter()
        .map(|&c| {
            if c.is_ascii_graphic() {
                format!("'{}'", c as char)
            } else {
                format!("({c}: no ASCII letter)")
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    println!("  the same three as text : {as_text}");
    println!(
        "  str::from_utf8(&cells) : {:?}",
        std::str::from_utf8(&cells).map_err(|e| e.to_string())
    );

    rule("you cannot fetch one bit -- you fetch the byte and mask");
    let b: u8 = 0b0101_1001;
    println!("  b                      = {b:08b}");
    println!("  read  bit 3            = {}", (b >> 3) & 1);
    println!("  test  bit 6            = {}", b & (1 << 6) != 0);
    println!("  set   bit 1            = {:08b}", b | (1 << 1));
    println!("  clear bit 3            = {:08b}", b & !(1 << 3));
    println!("  flip  all              = {:08b}   <- still 8 bits wide", !b);
    println!("  b.count_ones()         = {}", b.count_ones());

    rule("what is exactly one byte");
    println!("  u8                     : {}", size_of::<u8>());
    println!("  bool                   : {}", size_of::<bool>());
    println!("  Option<bool>           : {}   <- the None hides in an unused bit pattern", size_of::<Option<bool>>());
    println!("  Option<NonZeroU8>      : {}   <- the None hides in the zero", size_of::<Option<NonZeroU8>>());
    println!("  char                   : {}   <- NOT one byte: a Unicode scalar value", size_of::<char>());
    println!("  b'F' (a byte literal)  : {} = {:08b}", b'F', b'F');
    println!("  'F' as u32             : {}", 'F' as u32);

    rule("what is more than one byte");
    println!("  i32 / u64 / f64        : {} / {} / {}", size_of::<i32>(), size_of::<u64>(), size_of::<f64>());
    println!("  usize (this target)    : {}", size_of::<usize>());
    println!("  &str  (pointer + len)  : {}", size_of::<&str>());
    println!("  String (ptr+len+cap)   : {}   <- the text itself is on the heap", size_of::<String>());

    rule("a string's length is counted in BYTES");
    let heart = "\u{2764}";
    println!("  heart.len()            = {}   bytes", heart.len());
    println!("  heart.chars().count()  = {}   char", heart.chars().count());
    println!("  heart.as_bytes()       = {:?}", heart.as_bytes());
    println!("  as bits                = {}", bits(heart.as_bytes()));
    println!("  is_char_boundary(1)    = {}   <- &heart[0..1] would panic", heart.is_char_boundary(1));
    let lossy = String::from_utf8(vec![0xFF]);
    println!("  String::from_utf8(0xFF)= {:?}", lossy.map_err(|e| e.to_string()));

    rule("the bill for having a width: overflow");
    let full: u8 = 255;
    println!("  wrapping_add(1)        = {}", full.wrapping_add(1));
    println!("  checked_add(1)         = {:?}", full.checked_add(1));
    println!("  saturating_add(1)      = {}", full.saturating_add(1));
    println!("  overflowing_add(1)     = {:?}", full.overflowing_add(1));
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let plain = std::panic::catch_unwind(|| {
        let x: u8 = std::hint::black_box(255);
        x + 1
    });
    std::panic::set_hook(hook);
    match plain {
        Ok(v) => println!("  plain  255u8 + 1       = {v}   (overflow checks OFF -- a release build)"),
        Err(_) => println!("  plain  255u8 + 1       = panic 'attempt to add with overflow'  (checks ON)"),
    }
    println!("  cfg!(debug_assertions) = {}", cfg!(debug_assertions));

    rule("the two right shifts -- the TYPE decides which one you get");
    println!("  253u8 >> 1             = {}   <- unsigned: pad with 0", 253u8 >> 1);
    println!("  (-3i8) >> 1            = {}   <- signed: pad with the sign bit", (-3i8) >> 1);

    rule("the byte has no order; a MULTI-byte number does");
    println!("  271u16.to_be_bytes()   = {:?}   {}", 271u16.to_be_bytes(), bits(&271u16.to_be_bytes()));
    println!("  271u16.to_le_bytes()   = {:?}   {}", 271u16.to_le_bytes(), bits(&271u16.to_le_bytes()));
    println!("  this target is little  = {}", cfg!(target_endian = "little"));

    rule("eight bytes, many meanings");
    let raw: [u8; 8] = *b"computer";
    println!("  8 x u8       : {raw:?}");
    let u16s: Vec<u16> = raw.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    println!("  4 x u16 LE   : {u16s:?}");
    let u32s: Vec<u32> = raw
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes(c.try_into().expect("chunks_exact(4) yields 4 bytes")))
        .collect();
    println!("  2 x u32 LE   : {u32s:?}");
    println!("  1 x u64 LE   : {}", u64::from_le_bytes(raw));
    println!("  1 x u64 BE   : {}", u64::from_be_bytes(raw));
    println!("  1 x f64 LE   : {:e}", f64::from_le_bytes(raw));
    println!("  8 x ASCII    : {:?}", std::str::from_utf8(&raw).unwrap_or("not utf-8"));
}
