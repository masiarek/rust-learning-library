//! Nesting calls costs memory that is bounded and cannot grow, and running out
//! is an abort rather than an error you can catch.
//!
//! The overflow is real: this program re-runs ITSELF with a flag, lets the
//! child die, and reports what the parent saw. That is the only honest way to
//! demonstrate it, because an overflow aborts the process -- there is no
//! `Result` to return and nothing to unwind.
//!
//! Nothing machine-specific is printed. Depths are fixed, and the child's
//! stderr is tested for a substring rather than echoed: it carries a thread id
//! that differs every run.
//!
//!   rustc --edition 2024 recursion_and_the_stack.rs -o /tmp/rats && /tmp/rats

use std::env;
use std::panic;
use std::process::Command;
use std::thread;

/// A marker that reports when its frame lets go of it.
struct Frame(u32);

impl Drop for Frame {
    fn drop(&mut self) {
        println!("       [drop] the marker in frame {} is released", self.0);
    }
}

/// Recursion with something alive in every frame, so "they all stay" is visible.
fn descend(depth: u32, limit: u32) {
    let _marker = Frame(depth);
    println!("   entering frame {depth}");
    if depth < limit {
        descend(depth + 1, limit);
    } else {
        println!("   deepest frame reached; {limit} markers are alive at once");
    }
}

/// A plain sum, written the way that costs one frame per step.
fn sum_recursive(n: u64) -> u64 {
    if n == 0 { 0 } else { n + sum_recursive(n - 1) }
}

/// The same answer at a constant depth: the counter moved into a register,
/// and the "stack" of remaining work became a number.
fn sum_iterative(n: u64) -> u64 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

/// Unbounded on purpose. The `pad` makes each frame big enough that the child
/// dies promptly, and the comparison keeps `unconditional_recursion` quiet --
/// the compiler warns about a function that provably cannot return.
fn forever(depth: u64, ceiling: u64) -> u64 {
    let mut pad = [0u8; 1024];
    pad[0] = depth as u8;
    if depth >= ceiling {
        return pad[0] as u64;
    }
    forever(depth + 1, ceiling) + pad[0] as u64
}

/// The two things the child can be asked to do, both of which end the process.
fn run_child(mode: &str) -> (bool, bool) {
    let exe = env::current_exe().expect("current_exe");
    let out = Command::new(exe).arg(mode).output().expect("spawn");
    let stderr = String::from_utf8_lossy(&out.stderr);
    (out.status.success(), stderr.contains("has overflowed its stack"))
}

fn main() {
    // The child half. Reached only when this binary re-runs itself.
    let mode = env::args().nth(1).unwrap_or_default();
    if mode == "--overflow" {
        forever(0, u64::MAX);
        return;
    }
    if mode == "--overflow-caught" {
        // A catch_unwind that does not help, which is the point of running it.
        let _ = panic::catch_unwind(|| forever(0, u64::MAX));
        return;
    }

    println!("1. Every nested call adds a frame, and nothing leaves until it returns");
    descend(1, 4);
    println!("   ...and they are released in reverse, innermost first");

    println!("\n2. Depth is the cost, not the work");
    let n = 5_000;
    println!("   sum_recursive({n}) = {}   {n} frames, one per step",
             sum_recursive(n));
    println!("   sum_iterative({n}) = {}   one frame, whatever n is",
             sum_iterative(n));
    println!("   same answer; only one of the two has a depth limit");
    let big = 1_000_000;
    println!("   sum_iterative({big}) = {}", sum_iterative(big));
    println!("   the recursive version cannot be asked that. Part 4 shows what happens.");

    println!("\n3. The bound is chosen when a thread is spawned, and never grows");
    let handle = thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| sum_recursive(1_000))
        .expect("spawn");
    println!("   a thread asked for 256 KiB ran 1,000 frames fine: {}",
             handle.join().unwrap());
    println!("   there is no reallocation and no growth: a stack is a fixed region,");
    println!("   so the only question is whether the depth fits the one you asked for");

    println!("\n4. Running out is an abort. Not a panic, not a Result.");
    let (ok, said_overflow) = run_child("--overflow");
    println!("   child recursed without a base case, and exited successfully? {ok}");
    println!("   the runtime named the reason on stderr:                      {said_overflow}");
    let (ok_caught, said_overflow_caught) = run_child("--overflow-caught");
    println!("   the same run wrapped in catch_unwind, exited successfully?   {ok_caught}");
    println!("   and it still died with the same message:                     {said_overflow_caught}");
    println!("   catch_unwind catches PANICS. An overflow does not unwind, so there");
    println!("   is nothing to catch -- the process is killed where it stands.");

    println!("\n5. What this leaves behind");
    println!("   a panic runs every Drop on the way out; an abort runs none.");
    println!("   files stay open, locks stay locked, buffers stay unflushed --");
    println!("   the OS reclaims the memory, and nothing else gets a say.");
}
