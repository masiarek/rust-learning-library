//! Kata solution: collapse twenty-six arms into five, then find the three
//! collapses that look right and are not.
//!
//! The point is which of the three the compiler finds for you (one) and which it
//! cannot possibly find (two), and what you have to write yourself to cover the
//! difference.
//!
//!   rustc --edition 2024 one_arm_many_values_kata.rs -o /tmp/oamvk && /tmp/oamvk

/// The original: one arm per hour. This is the answer key for everything below.
fn original(hour: u32) -> &'static str {
    match hour {
        0 => "Classic",
        1 => "Classic",
        2 => "Classic",
        3 => "Classic",
        4 => "Classic",
        5 => "Classic",
        6 => "Classic",
        7 => "Classic",
        8 => "Food",
        9 => "Clothing",
        10 => "Clothing",
        11 => "Clothing",
        12 => "Food",
        13 => "Clothing",
        14 => "Clothing",
        15 => "Clothing",
        16 => "Clothing",
        17 => "Clothing",
        18 => "Food",
        19 => "Season ticket",
        20 => "Season ticket",
        21 => "Season ticket",
        22 => "Season ticket",
        23 => "Season ticket",
        24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

/// Part 1 — the collapse.
fn collapsed(hour: u32) -> &'static str {
    match hour {
        0..=7 => "Classic",
        8 | 12 | 18 => "Food",
        9..=11 | 13..=17 => "Clothing",
        19..=24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

/// Part 2a — the Clothing range swallows the middle Food hour.
/// rustc: `warning: unreachable pattern` with the caret under `12`.
#[allow(unreachable_patterns)]
fn wrong_swallowed(hour: u32) -> &'static str {
    match hour {
        0..=7 => "Classic",
        9..=17 => "Clothing",
        8 | 12 | 18 => "Food",
        19..=24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

/// Part 2b — `..` where `..=` was meant. Every arm is still reachable, so the
/// unreachable check has nothing to say — but a SECOND lint does, because the
/// gap it opens is exactly one wide and the next arm starts on the far side of it.
#[allow(non_contiguous_range_endpoints)]
fn wrong_exclusive(hour: u32) -> &'static str {
    match hour {
        0..7 => "Classic",
        8 | 12 | 18 => "Food",
        9..=11 | 13..=17 => "Clothing",
        19..=24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

/// Part 2c — one hour too generous at the top. This one nothing catches.
fn wrong_too_far(hour: u32) -> &'static str {
    match hour {
        0..=7 => "Classic",
        8 | 12 | 18 => "Food",
        9..=11 | 13..=17 => "Clothing",
        19..=25 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

/// The proof: every input, compared against the answer key.
fn disagreements(f: fn(u32) -> &'static str) -> Vec<(u32, &'static str, &'static str)> {
    (0..=30)
        .filter(|&h| f(h) != original(h))
        .map(|h| (h, original(h), f(h)))
        .collect()
}

fn main() {
    let candidates: [(&str, fn(u32) -> &'static str, &str); 4] = [
        ("collapsed", collapsed, "—"),
        ("wrong_swallowed", wrong_swallowed, "yes: unreachable pattern, caret under `12`"),
        ("wrong_exclusive", wrong_exclusive, "yes: non_contiguous_range_endpoints"),
        ("wrong_too_far", wrong_too_far, "no"),
    ];

    println!("Every hour 0..=30, against the twenty-six-arm original:\n");
    for (name, f, warned) in candidates {
        let bad = disagreements(f);
        println!("  {name:<16} disagreements: {}", bad.len());
        for (hour, want, got) in &bad {
            println!("      hour {hour:>2}: original {want:?}, this version {got:?}");
        }
        println!("      compiler warned? {warned}");
        println!();
    }

    println!("What the three mistakes have in common is that each is one character:");
    println!("  9..=11 | 13..=17  ->  9..=17     a range that ate a value from the arm below");
    println!("  0..=7             ->  0..7       inclusive to exclusive");
    println!("  19..=24           ->  19..=25    one hour past the end of the day");
    println!();
    println!("What they do not have in common is whether anyone tells you — and the");
    println!("count is two out of three, from two different lints:");
    println!();
    println!("  warning: unreachable pattern");
    println!("      9..=17 => ...            matches all the relevant values");
    println!("      8 | 12 | 18 => ...           ^^ no value can reach this");
    println!();
    println!("  warning: multiple ranges are one apart");
    println!("      0..7 => ...              this range doesn't match `7_u32` because");
    println!("                               `..` is an exclusive range");
    println!("      8 | 12 | 18 => ...       this could appear to continue range `0..7`,");
    println!("                               but `7_u32` isn't matched by either of them");
    println!("      help: use an inclusive range instead: `0_u32..=7_u32`");
    println!();
    println!("The second one is worth knowing about, because it is not an exhaustiveness");
    println!("check at all — every hour is still handled, hour 7 merely falls through to");
    println!("the catch-all. `non_contiguous_range_endpoints` is a lint about the SHAPE of");
    println!("your arms: two ranges left exactly one value apart is a typo often enough");
    println!("that rustc says so on sight, and it names the missing value.");
    println!();
    println!("Which leaves the third, and nothing catches the third. `19..=25` makes hour");
    println!("25 a valid hour; the catch-all below it still has 26, 27, 28 to serve, so no");
    println!("arm is dead and no two ranges are one apart. There is nothing structurally");
    println!("wrong for a compiler to notice. Exhaustiveness says every value is handled;");
    println!("it never claimed the handler was the right one.");
    println!();
    println!("So the loop above is not ceremony. Collapsing a match is a refactor, and the");
    println!("only claim a refactor makes is that the answers did not change — which is a");
    println!("claim you can check on every input, for a function this small, in four lines.");
}
