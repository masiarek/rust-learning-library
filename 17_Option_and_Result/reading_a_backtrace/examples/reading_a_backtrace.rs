//! Reading a backtrace: the call chain a panic message does not print.
//!
//! A panic names the line that panicked. When that line sits inside a helper
//! called from several places, the name you actually want — which caller got
//! it wrong — is only in the backtrace, and the backtrace is off by default.
//!
//! Everything below is measured rather than asserted: this binary re-runs
//! *itself* as a child process, once with `RUST_BACKTRACE` removed from the
//! environment and once with it set to `1`, and reads what the child wrote.
//! Running the child with an explicit environment is also what makes this an
//! answer key — `RUST_BACKTRACE` set in your own shell cannot change it.
//!
//! Only the frames belonging to this crate are printed. The panic machinery
//! above them and the runtime below them are real, and both are shown on the
//! page, but their names and their number move between platforms and between
//! compiler releases.
//!
//!   rustc --edition 2024 reading_a_backtrace.rs -o /tmp/rab && /tmp/rab

use std::backtrace::Backtrace;
use std::process::{Command, Stdio};

/// A price list with a hole in it: SHIP-STD is here, WIDGET-9 is not.
const PRICES: &[(&str, u32)] = &[("BOLT-1", 40), ("NUT-2", 15), ("SHIP-STD", 599)];

/// The helper. Two callers, one `unwrap`, and every panic points at this line.
fn unit_price(sku: &str) -> u32 {
    PRICES.iter().find(|(s, _)| *s == sku).unwrap().1
}

/// Caller A — asks for a SKU that is in the table.
fn shipping_total() -> u32 {
    unit_price("SHIP-STD")
}

/// Caller B — asks for one that is not. This is the bug, and its name appears
/// nowhere in the panic message.
fn cart_total() -> u32 {
    unit_price("BOLT-1") + unit_price("WIDGET-9")
}

// ── reading a child process ─────────────────────────────────────────────────

/// The prefix every frame from this crate carries in a backtrace.
const OURS: &str = "reading_a_backtrace::";

/// Run this same binary again with `arg`, with `RUST_BACKTRACE` set to
/// `setting` — or removed from the environment when that is `None`.
fn run_child(arg: &str, setting: Option<&str>) -> (String, String) {
    let me = std::env::current_exe().expect("the running binary has a path");
    let mut cmd = Command::new(me);
    cmd.arg(arg).stdout(Stdio::piped()).stderr(Stdio::piped());
    match setting {
        Some(v) => cmd.env("RUST_BACKTRACE", v),
        None => cmd.env_remove("RUST_BACKTRACE"),
    };
    let out = cmd.output().expect("the child runs");
    (
        String::from_utf8_lossy(&out.stdout).trim().to_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// The `file:line:col` the panic named, with the build directory removed —
/// rustc records the path it was handed, which is different on every machine.
fn panic_location(stderr: &str) -> String {
    stderr
        .lines()
        .find_map(|l| l.split_once("panicked at "))
        .map(|(_, rest)| rest.trim_end_matches(':'))
        .map(|loc| loc.rsplit('/').next().unwrap_or(loc).to_owned())
        .unwrap_or_else(|| "(none)".to_owned())
}

/// The panic's own message — the line under the `panicked at` line.
fn panic_message(stderr: &str) -> &str {
    let mut lines = stderr.lines().skip_while(|l| !l.contains("panicked at "));
    lines.nth(1).unwrap_or("(none)").trim()
}

/// This crate's own functions, in the order the backtrace printed them.
///
/// A closure gets a frame of its own — `unit_price::{{closure}}` — and whether
/// it survives to be printed depends on the platform and the optimiser. Keeping
/// only the names with no further `::` in them drops those, so the list below is
/// the same list everywhere.
fn our_frames(dump: &str) -> Vec<&str> {
    dump.lines()
        .filter_map(|l| l.split_once(OURS))
        .map(|(_, name)| name.trim())
        .filter(|name| !name.contains("::"))
        .collect()
}

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        // The child that panics. Its stderr is the subject of steps 1 to 3.
        Some("--panic-for-real") => {
            println!("{}", cart_total());
            return;
        }
        // A child that only reports whether a backtrace would be captured.
        Some("--capture-status") => {
            println!("{:?}", Backtrace::capture().status());
            return;
        }
        _ => {}
    }

    println!("The working caller returns {} cents.", shipping_total());

    banner(1, "the message you get by default");
    let (_, quiet) = run_child("--panic-for-real", None);
    println!("panicked at   {}", panic_location(&quiet));
    println!("message       {}", panic_message(&quiet));
    println!("our frames    {}", our_frames(&quiet).len());
    println!("but it tells you how to ask: {}", quiet.contains("RUST_BACKTRACE=1"));

    banner(2, "the same panic with RUST_BACKTRACE=1");
    let (_, loud) = run_child("--panic-for-real", Some("1"));
    println!("panicked at   {}   <- unchanged", panic_location(&loud));
    println!("message       {}   <- unchanged", panic_message(&loud));
    println!("our frames, in the order the backtrace printed them:");
    for (i, frame) in our_frames(&loud).iter().enumerate() {
        println!("   {i}: {frame}");
    }

    banner(3, "which of the two callers was wrong");
    let frames = our_frames(&loud);
    println!("the panic named      unit_price   (both callers reach it)");
    let culprit = frames.iter().find(|f| f.ends_with("_total"));
    println!("the backtrace named  {}", culprit.copied().unwrap_or("(none)"));
    println!("is shipping_total in the chain?  {}", frames.contains(&"shipping_total"));
    println!("innermost frame first, so the caller is the line BELOW the panic site.");

    banner(4, "capturing one without a panic, and without the variable");
    // `capture()` asks the same environment variable the panic hook asks.
    let (unset, _) = run_child("--capture-status", None);
    let (set, _) = run_child("--capture-status", Some("1"));
    println!("Backtrace::capture()        RUST_BACKTRACE unset -> {unset}");
    println!("Backtrace::capture()        RUST_BACKTRACE=1     -> {set}");
    // `force_capture()` never asks, so it always pays the cost.
    let forced = Backtrace::force_capture();
    println!("Backtrace::force_capture()  either way           -> {:?}", forced.status());
    let dump = forced.to_string();
    println!("and it saw this very function: {}", our_frames(&dump).contains(&"main"));
}
