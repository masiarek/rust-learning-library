//! Kata solution: seed it, prove it, measure the bias, then remove the bias.
//!
//!   rustc --edition 2024 randomness_kata.rs -o /tmp/rndk && /tmp/rndk

struct Xorshift64(u64);

impl Xorshift64 {
    fn new(seed: u64) -> Self {
        Xorshift64(if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed })
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    /// Four bits at a time, so the whole output space is 16 values and every
    /// claim below can be COUNTED rather than sampled.
    fn nibble(&mut self) -> u32 {
        (self.next_u64() & 0xF) as u32
    }
}

/// Fold a 4-bit draw into 1..=n. Biased whenever 16 % n != 0.
fn naive(bits: u32, n: u32) -> u32 {
    1 + bits % n
}

/// Reject the draws that would skew it, and ask for another. This is what
/// `rand`'s `random_range` does; the only cost is an occasional extra draw.
fn rejecting(bits: u32, n: u32) -> Option<u32> {
    let limit = (16 / n) * n; // the largest multiple of n that fits in 0..16
    if bits < limit { Some(1 + bits % n) } else { None }
}

/// Exact distribution over ALL 16 patterns — no sampling, no seed involved.
fn tally(n: u32, reject: bool) -> Vec<(u32, u32)> {
    let mut counts = vec![0u32; (n + 1) as usize];
    for bits in 0..16u32 {
        let hit = if reject { rejecting(bits, n) } else { Some(naive(bits, n)) };
        if let Some(v) = hit {
            counts[v as usize] += 1;
        }
    }
    (1..=n).map(|v| (v, counts[v as usize])).collect()
}

fn show(label: &str, rows: &[(u32, u32)]) {
    let body: Vec<String> = rows.iter().map(|(v, c)| format!("{v}:{c}")).collect();
    let hits: u32 = rows.iter().map(|(_, c)| c).sum();
    let lo = rows.iter().map(|(_, c)| *c).min().unwrap();
    let hi = rows.iter().map(|(_, c)| *c).max().unwrap();
    println!("   {label:<22} {}   ({hits}/16 patterns used, spread {lo}..{hi})", body.join(" "));
}

fn main() {
    println!("1. Same seed, same sequence — asserted, not eyeballed");
    let mut a = Xorshift64::new(7);
    let mut b = Xorshift64::new(7);
    let sa: Vec<u32> = (0..5).map(|_| a.nibble()).collect();
    let sb: Vec<u32> = (0..5).map(|_| b.nibble()).collect();
    println!("   A: {sa:?}");
    println!("   B: {sb:?}");
    assert_eq!(sa, sb, "a seeded generator must be reproducible");
    println!("   assert_eq!(A, B) passed — reproducibility is a property you can TEST,");
    println!("   which is the reason a library exposes seeding at all.\n");

    println!("2. A range that divides the output space, and one that does not");
    println!("   The generator emits 4 bits: 16 equally likely patterns.");
    show("1..=8   (16 % 8 == 0)", &tally(8, false));
    show("1..=10  (16 % 10 == 6)", &tally(10, false));
    println!("   `1..=8` is already fair: every outcome gets exactly 2 patterns.");
    println!("   `1..=10` is not, and the six low outcomes are twice as likely.\n");

    println!("3. The bias is a property of the ARITHMETIC, not of the generator");
    show("1..=3   (16 % 3 == 1)", &tally(3, false));
    show("1..=6   (16 % 6 == 4)", &tally(6, false));
    println!("   No seed appears in section 2 or 3. Every pattern is counted once,");
    println!("   so a better generator cannot fix this — only better arithmetic can.\n");

    println!("4. Reject and redraw, and the skew goes to zero");
    show("1..=10 rejecting", &tally(10, true));
    show("1..=3  rejecting", &tally(3, true));
    show("1..=6  rejecting", &tally(6, true));
    println!("   Every outcome now has the same number of patterns. The price is the");
    println!("   patterns thrown away — 6 of 16 for `1..=10` — which is why the real");
    println!("   implementation draws again rather than returning `None`.\n");

    println!("5. What that costs in practice");
    for n in [3u32, 6, 8, 10] {
        let kept = (16 / n) * n;
        println!(
            "   1..={n:<3} keeps {kept:>2}/16 patterns  ->  {:.1}% of draws are redrawn",
            (16 - kept) as f64 * 100.0 / 16.0
        );
    }
    println!("   With a real 64-bit generator the discarded slice is vanishingly small,");
    println!("   so the correct version is effectively free. The 4-bit generator here is");
    println!("   small on purpose: it makes the whole distribution countable.");
}
