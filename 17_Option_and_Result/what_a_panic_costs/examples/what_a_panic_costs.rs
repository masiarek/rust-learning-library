//! What a panic actually costs — the half of `unwrap` that is not about `Option`.
//!
//! Unwinding is tidy about *memory*: every destructor between the panic and the
//! catch runs, in order. It is not tidy about *work*. Whatever the job had
//! already done stays done, whatever it had not done never happens, and the
//! caller who could have coped never hears the question.
//!
//!   rustc --edition 2024 what_a_panic_costs.rs -o /tmp/wpc && /tmp/wpc

use std::panic::{self, AssertUnwindSafe};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

/// What a caught panic told us: its message, the file it named, and that line.
struct Panicked {
    message: String,
    file: String,
    line: u32,
}

/// Run `f`, catching a panic instead of dying, and report what the panic said.
///
/// The hook is swapped out only to keep this demo's output readable — the message
/// is exactly the one that would have reached stderr. Note the `.expect()` calls:
/// a poisoned mutex here really is impossible, because nothing else takes this
/// lock, and the message is where that reasoning gets written down.
fn caught<T>(f: impl FnOnce() -> T) -> Result<T, Panicked> {
    let sink: Arc<Mutex<Option<Panicked>>> = Arc::new(Mutex::new(None));
    let writer = Arc::clone(&sink);

    let prior = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let payload = info.payload();
        let message = payload
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "<payload was not a string>".to_string());
        let (file, line) = match info.location() {
            Some(loc) => (loc.file().to_string(), loc.line()),
            None => (String::new(), 0),
        };
        *writer.lock().expect("nothing else locks this") = Some(Panicked { message, file, line });
    }));
    let outcome = panic::catch_unwind(AssertUnwindSafe(f));
    panic::set_hook(prior);

    match outcome {
        Ok(value) => Ok(value),
        Err(_) => Err(sink
            .lock()
            .expect("nothing else locks this")
            .take()
            .expect("the hook ran before the unwind finished")),
    }
}

fn basename(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("<unknown file>")
}

// ─────────────────────────────────────────────────────────── Step 1
/// The line `gulp`'s `unwrap` sits on — recorded beside it with `line!()` rather
/// than counted by hand, so the comparison below survives an edit to this file.
static GULP_UNWRAP_LINE: AtomicU32 = AtomicU32::new(0);

fn gulp(order: Option<&str>) -> String {
    GULP_UNWRAP_LINE.store(line!() + 1, Ordering::Relaxed);
    let inside = order.unwrap();
    format!("I love {inside}s!")
}

fn step1() {
    banner(1, "Where the panic points — and where it does not");

    println!("  gulp(Some(\"coffee\")) -> {}", gulp(Some("coffee")));

    let call_site = line!() + 1;
    match caught(|| gulp(None)) {
        Ok(s) => println!("  gulp(None) somehow returned {s}"),
        Err(p) => {
            println!("  gulp(None) -> panicked: {}", p.message);
            println!(
                "      reported in {} (line number withheld here: it moves whenever",
                basename(&p.file)
            );
            println!("      this file is edited, and the recorded output would go stale)");
            println!(
                "      is it the `unwrap` line?     {}",
                p.line == GULP_UNWRAP_LINE.load(Ordering::Relaxed)
            );
            println!("      is it the caller's line?     {}", p.line == call_site);
        }
    }
    println!("      `Option::unwrap` is #[track_caller], so the location is YOUR");
    println!("      unwrap rather than a line inside core/src/option.rs. It is still");
    println!("      not the line that handed over the None — that one is only in the");
    println!("      backtrace (RUST_BACKTRACE=1), which is off by default.");
}

// ─────────────────────────────────────────────────────────── Step 2
/// Five drink orders to fill. The third one is missing.
const ORDERS: [Option<&str>; 5] = [Some("water"), Some("coffee"), None, Some("tea"), Some("cola")];

