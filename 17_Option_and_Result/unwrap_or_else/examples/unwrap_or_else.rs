//! `unwrap_or_else`: the fallback that is built only if it is needed — and, on a
//! `Result`, the only member of the family that is told why.
//!
//! Its two signatures are the whole lesson:
//!
//!     Option::unwrap_or_else(self, f: F)  where F: FnOnce()  -> T
//!     Result::unwrap_or_else(self, f: F)  where F: FnOnce(E) -> T
//!
//! A closure instead of a value buys three things: the default is not computed
//! on the happy path, it may consume what it captures, and on a `Result` it is
//! handed the error — so a fallback need not mean a forgotten failure.
//!
//!   rustc --edition 2024 unwrap_or_else.rs -o /tmp/uoe && /tmp/uoe

use std::sync::atomic::{AtomicUsize, Ordering};

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() {
    banner(1, "Two shapes: no argument on Option, the error on Result");

    let limit: Option<u16> = None;
    let parsed: Result<u16, String> = Err("'4x' is not a number".to_string());

    println!("  None.unwrap_or_else(|| 50)                -> {}", limit.unwrap_or_else(|| 50));
    println!(
        "  Err(e).unwrap_or_else(|e| e.len() as u16) -> {}",
        parsed.unwrap_or_else(|e| e.len() as u16)
    );
    println!("      The Option closure takes nothing, because None carries nothing.");
    println!("      The Result closure takes the error BY VALUE — it is the only");
    println!("      fallback in the family that ever sees it. Swap the two by habit");
    println!("      and the compiler is specific: E0593, closure is expected to take");
    println!("      1 argument, but it takes 0.");
}

// ─────────────────────────────────────────────────────────── Step 2
/// A row is a score only if it parses AND lands in the 0–5 range. Two different
/// failures, two different sentences.
fn score(raw: &str) -> Result<u8, String> {
    let n: u8 = raw.trim().parse().map_err(|e| format!("{e}"))?;
    if n > 5 {
        return Err(format!("{n} is outside the 0-5 range"));
    }
    Ok(n)
}

const RAW: [&str; 6] = ["5", "3", "4x", "2", "", "12"];

fn step2() {
    banner(2, "The payoff: fall back AND keep the reason");

    let mut log: Vec<String> = Vec::new();
    let mut total = 0u32;

    for (i, raw) in RAW.iter().enumerate() {
        let s = score(raw).unwrap_or_else(|why| {
            log.push(format!("row {}: {raw:?} — {why}", i + 1));
            0
        });
        total += s as u32;
    }

    println!("  raw rows: {RAW:?}");
    println!("  total    : {total}   ({} of {} rows counted as 0)", log.len(), RAW.len());
    for line in &log {
        println!("    {line}");
    }
    println!("      This is the shape to reach for when a bad row must not stop the");
    println!("      count but must not vanish either. `unwrap_or(0)` would produce");
    println!("      the same total and no second list — same arithmetic, no audit.");
}

// ─────────────────────────────────────────────────────────── Step 3
static BUILDS: AtomicUsize = AtomicUsize::new(0);

fn default_roster() -> Vec<String> {
    BUILDS.fetch_add(1, Ordering::Relaxed);
    vec!["Ada".to_string(), "Ben".to_string()]
}

fn configured_roster(race: &str) -> Option<Vec<String>> {
    match race {
        "library" | "school" => None,
        _ => Some(vec![race.to_string()]),
    }
}

const RACES: [&str; 6] = ["mayor", "council", "library", "sheriff", "school", "judge"];

fn step3() {
    banner(3, "Only built when it is needed — and it need not be a closure");

    BUILDS.store(0, Ordering::Relaxed);
    for race in RACES {
        let _ = configured_roster(race).unwrap_or_else(default_roster);
    }
    println!(
        "  6 races, 2 without a configured roster -> default_roster() ran {} times",
        BUILDS.load(Ordering::Relaxed)
    );

    let empty: Option<String> = None;
    let none_yet: Option<Vec<u8>> = None;
    println!("  None.unwrap_or_else(String::new)    -> {:?}", empty.unwrap_or_else(String::new));
    println!("  None.unwrap_or_else(Vec::new)       -> {:?}", none_yet.unwrap_or_else(Vec::new));
    println!("  None::<Vec<u8>>.unwrap_or_default() -> {:?}", None::<Vec<u8>>.unwrap_or_default());
    println!("      `unwrap_or_else(Vec::new)` passes the function itself — no `||`,");
    println!("      because a function name already IS something callable. And when");
    println!("      the function you would name is exactly T::default, the shorter");
    println!("      spelling of the same call is unwrap_or_default().");
}

