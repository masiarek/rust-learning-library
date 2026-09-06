//! Why a `char` is 32 bits: the largest Unicode scalar value needs 21 of them,
//! 21 is not a width a machine can address, and the 11 left over are not waste.
//!
//!   rustc --edition 2024 why_char_is_32_bits.rs -o /tmp/wc32 && /tmp/wc32

use std::mem::size_of;

fn main() {
    println!("1. The measurement");
    let max = char::MAX as u32;
    println!("   size_of::<char>()  {} bytes = {} bits", size_of::<char>(), size_of::<char>() * 8);
    println!("   char::MAX          U+{max:X} = {max}");
    println!("   in binary          {max:b}");
    println!("   bits it needs      {}", u32::BITS - max.leading_zeros());

    println!("\n2. Why not 16 bits");
    for c in ['*', '字', '😀'] {
        let n = c as u32;
        let verdict = if n <= 0xFFFF { "fits in 16" } else { "does NOT fit in 16" };
        println!("   U+{:<6X} {:>2} bits  {:<19} {c:?}", n, u32::BITS - n.leading_zeros(), verdict);
    }
    let total = (0..=max).filter(|&n| char::from_u32(n).is_some()).count();
    let in_bmp = (0..=0xFFFF).filter(|&n| char::from_u32(n).is_some()).count();
    println!("   16 bits reaches {in_bmp} scalar values of {total} -- under 6% of the space.");
    println!("   Almost all everyday text lives in those, which is why 16 bits looked");
    println!("   like enough in 1991. It was not, and UTF-16 pairs up the rest.");

    println!("\n3. Not every 32-bit pattern is a char");
    for n in [0x41u32, 0xD800, 0xFFFE, 0x10FFFF, 0x110000] {
        println!("   char::from_u32(0x{n:06X}) = {:?}", char::from_u32(n));
    }
    println!("   0xD800 is half of a UTF-16 surrogate pair -- never a character alone.");
    println!("   0x110000 is one past the top. Neither is a bit pattern a char can hold.");
    println!("   U+FFFE is accepted: a permanent noncharacter is still a scalar value,");
    println!("   and `char` is defined by the scalar range, not by what Unicode assigns.");

    println!("\n4. What the spare bit patterns buy");
    println!("   21 bits could hold       {} values", 1u32 << 21);
    println!("   Unicode actually defines {total}");
    println!("   32 bits could hold       {} values", u32::MAX as u64 + 1);
    println!("   size_of::<Option<char>>()         = {}", size_of::<Option<char>>());
    println!("   size_of::<Option<Option<char>>>() = {}", size_of::<Option<Option<char>>>());
    println!("   size_of::<Option<u32>>()          = {}   <- u32 has no spare patterns,",
        size_of::<Option<u32>>());
    println!("      so its Option needs a tag beside the value, and then padding.");

    println!("\n5. Four bytes is what a char COSTS, not what text costs");
    let s = "Hello, 字!";
    let chars: Vec<char> = s.chars().collect();
    println!("   {s:?}");
    println!("   as a &str, UTF-8           {} bytes", s.len());
    println!("   as [char], one per scalar  {} bytes", chars.len() * size_of::<char>());
    println!("   Text is stored UTF-8 for this reason; char is the decoded form you");
    println!("   compare, classify and range over.");

    println!("\n6. The five literals from the primitive-types table");
    println!("   code pt   UTF-8  written        prints back as");
    for c in ['*', '\n', '字', '\x7f', '\u{CA0}'] {
        let written = match c {
            '*' => "'*'",
            '\n' => r"'\n'",
            '字' => "'字'",
            '\x7f' => r"'\x7f'",
            _ => r"'\u{CA0}'",
        };
        println!("   U+{:<6X} {} B      {:<14} {:?}", c as u32, c.len_utf8(), written, c);
    }
    println!("   Every one of them is 4 bytes as a char. The UTF-8 column is what the");
    println!("   same character costs inside a String -- and \\x is ASCII-only, so");
    println!("   anything above U+007F has to be written \\u{{...}}.");
}
