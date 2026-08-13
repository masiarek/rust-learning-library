//! `map_or` and `map_or_else`: `map` and a fallback, fused — with the arguments
//! in the order you do not expect.
//!
//!     opt.map(f).unwrap_or(d)        ==  opt.map_or(d, f)
//!     opt.map(f).unwrap_or_else(g)   ==  opt.map_or_else(g, f)
//!
//! The default is written FIRST and runs LAST. That is the whole ergonomic story,
//! and it is why clippy pushes you into these two from one side and straight back
//! out of them from the other.
//!
//!   rustc --edition 2024 map_or.rs -o /tmp/mo && /tmp/mo

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
// Step 1 writes `map(f).unwrap_or(d)` on purpose, beside the call that replaces
// it — which is precisely what `clippy::map_unwrap_or` rewrites.
#[allow(clippy::map_unwrap_or)]
fn step1() {
    banner(1, "One call instead of two — and the type may change");

    let scored: Option<u8> = Some(4);
    let blank: Option<u8> = None;

    for (label, o) in [("Some(4)", scored), ("None", blank)] {
        let two_calls = o.map(|v| v * 25).unwrap_or(0);
        let one_call = o.map_or(0, |v| v * 25);
        println!("  {label:<8} map(|v| v*25).unwrap_or(0) = {two_calls:<3} map_or(0, |v| v*25) = {one_call}");
    }

    // Unlike unwrap_or, the answer need not have the type of what was inside.
    let described: String = scored.map_or("no score".to_string(), |v| format!("{v} stars"));
    let described_none: String = blank.map_or("no score".to_string(), |v| format!("{v} stars"));
    println!("  Option<u8> -> String: {described:?} / {described_none:?}");
    println!("      unwrap_or can only give you a T. map_or gives you a U, so it is the");
    println!("      one to reach for when the fallback and the transformed value are the");
    println!("      same KIND of answer — a label, a percentage, a row of a report — but");
    println!("      not the type you started with.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "The default is written first and runs last");

    let quorum: Option<u32> = Some(40);

    let by_match = match quorum {
        Some(q) => q * 2,
        None => 100,
    };
    let by_map_or = quorum.map_or(100, |q| q * 2);
    println!("  match: Some(q) => q*2, None => 100   -> {by_match}");
    println!("  quorum.map_or(100, |q| q * 2)        -> {by_map_or}");
    println!("      Same answer, opposite reading order: the match names the happy case");
    println!("      first, map_or names the fallback first. Nothing enforces the habit —");
    println!("      but swapping the two arguments is a type error, not a silent bug, so");
    println!("      the cost of the surprise is one compile, not one wrong report.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "On a Result, the ERROR closure comes first");

    let good: Result<u8, String> = Ok(4);
    let bad: Result<u8, String> = Err("row 7: '4x' is not a number".to_string());

    for (label, r) in [("Ok(4)", good), ("Err(..)", bad)] {
        let line = r.map_or_else(|e| format!("skipped — {e}"), |v| format!("counted {v}"));
        println!("  {label:<8} map_or_else(|e| .., |v| ..) -> {line}");
    }
    println!("      Read the signature, not the name: map_or_else(default, f), and on a");
    println!("      Result the 'default' is the closure taking E. So the sad path is");
    println!("      written on the left, which is the reverse of every match you have");
    println!("      written, where Ok comes first. This is the one member of the family");
    println!("      worth double-checking at the call site every single time.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn expensive_label() -> String {
    println!("      (building the fallback label...)");
    "no score".to_string()
}

fn step4() {
    banner(4, "map_or is eager, map_or_else is lazy — same rule as before");

    let scored: Option<u8> = Some(4);

    println!("  scored.map_or(expensive_label(), |v| format!(\"{{v}} stars\"))");
    let a = scored.map_or(expensive_label(), |v| format!("{v} stars"));
    println!("  -> {a:?}   (the fallback was built and thrown away)");

    println!("  scored.map_or_else(expensive_label, |v| format!(\"{{v}} stars\"))");
    let b = scored.map_or_else(expensive_label, |v| format!("{v} stars"));
    println!("  -> {b:?}   (nothing printed above: the closure never ran)");
}

// ─────────────────────────────────────────────────────────── Step 5
// The predicate calls below are exactly what `clippy::unnecessary_map_or` exists
// to rewrite. They are the point of the step, so the lint is silenced here and
// nowhere else.
#[allow(clippy::unnecessary_map_or)]
fn step5() {
    banner(5, "Where clippy pushes you in, and where it pushes you back out");

    let scored: Option<u8> = Some(4);
    let blank: Option<u8> = None;

    println!("  IN  — map(f).unwrap_or(d) is clippy::map_unwrap_or (pedantic):");
    println!("        \"use map_or(<a>, <f>) instead\"");

    println!("  OUT — map_or over a PREDICATE is clippy::unnecessary_map_or (on by default):");
    for (label, o) in [("Some(4)", scored), ("None", blank)] {
        println!(
            "        {label:<8} map_or(false, |v| v > 3) = {:<5} is_some_and(|v| v > 3) = {}",
            o.map_or(false, |v| v > 3),
            o.is_some_and(|v| v > 3)
        );
        println!(
            "        {label:<8} map_or(true,  |v| v > 3) = {:<5} is_none_or(|v| v > 3)  = {}",
            o.map_or(true, |v| v > 3),
            o.is_none_or(|v| v > 3)
        );
    }
    println!("      The two nudges are not in conflict: they mark a boundary. When the");
    println!("      fallback is a VALUE, map_or beats writing map and unwrap_or. When it");
    println!("      is `false` or `true`, you were never defaulting at all — you were");
    println!("      asking a yes/no question, and is_some_and / is_none_or say that in");
    println!("      words. (is_none_or is the newer of the two: Rust 1.82.)");
}

// ─────────────────────────────────────────────────────────── Step 6
#[derive(Debug)]
struct Ballot {
    voter: &'static str,
    score: Option<u8>,
}

fn step6() {
    banner(6, "Where a match still wins");

    let ballots = [
        Ballot { voter: "Ada", score: Some(5) },
        Ballot { voter: "Ben", score: None },
        Ballot { voter: "Cara", score: Some(0) },
    ];

    // Fine: one short expression per branch, and the result is one value.
    for b in &ballots {
        println!("  {:<5} {}", b.voter, b.score.map_or("—".to_string(), |s| format!("{s}/5")));
    }

    // Not fine as a one-liner: two branches that each do more than one thing.
    let mut counted = 0u32;
    let mut abstained = 0u32;
    let mut total = 0u32;
    for b in &ballots {
        match b.score {
            Some(s) => {
                counted += 1;
                total += u32::from(s);
            }
            None => abstained += 1,
        }
    }
    println!("  counted {counted}, abstained {abstained}, total {total}");
    println!("      The first loop is exactly what map_or is for: two short expressions");
    println!("      producing one value. The second is not — the branches update three");
    println!("      counters, and a closure that mutates its environment to be useful is");
    println!("      a match wearing a disguise. Neither closure can `?` or `return` out");
    println!("      of the enclosing function either, so anything with an early exit is a");
    println!("      match or an `if let` and not a debate.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    println!();
}
