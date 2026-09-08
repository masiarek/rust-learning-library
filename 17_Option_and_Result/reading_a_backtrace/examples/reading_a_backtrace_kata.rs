//! Kata solution: a backtrace describes ONE stack, and a thread is a wall.
//!
//! `RUST_BACKTRACE=1` answers "who called this?" perfectly — right up to the
//! point where the work crossed a thread boundary. Past that the trail stops,
//! because the code that handed the job over is not on the stack that failed.
//! The fix is to capture the caller's stack *at the handover* and carry it.
//!
//!   rustc --edition 2024 reading_a_backtrace_kata.rs -o /tmp/rabk && /tmp/rabk

use std::backtrace::Backtrace;
use std::panic;
use std::process::{Command, Stdio};
use std::thread;

const PRICES: &[(&str, u32)] = &[("BOLT-1", 40), ("NUT-2", 15), ("SHIP-STD", 599)];
const OURS: &str = "reading_a_backtrace_kata::";

fn unit_price(sku: &str) -> u32 {
    PRICES.iter().find(|(s, _)| *s == sku).unwrap().1
}

fn cart_total() -> u32 {
    unit_price("BOLT-1") + unit_price("WIDGET-9")
}

/// The submitter. It hands the work to another thread and waits — so it is on
/// the *calling* stack, never on the one that fails.
fn checkout_request() -> Result<u32, Submitted> {
    submit(cart_total)
}

// ── part 1: watch the trail stop at the thread boundary ─────────────────────

fn run_child(arg: &str) -> String {
    let me = std::env::current_exe().expect("the running binary has a path");
    let out = Command::new(me)
        .arg(arg)
        .env("RUST_BACKTRACE", "1")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect("the child runs");
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// Our own functions in a dump — closures dropped, so the list is the same on
/// every platform.
fn our_frames(dump: &str) -> Vec<&str> {
    dump.lines()
        .filter_map(|l| l.split_once(OURS))
        .map(|(_, n)| n.trim())
        .filter(|n| !n.contains("::"))
        .collect()
}

// ── part 2: carry the submitter's stack across by hand ──────────────────────

/// A failure plus the stack of whoever *asked* for the work, captured at the
/// moment of asking rather than at the moment of failing.
struct Submitted {
    what: String,
    submitted_from: Backtrace,
}

fn submit<T: Send + 'static>(job: fn() -> T) -> Result<T, Submitted> {
    // Captured HERE, on the calling thread, while its stack is still standing.
    let submitted_from = Backtrace::force_capture();
    thread::spawn(job).join().map_err(|payload| Submitted {
        // A panic payload is `&'static str` for a fixed message and `String`
        // for a formatted one. `unwrap` produces the first; ask for both.
        what: payload
            .downcast_ref::<&str>()
            .map(|s| (*s).to_owned())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "(unknown payload type)".to_owned()),
        submitted_from,
    })
}

fn main() {
    if std::env::args().any(|a| a == "--in-a-thread") {
        // The panic hook still prints this thread's backtrace to stderr.
        let _ = checkout_request();
        return;
    }

    println!("── Part 1: the same panic, on a thread of its own");
    let dump = run_child("--in-a-thread");
    let frames = our_frames(&dump);
    println!("our frames in the worker's backtrace: {frames:?}");
    println!("  unit_price       (where it failed)  present: {}", frames.contains(&"unit_price"));
    println!("  cart_total       (its caller)       present: {}", frames.contains(&"cart_total"));
    println!("  checkout_request (the submitter)    present: {}", frames.contains(&"checkout_request"));
    println!("  main                                present: {}", frames.contains(&"main"));
    println!("The last two ran on the other stack, so the trail simply ends.");

    println!("\n── Part 2: capture the submitter's stack at the handover");
    // The worker still panics, and the default hook would print its message and
    // its backtrace to stderr. Silence it: this half of the exercise is about
    // the stack we captured ourselves, on the near side of the boundary.
    panic::set_hook(Box::new(|_| {}));
    match checkout_request() {
        Ok(total) => println!("no panic, total = {total}"),
        Err(e) => {
            println!("the worker failed with: {}", e.what);
            let here = e.submitted_from.to_string();
            let names = our_frames(&here);
            println!("the captured stack names:");
            println!("  checkout_request  present: {}", names.contains(&"checkout_request"));
            println!("  main              present: {}", names.contains(&"main"));
            println!("Those are the two the worker's own backtrace could not see.");
        }
    }

    println!("\n── The rule");
    println!("Every boundary that moves work to another stack — a thread, a");
    println!("channel, an executor — ends the backtrace. Capture on the near");
    println!("side and carry it, or the far side reports a chain with no origin.");
}
