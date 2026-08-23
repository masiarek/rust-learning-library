//! Kata solution: the fingerprint that collided.
//!
//! You want a short fingerprint for a ballot file — something two election
//! observers can read aloud to each other and compare. Hex is the obvious
//! spelling. The obvious implementation is also wrong, and it is wrong in the
//! one way that matters: two different files can print the same fingerprint.
//!
//!   rustc --edition 2024 why_hexadecimal_kata.rs -o /tmp/whxk && /tmp/whxk

use std::collections::{BTreeMap, BTreeSet};

/// The version almost everyone writes first.
fn naive(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:x}")).collect()
}

/// The same thing, with the one property hex was chosen for.
fn encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[derive(Debug, PartialEq)]
enum HexError {
    OddLength(usize),
    BadDigit(String),
}

/// Read a fingerprint back. Accepts an optional `0x`, because a human will type one.
fn decode(text: &str) -> Result<Vec<u8>, HexError> {
    let t = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")).unwrap_or(text);
    if !t.len().is_multiple_of(2) {
        return Err(HexError::OddLength(t.len()));
    }
    (0..t.len())
        .step_by(2)
        .map(|i| {
            let pair = &t[i..i + 2];
            u8::from_str_radix(pair, 16).map_err(|_| HexError::BadDigit(pair.to_string()))
        })
        .collect()
}

fn main() {
    println!("=== two ballot files, one fingerprint ===");
    let file_a: [u8; 3] = [0x0A, 0xB0, 0x42];
    let file_b: [u8; 3] = [0xAB, 0x00, 0x42];
    let show = |bs: &[u8]| bs.iter().map(|b| format!("0x{b:02X}")).collect::<Vec<_>>().join(", ");
    println!("  file A  [{}]  ->  naive {:?}", show(&file_a), naive(&file_a));
    println!("  file B  [{}]  ->  naive {:?}", show(&file_b), naive(&file_b));
    println!("  different files:      {}", file_a != file_b);
    println!("  same fingerprint:     {}   <- the observers agree, and they are wrong",
             naive(&file_a) == naive(&file_b));

    println!("\n=== the fix is two characters of format string ===");
    println!("  file A  ->  {:?}", encode(&file_a));
    println!("  file B  ->  {:?}", encode(&file_b));
    println!("  now distinct:         {}", encode(&file_a) != encode(&file_b));

    println!("\n=== how bad was it? every two-byte file, counted ===");
    let all: Vec<[u8; 2]> = (0..=255u8).flat_map(|x| (0..=255u8).map(move |y| [x, y])).collect();
    let naive_distinct: BTreeSet<String> = all.iter().map(|p| naive(p)).collect();
    let fixed_distinct: BTreeSet<String> = all.iter().map(|p| encode(p)).collect();
    println!("  inputs                     : {}", all.len());
    println!("  distinct naive fingerprints: {}   <- {} files lost their identity",
             naive_distinct.len(), all.len() - naive_distinct.len());
    println!("  distinct fixed fingerprints: {}   <- one each, which is the job",
             fixed_distinct.len());

    let mut buckets: BTreeMap<String, Vec<[u8; 2]>> = BTreeMap::new();
    for p in &all {
        buckets.entry(naive(p)).or_default().push(*p);
    }
    let worst = buckets.values().map(|v| v.len()).max().unwrap_or(0);
    println!("  worst single collision     : {worst} different files share one string");
    println!("  the first few collisions:");
    for (text, group) in buckets.iter().filter(|(_, g)| g.len() > 1).take(3) {
        let shown: Vec<String> = group.iter().map(|p| format!("{p:?}")).collect();
        println!("    {text:>5} <- {}", shown.join("  "));
    }

    println!("\n=== reading a fingerprint back ===");
    for input in ["0ab042", "0x0ab042", "0AB042", "0ab04", "0ab0zz"] {
        println!("  {input:<10} -> {:?}", decode(input));
    }

    println!("\n=== the round trip, proved rather than asserted ===");
    let single_ok = (0..=255u8).all(|b| decode(&encode(&[b])) == Ok(vec![b]));
    let pairs_ok = all.iter().all(|p| decode(&encode(p)) == Ok(p.to_vec()));
    println!("  all 256 single bytes round-trip   : {single_ok}");
    println!("  all {} two-byte files round-trip: {pairs_ok}", all.len());
    let naive_single = (0..=255u8).filter(|&b| decode(&naive(&[b])) == Ok(vec![b])).count();
    println!("  ...and with the naive encoder     : {naive_single} of 256");
    let failures: Vec<u8> = (0..=255u8).filter(|&b| decode(&naive(&[b])) != Ok(vec![b])).collect();
    println!("  the {} that fail are 0x{:02X}..=0x{:02X} -- exactly the bytes one digit could spell,",
             failures.len(), failures[0], failures[failures.len() - 1]);
    println!("  which is why the bug hides: it needs a small byte to show itself at all");
}
