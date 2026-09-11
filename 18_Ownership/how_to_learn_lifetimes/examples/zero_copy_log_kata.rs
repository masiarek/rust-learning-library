//! Kata solution: a log that keeps views of its lines instead of a `String` per
//! line — and the one struct design the borrow checker will not accept.
//!
//!   rustc --edition 2024 zero_copy_log_kata.rs -o /tmp/zclk && /tmp/zclk

use std::alloc::{GlobalAlloc, Layout, System};
use std::ops::Range;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

/// Counts allocations and reallocations, so "zero-copy" is a number here.
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
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

fn counted<T>(f: impl FnOnce() -> T) -> (T, usize) {
    let before = ALLOCS.load(Relaxed);
    let out = f();
    (out, ALLOCS.load(Relaxed) - before)
}

/// A `String` per line: the obvious design, and an allocation for every line.
fn copies(text: &str) -> Vec<String> {
    text.lines().map(str::to_owned).collect()
}

/// Views into a buffer that lives somewhere else. The lifetime is the whole
/// contract: this log cannot outlive the text it points into.
struct Borrowed<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Borrowed<'a> {
    fn new(text: &'a str) -> Self {
        Borrowed { lines: text.lines().collect() }
    }

    fn errors(&self) -> usize {
        self.lines.iter().filter(|l| l.starts_with("ERROR")).count()
    }
}

/// The buffer inside, and byte RANGES into it instead of references. There is
/// nothing for the borrow checker to connect, so this log can be returned,
/// stored and moved like any other value; a line is sliced out when asked for.
struct Owned {
    buf: String,
    spans: Vec<Range<usize>>,
}

impl Owned {
    fn new(buf: String) -> Self {
        let mut spans = Vec::new();
        let mut start = 0;
        for piece in buf.split_inclusive('\n') {
            let line = piece.trim_end_matches(|c: char| c == '\n' || c == '\r');
            spans.push(start..start + line.len());
            start += piece.len();
        }
        Owned { buf, spans }
    }

    fn line(&self, i: usize) -> &str {
        &self.buf[self.spans[i].clone()]
    }

    fn errors(&self) -> usize {
        (0..self.spans.len()).filter(|&i| self.line(i).starts_with("ERROR")).count()
    }
}

/// A function can hand an `Owned` log to its caller. It could not hand back a
/// `Borrowed` one built from a `String` it made itself: that is E0515.
fn load(lines: usize) -> Owned {
    Owned::new(sample(lines))
}

/// Every hundredth line is an error, so the counts below are easy to check.
fn sample(lines: usize) -> String {
    (0..lines)
        .map(|i| if i % 100 == 0 { format!("ERROR at step {i}\n") } else { format!("ok {i}\n") })
        .collect()
}

fn main() {
    let text = sample(1000); // built before any counting starts

    println!("1. A String per line");
    let (lines, n) = counted(|| copies(&text));
    println!("   {} lines, {n} allocations: one per line, and the Vec growing", lines.len());

    println!();
    println!("2. Views into one buffer, with the buffer outside");
    let (log, n) = counted(|| Borrowed::new(&text));
    println!("   {} lines, {n} allocations: only the Vec of views growing", log.lines.len());
    println!("   {} of them are errors", log.errors());

    println!();
    println!("3. The buffer inside, and ranges instead of references");
    let copy = text.clone(); // the log will own this copy; made before counting
    let (owned, n) = counted(move || Owned::new(copy));
    println!("   {} lines, {n} allocations: the Vec of ranges growing, no text copied", owned.spans.len());
    println!("   line 0 {:?}, line 1 {:?}, {} errors", owned.line(0), owned.line(1), owned.errors());
    let loaded = load(3);
    println!("   load(3) returns a log that owns its text: {:?}", (0..3).map(|i| loaded.line(i)).collect::<Vec<_>>());

    println!();
    println!("4. The design in between does not compile");
    println!("   A struct holding the buffer AND views into it needs a lifetime that names");
    println!("   the struct itself. Building one from a local String is E0515, since the");
    println!("   views point into a local, and E0505, since moving the buffer into the");
    println!("   struct moves it while it is borrowed. Ranges are the way out: an offset");
    println!("   borrows nothing.");
}
