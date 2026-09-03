//! `unwrap_or`: the fallback you already have.
//!
//! `unwrap_or(d)` answers "and if there is nothing?" with a value you hand it up
//! front. That is its whole appeal and the source of all three of its costs: the
//! default is an ordinary argument, so it is built whether or not it is needed;
//! it takes `self`, so it eats the `Option`; and once applied, nothing downstream
//! can tell a supplied default from a real value.
//!
//!   rustc --edition 2024 unwrap_or.rs -o /tmp/uo && /tmp/uo

use std::sync::atomic::{AtomicUsize, Ordering};

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() {
    banner(1, "What it does — and what it throws away");

    let scored: Option<u8> = Some(5);
    let blank: Option<u8> = None;

    println!("  Some(5).unwrap_or(0)        -> {}", scored.unwrap_or(0));
    println!("  None.unwrap_or(0)             -> {}", blank.unwrap_or(0));

    let good: Result<u8, String> = Ok(5);
    let bad: Result<u8, String> = Err("row 7: '4x' is not a score".to_string());

    println!("  Ok(5).unwrap_or(0)            -> {}", good.unwrap_or(0));
    println!("  Err(e).unwrap_or(0)           -> {}", bad.clone().unwrap_or(0));

    let mut reason = String::new();
    let salvaged = bad.unwrap_or_else(|e| {
        reason = e;
        0
    });
    println!("  Err(e).unwrap_or_else(|e| ..) -> {salvaged}");
    println!("      Both produced 0, but only the closure was told why: {reason:?}.");
    println!("      On a Result, unwrap_or DISCARDS the error — the one thing an");
    println!("      Option had none of to lose. If the reason is worth reporting,");
    println!("      unwrap_or is the wrong end of the family.");
}

// ─────────────────────────────────────────────────────────── Step 2
static BUILDS: AtomicUsize = AtomicUsize::new(0);

/// Stands in for any default that costs something to produce: an allocation, a
/// file read, a query. Here it just counts how often it ran.
fn default_roster() -> Vec<String> {
    BUILDS.fetch_add(1, Ordering::Relaxed);
    vec!["Ada".to_string(), "Ben".to_string(), "Cara".to_string()]
}

/// Four of these five races configured their own roster; one did not.
fn configured_roster(race: &str) -> Option<Vec<String>> {
    match race {
        "library" => None,
        _ => Some(vec![race.to_string(), "Ben".to_string()]),
    }
}

const RACES: [&str; 5] = ["mayor", "council", "library", "sheriff", "school"];

