//! Kata solution: undo `escape_ascii`.
//!
//! std prints a byte string with `escape_ascii()` and has nothing that reads one
//! back. The reverse is short; proving it exact is the exercise — every one of
//! the 65,536 two-byte strings has to survive the round trip.
//!
//!   rustc --edition 2024 printing_bytes_kata.rs -o /tmp/pbk && /tmp/pbk

use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
enum UnescapeError {
    /// `escape_ascii` only ever writes ASCII.
    NotAscii(usize),
    /// A `\` with nothing after it.
    LoneBackslash(usize),
    /// `\x` without two hex digits after it.
    ShortHex(usize),
    /// A backslash escape `escape_ascii` never writes, such as `\q`.
    UnknownEscape(usize, char),
}

fn unescape_ascii(text: &str) -> Result<Vec<u8>, UnescapeError> {
    let src = text.as_bytes();
    if let Some(at) = src.iter().position(|b| !b.is_ascii()) {
        return Err(UnescapeError::NotAscii(at));
    }
    let hex = |at: usize| src.get(at).and_then(|&d| (d as char).to_digit(16));
    let mut out = Vec::with_capacity(src.len());
    let mut i = 0;
    while i < src.len() {
        if src[i] != b'\\' {
            out.push(src[i]);
            i += 1;
            continue;
        }
        match src.get(i + 1) {
            None => return Err(UnescapeError::LoneBackslash(i)),
            Some(b'n') => out.push(b'\n'),
            Some(b'r') => out.push(b'\r'),
            Some(b't') => out.push(b'\t'),
            Some(&q @ (b'\\' | b'\'' | b'"')) => out.push(q),
            Some(b'x') => match (hex(i + 2), hex(i + 3)) {
                (Some(hi), Some(lo)) => {
                    out.push((hi * 16 + lo) as u8);
                    i += 2; // the two digits, on top of the two below
                }
                _ => return Err(UnescapeError::ShortHex(i)),
            },
            Some(&other) => return Err(UnescapeError::UnknownEscape(i, other as char)),
        }
        i += 2;
    }
    Ok(out)
}

fn main() {
    println!("=== the lesson's string, there and back ===");
    let data = "Real 🐍".as_bytes();
    let shown = data.escape_ascii().to_string();
    println!("  escaped   {shown}");
    println!("  back      {:?}", unescape_ascii(&shown));
    println!("  equal     {}", unescape_ascii(&shown) == Ok(data.to_vec()));

    println!("\n=== exact, proved rather than asserted ===");
    let singles = (0..=255u8)
        .filter(|&b| unescape_ascii(&[b].escape_ascii().to_string()) == Ok(vec![b]))
        .count();
    println!("  single bytes that round-trip    : {singles} of 256");
    let mut pairs = 0;
    let mut forms = BTreeSet::new();
    for x in 0..=255u8 {
        for y in 0..=255u8 {
            let s = [x, y].escape_ascii().to_string();
            if unescape_ascii(&s) == Ok(vec![x, y]) {
                pairs += 1;
            }
            forms.insert(s);
        }
    }
    println!("  two-byte strings that round-trip: {pairs} of 65536");
    println!("  distinct escaped forms          : {}   <- one per input, so nothing collides", forms.len());

    println!("\n=== the inputs escape_ascii never writes ===");
    for input in [r"abc\", r"\q", r"\x4", r"\xzz", "café"] {
        println!("  {input:<8} -> {:?}", unescape_ascii(input));
    }

    println!("\n=== and it reads what Python prints, too ===");
    // Python shows b"it's" in double quotes, so that the ' needs no escape.
    println!("  it's      -> {:?}", unescape_ascii("it's").map(|v| String::from_utf8(v).unwrap()));
}
