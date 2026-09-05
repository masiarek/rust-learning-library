//! Every claim RFC 69 made, re-checked against the compiler that shipped it.
//!
//! RFC 69 (Simon Sapin, 2014-05-05) asked for `b'A'` and `b"A"`. It asserted
//! four equalities, promised one type, and closed with three questions it did
//! not answer. This program runs all of that.

use std::mem::size_of;

fn main() {
    println!("RFC 69'S FOUR ASSERTED EQUALITIES");
    println!("  b'A'    == 65u8         {}", b'A' == 65u8);
    println!("  b'\\t'   == 9u8          {}", b'\t' == 9u8);
    println!("  b'\\xFF' == 0xFFu8       {}", b'\xFF' == 0xFFu8);
    println!("  b\"A\\t\\xFF\"              {:?}", b"A\t\xFF");

    println!();
    println!("THE TYPE THE RFC PROMISED IS NOT THE TYPE THAT SHIPPED");
    let shipped: &'static [u8; 3] = b"A\t\xFF";
    println!("  RFC   &'static [u8]     a slice: pointer AND length, carried at run time");
    println!("  today &'static [u8; 3]  an array reference: the length is in the type");
    println!(
        "  a slice reference is exactly twice as wide   {}",
        size_of::<&[u8]>() == 2 * size_of::<&[u8; 3]>()
    );
    let as_the_rfc_wrote_it: &'static [u8] = shipped;
    println!(
        "  ...and it still coerces, so the RFC's own signature accepts it   {} bytes",
        as_the_rfc_wrote_it.len()
    );

    println!();
    println!("Q1  \"Should there be raw byte string literals?\"   ANSWERED YES");
    let pdf = br"<< /Title (FizzBuzz \(Part one\)) >>";
    println!("  br\"...\"    {} bytes  {}", pdf.len(), show(pdf));
    let quoted = br#"a "quoted" thing"#;
    println!("  br#\"...\"#  {} bytes  {}", quoted.len(), show(quoted));

    println!();
    println!("Q2  \"Should control characters be disallowed?\"    ANSWERED NO");
    let tabbed = b"a	b"; // a literal tab, typed between the quotes
    println!("  b\"a<TAB>b\"           {:?}", tabbed);
    let nulled = b"a\0b";
    println!("  b\"a\\0b\"              {:?}", nulled);

    println!();
    println!("Q3  \"Should bytes!() be removed?\"                 ANSWERED YES");
    println!("  b\"A\"                 {:?}   the macro is gone; this replaced it", b"A");

    println!();
    println!("THE RFC'S EXAMPLE COMPILES AGAIN, AND NO LONGER MEANS WHAT IT SAID");
    println!("            b'a' .. b'z'   b'a' ..= b'z'");
    for byte in [b'a', b'y', b'z'] {
        println!(
            "  {:?}       {:<14} {}",
            byte as char,
            exclusive(byte),
            inclusive(byte)
        );
    }
}

/// The RFC's own snippet, typed exactly as it was written in 2014.
fn exclusive(byte: u8) -> &'static str {
    match byte {
        b'a'..b'z' => "lower",
        _ => "other",
    }
}

/// What it meant in 2014, spelled the way you have to spell it now.
fn inclusive(byte: u8) -> &'static str {
    match byte {
        b'a'..=b'z' => "lower",
        _ => "other",
    }
}

/// Byte strings are not text, so there is no `Display` — say so by hand.
fn show(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| b as char).collect()
}
