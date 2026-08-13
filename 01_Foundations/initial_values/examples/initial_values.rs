//! `Option` for a variable that has no value *yet* — and the three times Rust
//! does not need it, because deferred initialization already covers the case.
//!
//!   rustc --edition 2024 initial_values.rs -o /tmp/iv && /tmp/iv

use std::sync::OnceLock;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() {
    banner(1, "The pattern you will see written first");

    let mut initial_value: Option<i32> = None;
    initial_value = Some(42);

    match initial_value {
        Some(value) => println!("  The initial value is: {value}"),
        None => println!("  No initial value"),
    }
    println!("      It works. But notice what it costs: a `mut`, a wrapper, and a `match`");
    println!("      over a case that — by the time we look — cannot happen.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2(flag: bool) {
    banner(2, "What Rust actually offers: declare now, assign later");

    let settled: i32; // no value, no `mut`, no Option
    if flag {
        settled = 42;
    } else {
        settled = 7;
    }
    println!("  settled = {settled}");

    // Usually you would not even need the branch statement:
    let same = if flag { 42 } else { 7 };
    println!("  same    = {same}");

    println!("      Rust does not require a value at DECLARATION — only before USE, and");
    println!("      it proves that at compile time. So `settled` is never Option-shaped,");
    println!("      never mut, and no branch can forget to set it.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "When Option IS right: absence survives to the point of use");

    // A setting the user may simply never have given. Simulated rather than read
    // from the real environment: a recorded example must print the same thing on
    // every machine, so nothing here touches env, the clock, or randomness.
    let settings = [("log_level", "debug")];
    let lookup = |key: &str| settings.iter().find(|(k, _)| *k == key).map(|(_, v)| *v);

    let configured: Option<u16> = lookup("port").and_then(|s| s.parse().ok());
    println!("  lookup(\"log_level\")  -> {:?}", lookup("log_level"));
    println!("  lookup(\"port\")       -> {:?}", lookup("port"));
    println!("  port in use          -> {}", configured.unwrap_or(8080));
    println!("      Here None is not 'not yet assigned', it is 'the user did not say'.");
    println!("      That fact is still true when we read it — so it belongs in the type.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "…and for a running 'best so far', where there is no sensible start");

    let scores = [3u32, 9, 4];
    let mut best: Option<u32> = None;
    for s in scores {
        best = Some(best.map_or(s, |b| b.max(s)));
    }
    println!("  best of {scores:?} -> {best:?}");

    let empty: [u32; 0] = [];
    let mut best_empty: Option<u32> = None;
    for s in empty {
        best_empty = Some(best_empty.map_or(s, |b| b.max(s)));
    }
    println!("  best of []        -> {best_empty:?}");
    println!("      Starting at 0 would be a LIE for an empty list, and wrong for negatives.");
    println!("      (In real code: scores.iter().max() — which returns Option for this reason.)");
    println!("  scores.iter().max() -> {:?}", scores.iter().max());
}

// ─────────────────────────────────────────────────────────── Step 5
static GREETING: OnceLock<String> = OnceLock::new();

fn step5() {
    banner(5, "Initialize once, later, globally: OnceLock");

    println!("  before set: {:?}", GREETING.get());
    GREETING.set("hello".to_string()).expect("first set always succeeds");
    println!("  after set:  {:?}", GREETING.get());
    println!("  second set: {:?}", GREETING.set("other".to_string()).is_err());
    println!("      A `static` cannot be deferred — so this is the case that genuinely");
    println!("      needs an 'empty until later' box. OnceLock is that box, and unlike a");
    println!("      mut Option it guarantees the value is written exactly once.");
}

fn main() {
    step1();
    step2(true);
    step3();
    step4();
    step5();
    println!();
}
