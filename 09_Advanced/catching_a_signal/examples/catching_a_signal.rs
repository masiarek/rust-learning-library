// Ctrl-C in Rust: std has no signal API, so the handler comes across the C ABI
// and may do almost nothing. Every signal here is delivered on purpose with
// raise(3), so nothing waits for a key.

use std::ffi::c_int;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

unsafe extern "C" {
    fn signal(sig: c_int, handler: usize) -> usize;
    fn raise(sig: c_int) -> c_int;
}

const SIGINT: c_int = 2;
const SIG_DFL: usize = 0;
const SIG_IGN: usize = 1;

static INTERRUPTED: AtomicBool = AtomicBool::new(false);
static SIGNUM: AtomicI32 = AtomicI32::new(0);
static TIMES: AtomicI32 = AtomicI32::new(0);

// This runs as the real C handler: at whatever instruction the process had
// reached, on whichever thread the kernel picked. Three atomic stores, and
// nothing else. No println!, no allocation, no lock.
extern "C" fn on_sigint(signum: c_int) {
    SIGNUM.store(signum, Ordering::SeqCst);
    TIMES.fetch_add(1, Ordering::SeqCst);
    INTERRUPTED.store(true, Ordering::SeqCst);
}

fn rule(n: u8, title: &str) {
    println!();
    println!("{n}. {title}");
    println!("{}", "-".repeat(62));
}

fn row(label: &str, value: impl std::fmt::Display) {
    println!("{label:.<40} {value}");
}

fn main() {
    rule(1, "What std offers");
    row("std::process", "exit, abort, Command");
    row("std::thread", "park, unpark, sleep");
    row("a way to hear about SIGINT", "none");
    println!();
    println!("There is no std::signal. Ctrl-C is therefore whatever the kernel");
    println!("does by default: the process ends, no unwinding, no Drop, and no");
    println!("destructor for the file you were half-way through writing.");

    rule(2, "Installing a handler through the C ABI");
    let previous = unsafe { signal(SIGINT, on_sigint as *const () as usize) };
    row("signal(SIGINT, on_sigint) returned", previous);
    row("...which is SIG_DFL", previous == SIG_DFL);
    println!();
    println!("SIG_DFL is {SIG_DFL} and SIG_IGN is {SIG_IGN}; anything else is a function");
    println!("pointer. That return value is the only way back to what was there,");
    println!("and nothing keeps a copy for you.");

    rule(3, "A real SIGINT, delivered to ourselves");
    row("INTERRUPTED before", INTERRUPTED.load(Ordering::SeqCst));
    let rc = unsafe { raise(SIGINT) };
    row("raise(SIGINT) returned", rc);
    row("INTERRUPTED after", INTERRUPTED.load(Ordering::SeqCst));
    row("the number the handler was passed", SIGNUM.load(Ordering::SeqCst));
    println!();
    println!("The handler ran, returned, and execution carried on at the next");
    println!("line. A handler that returns resumes the code it interrupted: it is");
    println!("a call, not an unwind, so nothing was dropped on the way through.");

    rule(4, "The flag is the program, not the handler");
    INTERRUPTED.store(false, Ordering::SeqCst);
    for round in 1..=4 {
        if round == 3 {
            unsafe { raise(SIGINT) };
        }
        if INTERRUPTED.load(Ordering::SeqCst) {
            row(&format!("round {round}"), "saw the flag, shutting down");
            break;
        }
        row(&format!("round {round}"), "working");
    }
    println!();
    println!("This is the whole Rust idiom. The handler sets an atomic; the work");
    println!("happens back in ordinary code, where allocating, locking and");
    println!("printing are allowed again. Everything a Python handler may do");
    println!("directly, a Rust handler has to ask for by leaving a note.");

    rule(5, "What the handler may not do");
    println!("It is a C signal handler, so the rule is C's rule: only");
    println!("async-signal-safe calls. That list is short, and nothing which");
    println!("locks or allocates is on it.");
    println!();
    row("    println!(...)", "takes the Stdout lock");
    row("    String::from, vec![], format!", "allocates");
    row("    Mutex::lock", "locks");
    row("    an atomic store", "fine, and is all this one does");
    println!();
    println!("The failure is a deadlock, not a compile error: the signal arrives");
    println!("while main holds the Stdout lock, the handler asks for the same");
    println!("lock on the same thread, and the process stops there. Nothing in");
    println!("the type system says a fn is a signal handler, so nothing checks it.");

    rule(6, "Ignoring, and putting the default back");
    let installed = unsafe { signal(SIGINT, SIG_IGN) };
    row("signal(SIGINT, SIG_IGN) returned", "a function pointer");
    row("...neither SIG_DFL nor SIG_IGN", installed != SIG_DFL && installed != SIG_IGN);
    let before = TIMES.load(Ordering::SeqCst);
    unsafe { raise(SIGINT) };
    row("handler calls before a raise", before);
    row("handler calls after it", TIMES.load(Ordering::SeqCst));
    println!();
    println!("SIG_IGN is the kernel discarding the signal before anyone is woken.");
    println!();
    let ignoring = unsafe { signal(SIGINT, SIG_DFL) };
    row("signal(SIGINT, SIG_DFL) returned", format!("{ignoring}, which is SIG_IGN"));
    println!();
    println!("SIGINT is back to ending the process. Raising it now would stop this");
    println!("program with no further output, which is why this line is the last.");
}
