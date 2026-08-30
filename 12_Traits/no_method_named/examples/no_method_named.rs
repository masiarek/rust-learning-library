//! `E0599` covers three unrelated mistakes. This program is all three of them
//! already fixed, so each section shows what the missing piece was buying.
//!
//!   rustc --edition 2024 no_method_named.rs -o /tmp/nmn && /tmp/nmn

use std::fmt;
use std::io::BufRead; // 2. IMPORTED: `.lines()` on a byte slice comes from here

// 3. IMPLEMENTED: `.clone()` is in the prelude, but only exists on a type
//    that has the impl. `derive` writes it.
#[derive(Clone, Debug)]
struct Ballot {
    voter: &'static str,
    star: u8,
}

// 1. EXISTS: an inherent method. Nothing imports this and no trait names it;
//    it is simply written, or it is not there.
impl Ballot {
    fn doubled(&self) -> u8 {
        self.star * 2
    }
}

// 4. The bound satisfied: `.to_string()` is not implemented by hand. It
//    arrives through a blanket impl of `ToString` for every `T: Display`,
//    so writing `Display` is what makes the method appear.
impl fmt::Display for Ballot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} gave {} star(s)", self.voter, self.star)
    }
}

fn main() {
    let b = Ballot { voter: "Ada", star: 5 };

    println!("1. The method EXISTS because an inherent impl wrote it");
    println!("   b.doubled() = {}", b.doubled());

    println!();
    println!("2. The trait is IN SCOPE, so its method appears on a type std wrote");
    let text = "first\nsecond\nthird".as_bytes(); // a &[u8] is a BufRead
    let lines: Vec<String> = text.lines().map(|l| l.unwrap()).collect();
    println!("   text.lines().count() = {}", lines.len());
    println!("   collected            = {lines:?}");

    println!();
    println!("3. The trait is IMPLEMENTED, so .clone() exists on our own type");
    let mut copy = b.clone();
    copy.star = 0;
    println!("   original = {b:?}");
    println!("   clone    = {copy:?}");

    println!();
    println!("4. The BOUND is satisfied, so the blanket impl hands us .to_string()");
    println!("   b.to_string() = {}", b.to_string());
    println!("   ^ nothing here implements ToString; implementing Display did it.");
}
