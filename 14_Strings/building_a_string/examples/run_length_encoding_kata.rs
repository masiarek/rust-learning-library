//! Kata solution: run-length encoding both ways — and the three inputs that
//! turn the round trip into a lie.
//!
//!   rustc --edition 2024 run_length_encoding_kata.rs -o /tmp/rlek && /tmp/rlek

/// "AAABBBCCDAA" -> "3A3B2C1D2A". Counts characters, not bytes, so a multibyte
/// run survives; `push_str` and `push` grow one buffer instead of allocating
/// a String per run.
fn encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        let mut run = 1usize;
        while chars.peek() == Some(&c) {
            chars.next();
            run += 1;
        }
        out.push_str(&run.to_string());
        out.push(c);
    }
    out
}

/// "3A3B2C1D2A" -> "AAABBBCCDAA". The count may be several digits, so the
/// digits are accumulated until a non-digit arrives — that character is the
/// one being repeated.
fn decode(s: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut count = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            count.push(c);
        } else if count.is_empty() {
            return Err(format!("character {c:?} has no count in front of it"));
        } else {
            let n: usize = count.parse().map_err(|e| format!("bad count {count:?}: {e}"))?;
            for _ in 0..n {
                out.push(c);
            }
            count.clear();
        }
    }
    if count.is_empty() {
        Ok(out)
    } else {
        Err(format!("input ended with the count {count:?} and no character"))
    }
}

fn round_trip(s: &str) {
    let encoded = encode(s);
    let decoded = decode(&encoded);
    let ok = decoded.as_deref() == Ok(s);
    println!("   {:<24} -> {:<24} -> {:<24} {}",
        format!("{s:?}"),
        format!("{encoded:?}"),
        match &decoded {
            Ok(d) => format!("{d:?}"),
            Err(e) => format!("Err({e})"),
        },
        if ok { "round trip ok" } else { "MISMATCH" });
}

fn main() {
    println!("1. Encoding");
    for s in ["AAABBBCCDAA", "AAAAAAAAAAAA", "ABCDEF", "", "🦀🦀🦀ss"] {
        println!("   {:<16} -> {:?}", format!("{s:?}"), encode(s));
    }
    println!("   Twelve As encode as \"12A\", not \"9A3A\" — which is the whole reason");
    println!("   the decoder cannot just read one digit.");

    println!("\n2. Decoding");
    for s in ["3A3B2C1D2A", "12A", "1A1B1C", "A3B", "3A2"] {
        match decode(s) {
            Ok(d) => println!("   {:<14} -> {d:?}", format!("{s:?}")),
            Err(e) => println!("   {:<14} -> Err: {e}", format!("{s:?}")),
        }
    }
    println!("   Malformed input is a Result, not a panic: a decoder is the half of");
    println!("   this pair that meets data it did not produce.");

    println!("\n3. The round trip");
    for s in ["AAABBBCCDAA", "ABCDEF", "🦀🦀🦀ss", "Mississippi"] {
        round_trip(s);
    }

    println!("\n4. Where the round trip breaks");
    let digits = "AA3BB";
    println!("   Input containing digits: {digits:?}");
    let encoded = encode(digits);
    let back = decode(&encoded).unwrap_or_default();
    println!("     encode -> {encoded:?}");
    println!("     decode -> {} chars, starting {:?}", back.chars().count(), &back[..4]);
    println!("     round trip holds: {}", back == digits);
    println!("   The lone '3' encoded as \"13\" — one 3 — and its digit then ran into");
    println!("   the count in front of the Bs, so the decoder read \"132B\" as 132 Bs.");
    println!("   This encoding has no escape, so its real precondition is \"the alphabet");
    println!("   contains no digits\" — say that in a comment or in the signature,");
    println!("   rather than discovering it in production.");

    println!("\n5. And it is not always compression");
    for s in ["ABCDEF", "AAAAAA"] {
        let e = encode(s);
        println!("   {:<10} {} chars -> {} chars   {}",
            format!("{s:?}"), s.chars().count(), e.chars().count(),
            if e.chars().count() > s.chars().count() { "BIGGER" } else { "smaller" });
    }
    println!("   Run-length encoding pays only when runs are long. On text with no");
    println!("   repeats it doubles the size, which is why real formats keep a literal");
    println!("   mode and switch between the two.");
}