fn step2() {
    banner(2, "The default is computed whether or not it is used");

    BUILDS.store(0, Ordering::Relaxed);
    for race in RACES {
        let _ = configured_roster(race).unwrap_or(default_roster());
    }
    let eager = BUILDS.load(Ordering::Relaxed);

    BUILDS.store(0, Ordering::Relaxed);
    for race in RACES {
        let _ = configured_roster(race).unwrap_or_else(default_roster);
    }
    let lazy = BUILDS.load(Ordering::Relaxed);

    println!("  5 lookups, exactly 1 of them missing:");
    println!("    unwrap_or(default_roster())    -> default_roster() ran {eager} times");
    println!("    unwrap_or_else(default_roster) -> default_roster() ran {lazy} time");
    println!("      `unwrap_or(f())` is a method call with an argument, so Rust");
    println!("      evaluates f() first, every time, and then usually throws the");
    println!("      result away. Nothing is optimized away either: default_roster");
    println!("      allocates, and an allocation is a side effect the compiler must");
    println!("      keep. The lazy form never even calls it on the happy path.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "It consumes the Option — reach for a borrowing form instead");

    let name: Option<String> = Some("Ada".to_string());

    // `unwrap_or` takes `self`, and String is not Copy, so this would move
    // `name` and make the next line a compile error. as_deref borrows instead.
    let missing: Option<String> = None;
    println!("  name.as_deref().unwrap_or(\"(unnamed)\")     -> {}", name.as_deref().unwrap_or("(unnamed)"));
    println!("  missing.as_deref().unwrap_or(\"(unnamed)\")  -> {}", missing.as_deref().unwrap_or("(unnamed)"));
    println!("  name is still usable afterwards            -> {name:?}");

    let scores = [5u8, 3, 1];
    let second: Option<&u8> = scores.get(1);
    let tenth: Option<&u8> = scores.get(9);
    println!("  scores.get(1).copied().unwrap_or(0)        -> {}", second.copied().unwrap_or(0));
    println!("  scores.get(9).copied().unwrap_or(0)        -> {}", tenth.copied().unwrap_or(0));

    println!("  name.as_ref().map_or(0, |s| s.len())       -> {}", name.as_ref().map_or(0, |s| s.len()));
    println!("      Three ways to keep the Option alive: as_deref (Option<String>");
    println!("      -> Option<&str>), copied/cloned (Option<&T> -> Option<T>), and");
    println!("      as_ref for everything else. The type error you get without them");
    println!("      is a MOVE error, not a borrow error, which is why it reads as if");
    println!("      you did something exotic when you only asked for a default.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "Which member of the family");

    let missing: Option<i32> = None;
    let bad: Result<i32, String> = Err("no limit recorded".to_string());

    println!("  None.unwrap_or(0)                 -> {}   value already in hand", missing.unwrap_or(0));
    println!("  None.unwrap_or_else(|| 6 * 7)     -> {}  costs something to build", missing.unwrap_or_else(|| 6 * 7));
    println!("  None.unwrap_or_default()          -> {}   T::default(), no argument", missing.unwrap_or_default());
    println!("  None.map_or(-1, |v| v * 10)       -> {}  transform, or a default", missing.map_or(-1, |v| v * 10));
    println!("  Some(4).map_or(-1, |v| v * 10)    -> {}  same call, value present", Some(4).map_or(-1, |v| v * 10));
    println!(
        "  Err(e).unwrap_or_else(|e| e.len())-> {}  the fallback FROM the error",
        bad.unwrap_or_else(|e| e.len() as i32)
    );
    println!("      The rule is one question: does the default already exist? A");
    println!("      literal, a constant, a copy of something on the stack — pass it,");
    println!("      unwrap_or is clearer. Anything you have to BUILD — allocate,");
    println!("      read, compute — goes in a closure, because the cost is real and");
    println!("      the happy path is the common one.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "A default erases the difference it stood for");

    // Same three questions. One form answered the third 0; one left it blank.
    let scored_zero: [Option<u8>; 3] = [Some(5), Some(3), Some(0)];
    let left_blank: [Option<u8>; 3] = [Some(5), Some(3), None];

    for (label, row) in [("scored 0", scored_zero), ("left blank", left_blank)] {
        let total: u32 = row.iter().map(|s| s.unwrap_or(0) as u32).sum();
        let marked = row.iter().flatten().count();
        println!("  {label:<10} {row:?}");
        println!("             unwrap_or(0) total = {total}, but {marked} of 3 marked");
    }
    println!("      Both total 8, which is correct: a blank sums as zero. What");
    println!("      is not recoverable is the second number — after unwrap_or(0) the");
    println!("      two rows are the same array of u8 and no later code can tell");
    println!("      them apart. So apply the default at the BOUNDARY where the");
    println!("      distinction stops mattering (the sum), never when loading the");
    println!("      data, or you have thrown away a fact you still needed.");
}

// ─────────────────────────────────────────────────────────── Step 6
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

/// The example from the std docs, verbatim in shape: `unwrap_or` inside an array
/// repeat expression. It compiles — but not for the reason the signature suggests.
fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

/// `unwrap` IS const-stable (since 1.83), so this one really is computed at
/// compile time. The line below it, with `unwrap_or`, does not compile today.
const START: u8 = ZERO.unwrap();

fn step6() {
    banner(6, "The `const fn` in the documentation is not the whole story");

    println!("  compute_digest(\"Hello\") -> {:?}", compute_digest("Hello"));
    println!("  const START: u8 = ZERO.unwrap();  -> {START}");
    println!("      The docs show `pub const fn unwrap_or`, and the array-repeat");
    println!("      example above looks like proof that you can use it in a const");
    println!("      context. Two corrections. First, `[expr; N]` needs a constant");
    println!("      only when the element is not Copy; u8 is Copy, so the repeat is");
    println!("      legal with an ordinary runtime expression and the const never");
    println!("      mattered. Second, writing `const D: u8 = ZERO.unwrap_or(0);`");
    println!("      today is rejected: \"`Option::<T>::unwrap_or` is not yet stable");
    println!("      as a const fn\". The signature is const-generic-over-Destruct on");
    println!("      nightly. `unwrap` and `expect` ARE const-stable — and");
    println!("      unwrap_or_else is not const-stable either (its const form needs");
    println!("      the closure itself to be const-callable). So in a const, those");
    println!("      two are the whole family, and the eager/lazy advice does not");
    println!("      apply at all.");
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
