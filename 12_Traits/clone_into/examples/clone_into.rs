//! `clone_into` writes into a buffer you already own; `to_owned` allocates a new one.
//! Every claim below is a counted allocation, not an inferred one.
//!
//!   rustc --edition 2024 clone_into.rs -o /tmp/ci && /tmp/ci

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);

/// Wraps the system allocator and counts what passes through it. It allocates
/// nothing itself, so the only thing it changes about the program is that the
/// traffic becomes visible. Same shape as the one on the global-allocator page.
struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    // Counted separately: a grow-in-place is not a fresh allocation, and
    // lumping the two together would hide exactly the case section 3 is about.
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        REALLOCS.fetch_add(1, Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Run `work` and report only what it cost. Nothing is printed inside the
/// measured region: `println!` allocates, and a counter that includes its own
/// reporting is the first way this measurement goes wrong.
fn measure<T>(label: &str, work: impl FnOnce() -> T) -> T {
    let (a0, r0) = (ALLOCS.load(Relaxed), REALLOCS.load(Relaxed));
    let out = work();
    let (a1, r1) = (ALLOCS.load(Relaxed), REALLOCS.load(Relaxed));
    println!(
        "   {label:<42} alloc {:<3} realloc {}",
        a1 - a0,
        r1 - r0
    );
    out
}

const ROWS: [&str; 4] = ["Ada Lovelace", "Ben Carter", "Cara Ng", "Dev Patel"];

#[derive(Clone)]
struct Row {
    name: String,
}

/// A hand-written `Clone` that forwards `clone_from` to the field's own.
struct TunedRow {
    name: String,
}

impl Clone for TunedRow {
    fn clone(&self) -> Self {
        TunedRow { name: self.name.clone() }
    }
    fn clone_from(&mut self, source: &Self) {
        self.name.clone_from(&source.name);
    }
}

fn main() {
    println!("1. Same result, different cost");
    let mut buf = String::with_capacity(32);
    let fresh = measure("s.to_owned()", || ROWS[0].to_owned());
    measure("s.clone_into(&mut buf)  (buf has room)", || {
        ROWS[0].clone_into(&mut buf)
    });
    println!("   both produced {fresh:?} — one of them bought a new buffer to do it");

    println!();
    println!("2. Where it pays: a loop with a reusable buffer");
    measure("4 rows, a fresh String each time", || {
        for row in ROWS {
            let owned = row.to_owned();
            let _ = owned.len();
        }
    });
    measure("4 rows, one buffer refilled", || {
        for row in ROWS {
            row.clone_into(&mut buf);
            let _ = buf.len();
        }
    });
    println!("   the second loop allocates once — before the loop, not in it");

    println!();
    println!("3. It is not magic: the buffer has to be big enough");
    let mut empty = String::new();
    measure("clone_into an empty String", || {
        ROWS[0].clone_into(&mut empty)
    });
    let mut tight = String::with_capacity(4);
    measure("clone_into a 4-byte String", || {
        ROWS[0].clone_into(&mut tight)
    });
    println!("   an empty target allocates; a too-small one grows. Neither is free.");

    println!();
    println!("4. The saving exists only where the impl overrides the default");
    // `#[derive(Clone)]` emits `clone`, and nothing else — so `clone_from`
    // falls back to the trait's `*self = source.clone()`.
    let src = Row { name: "a name long enough to need the heap".to_string() };
    let mut derived = Row { name: String::with_capacity(64) };
    measure("#[derive(Clone)] struct: clone_from", || {
        derived.clone_from(&src)
    });

    let tuned_src = TunedRow { name: src.name.clone() };
    let mut tuned = TunedRow { name: String::with_capacity(64) };
    measure("hand-written clone_from: clone_from", || {
        tuned.clone_from(&tuned_src)
    });
    println!("   the derive was asked to do this in 2022 and libs-api said no —");
    println!("   write clone_from by hand if you want it (rust#98374, wontfix)");

    println!();
    println!("5. On a slice of Strings, the INNER buffers are reused too");
    let names: Vec<String> = ROWS.iter().map(|s| s.to_string()).collect();
    let mut slots: Vec<String> = (0..4).map(|_| String::with_capacity(32)).collect();
    measure("Vec<String> to_owned", || names[..].to_owned());
    measure("Vec<String> clone_into (roomy slots)", || {
        names[..].clone_into(&mut slots)
    });
    println!("   to_owned bought 5 buffers: the Vec, then one String each.");
    println!("   clone_into bought none — [T]'s impl clones into the slots in place.");

    println!();
    println!("6. What you trade for it: the buffer keeps its high-water mark");
    let long = "x".repeat(300);
    let mut kept = String::new();
    long.clone_into(&mut kept);
    let big = kept.capacity();
    "tiny".clone_into(&mut kept);
    println!("   after a 300-byte row: capacity {big}");
    println!("   after a 4-byte row:   capacity {} , len {}", kept.capacity(), kept.len());
    println!("   a long-lived buffer holds the largest row it ever saw.");

    println!();
    println!("7. The two spellings point opposite ways");
    let mut dst = String::with_capacity(32);
    "source".clone_into(&mut dst); //           receiver is the SOURCE
    println!("   \"source\".clone_into(&mut dst) -> dst = {dst:?}");
    dst.clone_from(&String::from("other")); //  receiver is the DESTINATION
    println!("   dst.clone_from(&other)        -> dst = {dst:?}");

    println!();
    println!("8. The shape clippy actually flags — and whether it pays");
    // A test fixture overriding one field: the canonical `assigning_clones`
    // site. Whether the rewrite buys anything is decided by two byte counts
    // nobody looks at, so measure both before believing the suggestion.
    println!("   fixture default {:?} is {} bytes", DEFAULT_FIELD, DEFAULT_FIELD.len());
    println!("   override        {:?} is {} bytes", OVERRIDE, OVERRIDE.len());

    let mut short = fixture(DEFAULT_FIELD);
    println!("   capacity the fixture handed over: {}", short.field.capacity());
    measure("assignment  (one byte short)", || {
        short.field = OVERRIDE.to_owned()
    });
    let mut short2 = fixture(DEFAULT_FIELD);
    measure("clone_into  (one byte short)", || {
        OVERRIDE.clone_into(&mut short2.field)
    });

    let mut roomy = fixture(DEFAULT_FIELD_FITS);
    measure("assignment  (default already fits)", || {
        roomy.field = OVERRIDE.to_owned()
    });
    let mut roomy2 = fixture(DEFAULT_FIELD_FITS);
    measure("clone_into  (default already fits)", || {
        OVERRIDE.clone_into(&mut roomy2.field)
    });
    println!("   one byte short, the rewrite trades an alloc for a realloc.");
    println!("   give the default room and the same rewrite is free.");
}

const DEFAULT_FIELD: &str = "default_value"; //      13 bytes
const DEFAULT_FIELD_FITS: &str = "default_value!"; // 14, the width of the override
const OVERRIDE: &str = "override_value"; //          14 bytes

struct Fixture {
    field: String,
}

fn fixture(default: &str) -> Fixture {
    Fixture { field: default.to_owned() }
}
