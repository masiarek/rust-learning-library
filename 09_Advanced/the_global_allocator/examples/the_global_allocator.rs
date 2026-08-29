//! Where a `String`'s bytes come from — and how to count them.
//!
//!   rustc --edition 2024 the_global_allocator.rs -o /tmp/tga && /tmp/tga

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);
static FREES: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

/// Wraps the default allocator and tallies what passes through. It does no
/// allocating of its own — `System` still does the work — so the only thing
/// this changes about the program is that the traffic becomes visible.
struct Counting;

// `unsafe impl` because GlobalAlloc is a contract, not just an interface: the
// pointers handed back must be valid for the requested Layout, and everything
// in the program is about to trust that.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        BYTES.fetch_add(layout.size(), Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        FREES.fetch_add(1, Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }

    // Worth overriding rather than taking the default. The provided `realloc`
    // is alloc + copy + dealloc, so leaving it out would both hide the growth
    // (it would read as an ordinary allocation) and force a copy the real
    // allocator can sometimes avoid by extending the block in place.
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        REALLOCS.fetch_add(1, Relaxed);
        BYTES.fetch_add(new_size.saturating_sub(layout.size()), Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

#[derive(Clone, Copy)]
struct Tally {
    allocs: usize,
    reallocs: usize,
    frees: usize,
    bytes: usize,
}

fn tally() -> Tally {
    Tally {
        allocs: ALLOCS.load(Relaxed),
        reallocs: REALLOCS.load(Relaxed),
        frees: FREES.load(Relaxed),
        bytes: BYTES.load(Relaxed),
    }
}

/// Run `work` and report only what IT cost. Nothing is printed inside the
/// measured region — `println!` allocates too, and a counter that includes its
/// own reporting is the first way this measurement goes wrong.
fn measure<T>(label: &str, work: impl FnOnce() -> T) -> T {
    let before = tally();
    let out = work();
    let after = tally();
    println!(
        "   {label:<34} alloc {}  realloc {}  free {}  bytes {}",
        after.allocs - before.allocs,
        after.reallocs - before.reallocs,
        after.frees - before.frees,
        after.bytes - before.bytes,
    );
    out
}

fn main() {
    // Warm up stdout before the first measurement: its one-time setup would
    // otherwise be charged to whatever we happened to measure first.
    println!("1. Every heap byte in the program comes from ONE place");
    println!("   The default is std::alloc::System — malloc/free here. Swapping in");
    println!("   a wrapper that counts is the whole of `#[global_allocator]`.");

    println!();
    println!("2. A String growing, as ALLOCATION EVENTS rather than capacity numbers");
    let grown = measure("String::new() + 3 push_str", || {
        let mut s = String::new();
        s.push_str("aa");
        s.push_str("bbbbbbb");
        s.push_str("cccccccccc");
        s
    });
    println!("   final len {} capacity {}", grown.len(), grown.capacity());
    println!("   capacity went 0 -> 8 -> 16 -> 32: one alloc to buy the first");
    println!("   buffer, then a realloc at each jump. The empty String cost nothing.");

    println!();
    println!("3. with_capacity collapses the ladder to a single purchase");
    let planned = measure("String::with_capacity(32) + 3", || {
        let mut s = String::with_capacity(32);
        s.push_str("aa");
        s.push_str("bbbbbbb");
        s.push_str("cccccccccc");
        s
    });
    println!("   final len {} capacity {}", planned.len(), planned.capacity());
    println!("   Same text, same result, one trip to the allocator instead of three.");

    println!();
    println!("4. What does and does not go to the heap");
    measure("let n: i64 = 42", || {
        let n: i64 = 42;
        std::hint::black_box(n);
    });
    measure("let s: &str = \"a literal\"", || {
        let s: &str = "a literal";
        std::hint::black_box(s);
    });
    measure("Box::new(42_i64)", || std::hint::black_box(Box::new(42_i64)));
    measure("vec![0_u8; 100]", || std::hint::black_box(vec![0_u8; 100]));
    println!("   A literal is already in the binary; an i64 lives on the stack.");
    println!("   Neither one asks the allocator for anything.");

    println!();
    println!("5. The accidental clone, now countable");
    let owned = String::from("already owned");
    measure("&owned  (a view)", || std::hint::black_box(owned.len()));
    measure("owned.to_string()", || std::hint::black_box(owned.to_string()));
    println!("   `.to_string()` on a String is a second buffer for the same bytes.");
    println!("   The borrow is free. That is the whole argument, in one column.");

    println!();
    println!("6. The five spellings, priced");
    // The claim on the Making a `String` page is that four of these are the
    // same call and only `format!` is different. That is checkable, not a
    // matter of taste.
    measure("\"equal vote\".to_owned()", || std::hint::black_box("equal vote".to_owned()));
    measure("String::from(\"equal vote\")", || std::hint::black_box(String::from("equal vote")));
    measure("\"equal vote\".to_string()", || std::hint::black_box("equal vote".to_string()));
    measure("let _: String = \"...\".into()", || {
        let s: String = "equal vote".into();
        std::hint::black_box(s);
    });
    measure("format!(\"equal vote\")", || std::hint::black_box(format!("equal vote")));
    println!("   Five identical rows. Even format!, because std has a fast path");
    println!("   for a format string with no arguments: it compiles down to the");
    println!("   same .to_owned(). Allocation count cannot tell these apart at");
    println!("   all, so `to_string` vs `to_owned` really is a documentation");
    println!("   question — this is the column that retires the 2015 benchmark.");
    println!("   The `free 1` is not a fact about `.into()`: that closure drops");
    println!("   its String inside the measured region while the others hand");
    println!("   theirs back. Where a value dies decides who is charged for it.");

    println!();
    println!("7. What this counter can and cannot tell you");
    println!("   Deltas around a known region: trustworthy, and the numbers above");
    println!("   are the same on every platform because String's growth is Rust's");
    println!("   policy, not the C library's.");
    println!("   Process-wide totals: NOT printed here on purpose. They include");
    println!("   the runtime's own startup and stdout's buffers, which differ");
    println!("   between macOS and Linux — an answer key full of those would fail");
    println!("   in CI for a reason that has nothing to do with your program.");
    println!("   And it counts calls, not live bytes: frees trail allocs while");
    println!("   values are still alive, so this is not a leak detector.");
}
