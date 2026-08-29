//! The same function twice: as written, and flattened into a state machine.
//!
//! Flattening is what an obfuscation pass does to a control-flow graph. The
//! second version has identical behaviour and no readable shape — every branch
//! has become an assignment to `state`, and the dispatcher decides everything.

/// As anyone would write it. The graph is the source: two decisions, three
/// outcomes, one exit.
fn classify(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}

/// The same function, flattened. Each basic block is numbered and parked as a
/// sibling under one loop; the block's last act is to set the next state.
/// A real obfuscator uses large random constants here, not 0..=5.
fn classify_flattened(n: i32) -> &'static str {
    let mut state = 0u32;
    let mut out = "";
    loop {
        match state {
            0 => state = if n < 0 { 1 } else { 2 },
            1 => {
                out = "negative";
                state = 5;
            }
            2 => state = if n == 0 { 3 } else { 4 },
            3 => {
                out = "zero";
                state = 5;
            }
            4 => {
                out = "positive";
                state = 5;
            }
            _ => return out,
        }
    }
}

/// An opaque predicate: always true, for a reason a reader has to work out.
/// `n * (n - 1)` is a product of consecutive integers, so one of them is even.
fn always_true(n: u32) -> bool {
    n.wrapping_mul(n.wrapping_sub(1)) % 2 == 0
}

/// Instruction substitution, the `sub` pass: replace an operation with a more
/// complicated one that computes the same thing. `a + b` becomes `a - (-b)`.
fn add_substituted(a: i32, b: i32) -> i32 {
    a.wrapping_sub(b.wrapping_neg())
}

/// A second substitution for the same operation: `a + b` = `(a ^ b) + 2*(a & b)`
/// — the bits that do not carry, plus twice the bits that do.
fn add_substituted_twice(a: i32, b: i32) -> i32 {
    (a ^ b).wrapping_add((a & b).wrapping_mul(2))
}

fn main() {
    for n in [-7, -1, 0, 1, 42] {
        let plain = classify(n);
        let flat = classify_flattened(n);
        println!("classify({n:>3}) = {plain:<8}  flattened = {flat:<8}  agree = {}", plain == flat);
    }

    let holds = (0..1000).all(always_true);
    println!("opaque predicate held for all 1000 inputs: {holds}");

    // Both substitutions agree with `+` on every pair in a wide sample.
    let pairs = || (-500..500).flat_map(|a| (-500..500).map(move |b| (a, b)));
    let subs_agree = pairs().all(|(a, b)| add_substituted(a, b) == a + b);
    let xor_agree = pairs().all(|(a, b)| add_substituted_twice(a, b) == a + b);
    println!("a - (-b) matched a + b on 1,000,000 pairs: {subs_agree}");
    println!("(a^b) + 2*(a&b) matched a + b on 1,000,000 pairs: {xor_agree}");
}