// ─────────────────────────────────────────────────────────── Step 4
fn from_flag(_name: &str) -> Option<u16> {
    None
}

fn from_env(name: &str) -> Option<u16> {
    match name {
        "port" => Some(8080),
        _ => None,
    }
}

fn compiled_in_default() -> u16 {
    9000
}

fn step4() {
    banner(4, "The two `_else` methods do opposite things");

    for name in ["port", "metrics_port"] {
        let found = from_flag(name).or_else(|| from_env(name));
        let settled = found.unwrap_or_else(compiled_in_default);
        println!("  {name:<13} flag={:?} then env={:?} -> or_else gives {found:?}, unwrap_or_else gives {settled}",
                 from_flag(name), from_env(name));
    }

    let ok: Result<u16, String> = Ok(443);
    let bad: Result<u16, String> = Err("not a port".to_string());
    println!("  Ok(443).map_or_else(|e| .., |v| ..)  -> {}", ok.map_or_else(|e| format!("failed: {e}"), |v| format!("listening on {v}")));
    println!("  Err(e).map_or_else(|e| .., |v| ..)   -> {}", bad.map_or_else(|e| format!("failed: {e}"), |v| format!("listening on {v}")));
    println!("      or_else STAYS inside the wrapper: Option -> Option, so you can");
    println!("      chain a second and third source. unwrap_or_else LEAVES it: the");
    println!("      result is a plain u16 and the chain is over. Writing one where");
    println!("      you meant the other is E0308 — expected `u16`, found `Option<u16>`.");
    println!("      map_or_else handles both sides at once: one closure per branch.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "The lab coat: a closure that ignores its argument");

    let bad: Result<u8, String> = Err("row 3: '4x' is not a number".to_string());

    println!("  bad.clone().unwrap_or(0)          -> {}", bad.clone().unwrap_or(0));
    println!("  bad.clone().unwrap_or_else(|_| 0) -> {}", bad.clone().unwrap_or_else(|_| 0));
    println!("      Identical, in every way that matters: same value, same discarded");
    println!("      error, same cost. The second one just looks more careful. If the");
    println!("      underscore is there because there is genuinely nothing to do with");
    println!("      the error, write unwrap_or(0) and say so plainly. If there IS —");
    println!("      a log line, a counter, a tag on the value — the closure is the");
    println!("      place for it, and that is Step 2.");

    // Not a default at all: a value that remembers it was defaulted.
    enum Score {
        Cast(u8),
        Defaulted { to: u8, why: String },
    }
    let rows: [Result<u8, String>; 3] = [
        Ok(4),
        Err("row 3: '4x' is not a number".to_string()),
        Ok(5),
    ];
    let salvaged: Vec<Score> = rows
        .into_iter()
        .map(|r| r.map(Score::Cast).unwrap_or_else(|why| Score::Defaulted { to: 0, why }))
        .collect();
    let total: u32 = salvaged
        .iter()
        .map(|s| match s {
            Score::Cast(n) => *n as u32,
            Score::Defaulted { to, .. } => *to as u32,
        })
        .sum();
    println!("  a fallback that remembers it was one:");
    for s in &salvaged {
        match s {
            Score::Cast(n) => println!("    cast      counts {n}"),
            Score::Defaulted { to, why } => println!("    defaulted counts {to}, because {why}"),
        }
    }
    println!("    total {total}, and the sum can still say which rows it trusted.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn step6() {
    banner(6, "A closure is a struct, and FnOnce means it may eat its captures");

    let capture_nothing = || 50u16;
    let columns = 3u16;
    let capture_a_copy = move || columns * 2;
    let fallback = String::from("(unnamed)");
    let capture_and_consume = move || fallback;

    println!("  size_of_val(&|| 50)             = {}", size_of_val(&capture_nothing));
    println!("  size_of_val(&move || columns * 2) = {}", size_of_val(&capture_a_copy));
    println!("  size_of_val(&move || fallback)  = {}  (a String moved in)", size_of_val(&capture_and_consume));

    let missing: Option<String> = None;
    println!("  missing.unwrap_or_else(that closure) -> {:?}", missing.unwrap_or_else(capture_and_consume));

    let present: Option<u16> = Some(7);
    println!("  Some(7).unwrap_or_else(|| 50)   -> {}", present.unwrap_or_else(capture_nothing));
    println!("      A closure capturing nothing is zero bytes and compiles to the");
    println!("      same code as the branch you would have written by hand, so the");
    println!("      laziness is free. FnOnce (rather than Fn) is what lets the last");
    println!("      one hand its captured String straight out as the fallback: it is");
    println!("      called at most once, so it is allowed to consume what it holds.");
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
