//! `Vec::shrink_to_fit` bottoms out in `Allocator::shrink`. That method is
//! nightly-only, so you cannot name it here — but you can watch what it asks
//! the global allocator for, which is the part that has consequences.
//!
//!   rustc --edition 2024 allocator_shrink.rs -o /tmp/allocator_shrink && /tmp/allocator_shrink

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};

static ARMED: AtomicBool = AtomicBool::new(false);
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);
static DEALLOCS: AtomicUsize = AtomicUsize::new(0);
static OLD_SIZE: AtomicUsize = AtomicUsize::new(0);
static NEW_SIZE: AtomicUsize = AtomicUsize::new(0);

// A pass-through allocator that records what it was asked for, but only while
// armed — so the counts belong to one named region and not to stdout or the
// runtime's own startup.
struct Watch;

unsafe impl GlobalAlloc for Watch {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if ARMED.load(Relaxed) {
            ALLOCS.fetch_add(1, Relaxed);
        }
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ARMED.load(Relaxed) {
            DEALLOCS.fetch_add(1, Relaxed);
        }
        unsafe { System.dealloc(ptr, layout) }
    }
    // Overridden on purpose: the default is alloc + copy + dealloc, which would
    // report every resize as a fresh allocation and hide the whole lesson.
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if ARMED.load(Relaxed) {
            REALLOCS.fetch_add(1, Relaxed);
            OLD_SIZE.store(layout.size(), Relaxed);
            NEW_SIZE.store(new_size, Relaxed);
        }
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: Watch = Watch;

/// Run one operation with the counters on, and report afterwards — never
/// inside, because `println!` allocates too.
fn watch(label: &str, op: impl FnOnce()) {
    ALLOCS.store(0, Relaxed);
    REALLOCS.store(0, Relaxed);
    DEALLOCS.store(0, Relaxed);
    OLD_SIZE.store(0, Relaxed);
    NEW_SIZE.store(0, Relaxed);

    ARMED.store(true, Relaxed);
    op();
    ARMED.store(false, Relaxed);

    let (a, r, d) = (
        ALLOCS.load(Relaxed),
        REALLOCS.load(Relaxed),
        DEALLOCS.load(Relaxed),
    );
    let resize = if r == 0 {
        String::new()
    } else {
        format!(
            "   {} bytes -> {}",
            OLD_SIZE.load(Relaxed),
            NEW_SIZE.load(Relaxed)
        )
    };
    println!("{label:<34} alloc {a}  realloc {r}  dealloc {d}{resize}");
}

fn main() {
    // Warm up stdout before arming anything: its one-time setup would otherwise
    // be charged to whichever region happens to run first.
    println!("what one Vec operation asks the allocator for:");

    // The ordinary case. Same alignment, smaller size, so the request is a
    // shrinking `realloc` — one call, no copy that the allocator did not choose.
    let mut v: Vec<u32> = Vec::with_capacity(100);
    v.extend_from_slice(&[1, 2, 3]);
    watch("shrink_to_fit, 100 -> 3", || v.shrink_to_fit());
    println!("  ...and the vector still holds {v:?}");

    // Shrinking to zero is the branch that is not a shrink at all: `RawVec`
    // deallocates instead, because there is no block worth keeping.
    let mut v = vec![0u8; 64];
    v.clear();
    watch("shrink_to_fit, 64 -> 0", || v.shrink_to_fit());

    // Nothing to ask for: `Vec` guards on `capacity() > len` before it calls
    // down at all, so an already-tight vector never reaches the allocator.
    let mut v = vec![1u16, 2, 3];
    watch("shrink_to_fit, already tight", || v.shrink_to_fit());

    // `shrink_to` is the same request with a floor — the allocator sees the
    // floor, not the length.
    let mut v: Vec<u32> = Vec::with_capacity(100);
    v.extend_from_slice(&[1, 2, 3]);
    watch("shrink_to(50), 100 -> 50", || v.shrink_to(50));

    // `into_boxed_slice` calls `shrink_to_fit` itself, so it makes exactly the
    // same request. The `Box<[T]>` is the same buffer, minus the capacity field.
    let mut v: Vec<u32> = Vec::with_capacity(100);
    v.extend_from_slice(&[1, 2, 3]);
    let mut boxed = None;
    watch("into_boxed_slice, 100 -> 3", || {
        boxed = Some(v.into_boxed_slice())
    });
    println!("  ...and the box holds {:?}", boxed.unwrap());

    // The mirror image: growing past capacity is `Allocator::grow`, which lands
    // on the same `realloc` hook pointing the other way.
    let mut v: Vec<u32> = Vec::with_capacity(4);
    v.extend_from_slice(&[1, 2, 3, 4]);
    watch("push past capacity (grow)", || v.push(5));
}
