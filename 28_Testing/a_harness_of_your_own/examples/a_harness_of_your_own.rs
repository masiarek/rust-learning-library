//! A test harness is a `main` that runs tests and picks an exit status. This is
//! a small one, with the parts `harness = false` makes you write yourself.
//!
//!   rustc --edition 2024 a_harness_of_your_own.rs -o /tmp/ahooy && /tmp/ahooy
//!
//! The command lines are passed in as slices rather than read from the real
//! arguments, so one run shows all of them. Section 5 starts a copy of this
//! program to show a harness that does not catch panics.

use std::env;
use std::panic;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

static SETUPS: AtomicUsize = AtomicUsize::new(0);

/// A test is a name and a function that panics when it is unhappy.
struct Trial {
    name: &'static str,
    run: fn(),
}

fn price_in_cents(text: &str) -> Option<u32> {
    let (whole, cents) = text.split_once('.')?;
    if cents.len() != 2 {
        return None;
    }
    Some(whole.parse::<u32>().ok()? * 100 + cents.parse::<u32>().ok()?)
}

/// Registered by hand, in this order. Nothing collects `#[test]` functions for you.
const TRIALS: [Trial; 3] = [
    Trial { name: "parses_a_price", run: || assert_eq!(price_in_cents("2.50"), Some(250)) },
    Trial { name: "accepts_one_decimal", run: || assert_eq!(price_in_cents("2.5"), Some(250)) },
    Trial { name: "rejects_a_word", run: || assert_eq!(price_in_cents("two"), None) },
];

/// Runs once for the whole binary: the reason most custom harnesses exist.
struct Suite;

impl Suite {
    fn start() -> Suite {
        SETUPS.fetch_add(1, Ordering::SeqCst);
        println!("   [setup] once, before any test (start a container, seed a database)");
        Suite
    }
}

impl Drop for Suite {
    fn drop(&mut self) {
        println!("   [teardown] once, after the last test");
    }
}

fn message(payload: &(dyn std::any::Any + Send)) -> String {
    let text = payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default();
    text.lines().map(str::trim).collect::<Vec<_>>().join("; ")
}

/// Honours a name filter and `--list`, catches each panic, and returns the
/// exit status libtest would: 0 when everything passed, 101 otherwise.
fn harness(args: &[&str], trials: &[Trial]) -> i32 {
    if args.contains(&"--list") {
        for t in trials {
            println!("   {}: test", t.name);
        }
        return 0;
    }
    let filter = args.iter().find(|a| !a.starts_with("--"));
    let selected: Vec<&Trial> = trials.iter().filter(|t| filter.is_none_or(|f| t.name.contains(f))).collect();

    let _suite = Suite::start();
    println!("   running {} test{}", selected.len(), if selected.len() == 1 { "" } else { "s" });
    let mut failures = Vec::new();
    for t in &selected {
        match panic::catch_unwind(t.run) {
            Ok(()) => println!("   test {} ... ok", t.name),
            Err(payload) => {
                println!("   test {} ... FAILED", t.name);
                failures.push((t.name, message(payload.as_ref())));
            }
        }
    }
    for (name, why) in &failures {
        println!("   ---- {name} ---- {why}");
    }
    let verdict = if failures.is_empty() { "ok" } else { "FAILED" };
    println!(
        "   test result: {verdict}. {} passed; {} failed; {} filtered out",
        selected.len() - failures.len(),
        failures.len(),
        trials.len() - selected.len()
    );
    if failures.is_empty() { 0 } else { 101 }
}

/// The same loop with no catch_unwind: the first panic ends the process.
fn harness_without_catch(trials: &[Trial]) {
    for t in trials {
        print!("test {} ... ", t.name);
        (t.run)();
        println!("ok");
    }
}

fn main() {
    if env::args().nth(1).as_deref() == Some("without-catch") {
        return harness_without_catch(&TRIALS);
    }
    let quiet = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    println!("1. cargo test --test prices");
    let status = harness(&[], &TRIALS);
    println!("   exit status {status}");

    println!();
    println!("2. cargo test --test prices -- parses");
    let status = harness(&["parses"], &TRIALS);
    println!("   exit status {status}");

    println!();
    println!("3. cargo test --test prices -- --list");
    let status = harness(&["--list"], &TRIALS);
    println!("   exit status {status}");

    println!();
    println!("4. Setup ran {} times for 2 runs that executed tests", SETUPS.load(Ordering::SeqCst));
    println!("   Once per run of the binary, not once per test, and never for --list.");

    panic::set_hook(quiet);
    println!();
    println!("5. The same tests, with no catch_unwind around each one");
    let out = Command::new(env::current_exe().expect("own path"))
        .arg("without-catch")
        .stderr(Stdio::null())
        .output()
        .expect("child runs");
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        println!("   {}", line.trim_end());
    }
    println!("   exit status {:?}; rejects_a_word never ran, and no summary line says so", out.status.code());
}