fn step2() {
    banner(2, "A panic is not a return value: the job stops half-done");

    let served = Arc::new(Mutex::new(Vec::new()));
    let log = Arc::clone(&served);
    let outcome = caught(move || {
        for order in ORDERS {
            let drink = order.unwrap();
            log.lock().expect("single-threaded here").push(drink);
        }
    });
    let poured = served.lock().expect("single-threaded here").clone();
    match outcome {
        Ok(()) => println!("  with unwrap: served all of {poured:?}"),
        Err(p) => println!("  with unwrap: panicked (\"{}\") after {poured:?}", p.message),
    }
    println!("      Two glasses poured, three orders never looked at — and the two");
    println!("      poured glasses are still poured. Anything already written to a");
    println!("      file, a socket, or a database stays written.");

    let all: Vec<&str> = ORDERS.iter().map(|o| o.unwrap_or("(nothing)")).collect();
    println!("  with unwrap_or: served {all:?}");
    println!("      Same five orders, one answer each, the gap visible in the result.");
}

// ─────────────────────────────────────────────────────────── Step 3
/// Prints when it is dropped, so we can watch the unwind clean up.
struct Glass(&'static str);

impl Drop for Glass {
    fn drop(&mut self) {
        println!("      Drop: the {} glass is washed up", self.0);
    }
}

fn step3() {
    banner(3, "Unwinding cleans up your memory, not your work");

    let outcome = caught(|| {
        let _first = Glass("first");
        let _second = Glass("second");
        println!("      two glasses in hand, about to unwrap a None");
        Option::<&str>::None.expect("the bar always pours something");
    });
    println!("  caught: {}", outcome.err().map(|p| p.message).unwrap_or_default());
    println!("      Both destructors ran, in reverse order, on the way out — RAII");
    println!("      holds during a panic, which is why a lock is released and a file");
    println!("      is closed. What does NOT happen is the rest of the function.");
    println!("      (With `panic = \"abort\"` in the release profile, not even this:");
    println!("      the process stops where it stands and no destructor runs.)");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "In a thread, only that thread dies");

    let prior = panic::take_hook();
    panic::set_hook(Box::new(|_| {})); // the child's message would go to stderr
    let handle = thread::spawn(|| -> u32 {
        let missing: Option<u32> = None;
        missing.expect("the worker was promised a limit")
    });
    let joined = handle.join();
    panic::set_hook(prior);

    match joined {
        Ok(v) => println!("  worker returned {v}"),
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<payload was not a string>".to_string());
            println!("  worker panicked, join() -> Err(\"{msg}\")");
        }
    }
    println!("  main is still running, and prints this line");
    println!("      A panic unwinds one thread. `join` hands you the payload as an");
    println!("      Err, so the parent decides — which is the choice the unwrap");
    println!("      inside the worker had taken away. Any Mutex the worker held");
    println!("      while it died is now poisoned.");
}

// ─────────────────────────────────────────────────────────── Step 5
/// Re-runs this same binary with a flag that makes it panic for real, so the
/// exit code is observed rather than asserted. Only the code is printed, so the
/// recorded output does not depend on this machine.
fn step5() {
    banner(5, "An uncaught panic is exit code 101, not 1");

    let me = std::env::current_exe().expect("the running binary has a path");
    let child = Command::new(me)
        .arg("--panic-for-real")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("this binary can run itself");

    println!("  a child process that really panics exits {:?}", child.code());
    println!("      101 is std's exit code for an unhandled panic — distinct from 1,");
    println!("      so a supervisor can tell 'the program failed and said so' from");
    println!("      'the program broke'. catch_unwind is how THIS demo survived its");
    println!("      own panics, but it is not a try/catch: it exists for FFI and");
    println!("      test harnesses, and it cannot catch an abort.");
}

fn main() {
    // The child process spawned by step 5 lands here.
    if std::env::args().any(|a| a == "--panic-for-real") {
        let missing: Option<u32> = None;
        missing.expect("this panic is the point: it is never caught");
        return;
    }

    println!("What a panic costs: unwinding is tidy about memory, not about work");
    step1();
    step2();
    step3();
    step4();
    step5();
    println!();
}
