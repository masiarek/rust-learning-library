//! Randomness: std has none, so here is what a generator actually is —
//! a seed, a step function, and a bias you have to know about.
//!
//!   rustc --edition 2024 randomness.rs -o /tmp/rnd && /tmp/rnd

/// xorshift64 in six lines. Not cryptographic, not unbiased, but it is a real
/// pseudo-random generator and every part of it is on this page — which is the
/// point: a seed plus a step function IS the whole idea.
struct Xorshift64(u64);

impl Xorshift64 {
    fn new(seed: u64) -> Self {
        // A zero state is a fixed point of xorshift: it would emit 0 forever.
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

    /// The shape `rand`'s `random_range(low..=high)` has — with the naive
    /// modulo that a real library takes care to avoid. See section 4.
    fn range(&mut self, low: u64, high: u64) -> u64 {
        low + self.next_u64() % (high - low + 1)
    }
}

fn main() {
    println!("1. A seed determines the whole sequence");
    let mut a = Xorshift64::new(2026);
    let mut b = Xorshift64::new(2026);
    let seq_a: Vec<u64> = (0..4).map(|_| a.next_u64() % 1000).collect();
    let seq_b: Vec<u64> = (0..4).map(|_| b.next_u64() % 1000).collect();
    println!("   seed 2026, generator A: {seq_a:?}");
    println!("   seed 2026, generator B: {seq_b:?}");
    println!("   identical: {}", seq_a == seq_b);
    println!("   'Random' here means unpredictable-if-you-do-not-know-the-seed,");
    println!("   not unrepeatable. Same seed, same numbers, forever.");

    println!("\n2. A different seed is a different sequence");
    let mut c = Xorshift64::new(2027);
    let seq_c: Vec<u64> = (0..4).map(|_| c.next_u64() % 1000).collect();
    println!("   seed 2027:              {seq_c:?}");
    println!("   differs from seed 2026: {}", seq_c != seq_a);

    println!("\n3. The guessing game's secret number");
    let mut rng = Xorshift64::new(431);
    let secret = rng.range(1, 100);
    println!("   secret = rng.range(1, 100) = {secret}");
    println!("   in 1..=100: {}", (1..=100).contains(&secret));
    println!("   With `rand` this line is `rand::random_range(1..=100)` and the");
    println!("   seed comes from the operating system, so it is different each run.");

    println!("\n4. Why `% n` is not good enough, counted exactly");
    println!("   Take 4 random bits (16 equally likely patterns) and map them to");
    println!("   1..=10 with `1 + bits % 10`. Count how many patterns reach each:");
    let mut counts = [0u32; 11];
    for bits in 0u32..16 {
        counts[(1 + bits % 10) as usize] += 1;
    }
    let mut line = String::new();
    for value in 1..=10 {
        line.push_str(&format!(" {value}:{}", counts[value]));
    }
    println!("  {line}");
    println!("   Six of the ten outcomes are twice as likely as the other four.");
    println!("   That is modulo bias, and it is exactly why you use a library:");
    println!("   `rand` rejects and redraws the values that would skew the result");
    println!("   instead of folding them back in.");
    println!("   Total patterns: {} (and 16 is not a multiple of 10 — that is the whole cause)",
        counts.iter().sum::<u32>());

    println!("\n5. What std actually gives you");
    println!("   No RNG at all. std has `RandomState` (for HashMap) and `DefaultHasher`,");
    println!("   which are seeded from the OS but are hashing tools, not a generator API.");
    println!("   The answer in real code is the `rand` crate. The answer in THIS library's");
    println!("   examples is a seeded generator like the one above, because every example");
    println!("   here is compiled with bare `rustc` and diffed against a recorded answer");
    println!("   key — a value that changed per run would fail the check every time.");
}
