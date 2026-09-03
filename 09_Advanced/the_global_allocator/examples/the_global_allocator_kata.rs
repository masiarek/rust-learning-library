//! Kata solution: predict the allocation count, then let the allocator answer.
//!
//!   rustc --edition 2024 the_global_allocator_kata.rs -o /tmp/tgak && /tmp/tgak

use std::alloc::{GlobalAlloc, Layout, System};
use std::fmt::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        REALLOCS.fetch_add(1, Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

const ROWS: [(&str, u8); 6] = [
    ("Ada", 5), ("Ben", 2), ("Cara", 0),
    ("Dan", 4), ("Eve", 3), ("Fay", 1),
];

/// Worst: a String per row, a Vec to hold them, then a seventh buffer to join.
fn via_collect() -> String {
    ROWS
        .iter()
        .map(|(name, score)| format!("{name}={score}"))
        .collect::<Vec<String>>()
        .join(", ")
}

/// Better: one buffer that grows. Every row is a `write!`, no per-row String.
fn via_push() -> String {
    let mut out = String::new();
    for (i, (name, score)) in ROWS.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "{name}={score}");
    }
    out
}

/// Told the answer in advance — except the answer was a guess, and it is one
/// byte short. The line is 41 bytes; this reserves 40.
fn via_guessed_capacity() -> String {
    let mut out = String::with_capacity(40);
    fill(&mut out);
    out
}

/// The same call with a number that is actually big enough.
fn via_exact_capacity() -> String {
    let mut out = String::with_capacity(41);
    fill(&mut out);
    out
}

fn fill(out: &mut String) {
    for (i, (name, score)) in ROWS.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "{name}={score}");
    }
}

fn measure(label: &str, work: impl FnOnce() -> String) {
    let (a0, r0) = (ALLOCS.load(Relaxed), REALLOCS.load(Relaxed));
    let out = work();
    let (a1, r1) = (ALLOCS.load(Relaxed), REALLOCS.load(Relaxed));
    println!("   {label:<14} alloc {:>2}  realloc {:>2}   len {}", a1 - a0, r1 - r0, out.len());
}

fn main() {
    println!("Six rows, one line of output, four ways to build it.");
    println!("Predict the allocation counts before reading them.");
    println!();
    measure("via_collect", via_collect);
    measure("via_push", via_push);
    measure("guessed (40)", via_guessed_capacity);
    measure("exact (41)", via_exact_capacity);

    println!();
    println!("via_collect pays for a String PER ROW, plus the Vec holding them,");
    println!("plus the final joined buffer — and every one of those is freed");
    println!("immediately. The work is real; the result is thrown away.");
    println!();
    println!("via_push keeps one buffer and lets it grow: no per-row String at");
    println!("all, and the reallocs are the 8/16/32 ladder doing its job.");
    println!();
    println!("with_capacity only helps if the number is right. 40 was a guess,");
    println!("the line is 41 bytes, and being ONE byte short bought back the");
    println!("reallocation the call existed to avoid. 41 costs one allocation");
    println!("and nothing else.");
    println!();
    println!("The lesson is not 'always call with_capacity'. It is that the");
    println!("difference between these four was a guess until something counted");
    println!("— including the guess that with_capacity had worked.");
}
