# `Allocator::shrink`

**Level:** 301 · working knowledge

**One line:** What [`Vec::shrink_to_fit`](../../26_Collections/vec_methods/vec_shrink_to_fit/README.md) actually asks for — hand back the tail of this block, and the pointer you get in return is the only one that still works.

```text
unsafe fn shrink(
    &self,
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
) -> Result<NonNull<[u8]>, AllocError>
```

It is a provided method on the [`Allocator` trait ↗](https://doc.rust-lang.org/std/alloc/trait.Allocator.html), which is **nightly only** — `#![feature(allocator_api)]`, [tracked since 2016 ↗](https://github.com/rust-lang/rust/issues/32838). You cannot name it on stable. Every `Vec` in your program calls it anyway.

It is also why `shrink_to_fit`'s own documentation hedges — *"the behavior of this method depends on the allocator, which may either shrink the vector in-place or reallocate"* — which summarises a three-way branch you can go and read.

## The chain

```text
Vec::shrink_to_fit()         guards on capacity() > len, then
  RawVec::shrink_to_fit(len)   which is
    RawVec::shrink_unchecked   which calls one of
      Allocator::deallocate      when the new capacity is 0
      Allocator::shrink          otherwise
```

**Shrinking to zero is not a shrink.** `shrink_unchecked` special-cases `cap == 0` and deallocates the block outright, leaving the vector pointing at a dangling aligned address. A `Vec` you cleared and then shrank makes no `shrink` call at all.

**An already-tight vector never reaches the allocator.** `Vec` checks `capacity() > len` before calling down, so `shrink_to_fit` on a vector with no slack is free rather than cheap.

## What `Global` does with the request

The default allocator answers in three ways, chosen on the numbers — there is no heuristic about whether shrinking is worth the trouble:

| when | what happens |
|---|---|
| new size is 0 | `deallocate`, and return a dangling pointer |
| alignment unchanged | `realloc` — the allocator may resize in place, or move and copy |
| alignment changed | [`allocate` ↗](https://doc.rust-lang.org/std/alloc/trait.Allocator.html#tymethod.allocate) + `copy_nonoverlapping` + `deallocate`, always |

The middle row is the one `Vec` always takes, since `T` does not change. The third row exists because C has `realloc` and `aligned_alloc` but nothing that is both — which is visible one layer further down, where `System::realloc` on Unix hands off to `libc::realloc` only while `layout.align() <= MIN_ALIGN` and otherwise falls back to allocate-copy-free itself.

## The rule that catches people

> Any access to the old `ptr` is Undefined Behavior, **even if the allocation was shrunk in-place**. The newly returned pointer is the only valid pointer for accessing this memory now.

A shrink that keeps the block exactly where it was still invalidates the pointer you passed in. The addresses may be equal; the provenance is not. Code that "knows" the buffer did not move and keeps using the old pointer is unsound on an allocator that happened to agree with it, which is why [`Vec::shrink_to_fit`](../../26_Collections/vec_methods/vec_shrink_to_fit/README.md) says the move is not something to assert on.

`Err` means the opposite: ownership was never transferred, the block is untouched, and the old pointer stays valid. `Vec::shrink_to_fit` has no way to report that, so it aborts the process. The fallible version that hands the `Result` back, `try_shrink_to_fit`, is itself nightly — [tracking issue `vec_fallible_shrink` ↗](https://github.com/rust-lang/rust/issues/152350).

## The provided body is a trap, exactly like `realloc`

If you write an `Allocator` and leave `shrink` alone, you get allocate + copy + deallocate — a guaranteed move and a guaranteed memcpy, for an operation whose entire purpose was to give memory back cheaply. An arena that could have simply lowered its bump pointer instead does the most expensive thing available.

Same shape as the `GlobalAlloc::realloc` default that [The global allocator](../the_global_allocator/README.md) warns about, one layer up, and `grow` carries it too. Three provided methods, three defaults that are correct and slow.

## Example

You cannot call `Allocator::shrink` on stable, but the request still arrives at the global allocator as a `realloc` to a smaller size — so a counting `GlobalAlloc` can watch it. The counters are armed around one operation at a time, and read afterwards, because `println!` allocates.

<!-- source:allocator_shrink -->
*[`allocator_shrink.rs`](examples/allocator_shrink.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
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
```
<!-- /source -->

<!-- output:allocator_shrink -->
*Verified output of [`allocator_shrink.rs`](examples/allocator_shrink.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
what one Vec operation asks the allocator for:
shrink_to_fit, 100 -> 3            alloc 0  realloc 1  dealloc 0   400 bytes -> 12
  ...and the vector still holds [1, 2, 3]
shrink_to_fit, 64 -> 0             alloc 0  realloc 0  dealloc 1
shrink_to_fit, already tight       alloc 0  realloc 0  dealloc 0
shrink_to(50), 100 -> 50           alloc 0  realloc 1  dealloc 0   400 bytes -> 200
into_boxed_slice, 100 -> 3         alloc 0  realloc 1  dealloc 0   400 bytes -> 12
  ...and the box holds [1, 2, 3]
push past capacity (grow)          alloc 0  realloc 1  dealloc 0   16 bytes -> 32
```
<!-- /output -->

`into_boxed_slice` makes the identical request because it calls `shrink_to_fit` itself — literally, as its first statement — and then drops the capacity field. Growing lands on the same hook pointing the other way: that last row is `Allocator::grow`.

## If you are coming from another language

**C.** `realloc(ptr, smaller)` is the same call, and the same rule already applies: on success the old pointer is dead whether or not the block moved. What changed is that Rust writes it into a safety contract instead of leaving it as folklore, and that `Layout` carries the alignment — so an over-aligned block can be shrunk at all, which in C means writing the allocate-copy-free path by hand every time.

**C++.** `std::pmr::memory_resource` is the per-container allocator that `Allocator` is still trying to become, and its `do_deallocate` takes the size and alignment back for the same reason `shrink` takes `old_layout` — the allocator was never asked to remember them. The difference is scope: any C++ translation unit may replace global `operator new`, while Rust permits one `#[global_allocator]` per program and rejects a second at link time.

**Python.** No equivalent. `list` over-allocates and shrinks on its own schedule, and CPython's `PyObject_Realloc` is not reachable from Python code; `sys.getsizeof` reports the result after the fact. The closest thing to a deliberate shrink is rebuilding the list, which is the allocate-copy-free path with no way to ask for the cheaper one.

**ABAP.** An internal table's memory is the kernel's, and `FREE itab` releases it wholesale rather than trimming it — there is no "keep the rows, return the slack". The nearest analogue to `shrink_to_fit` is that `FREE` is a different statement from `CLEAR` for the same reason [`clear`](../../26_Collections/vec_methods/vec_clear/README.md) and `shrink_to_fit` are different methods: emptying a container and returning its memory are separate decisions.

## See also

- [The global allocator](../the_global_allocator/README.md) — the stable, program-wide hook this is measured through
- [`Vec::shrink_to_fit`](../../26_Collections/vec_methods/vec_shrink_to_fit/README.md) — the caller, and the one most people ever touch
- [`Vec::into_boxed_slice`](../../26_Collections/vec_methods/vec_into_boxed_slice/README.md) — the same shrink, followed by dropping the capacity field
- [`Vec::shrink_to`](../../26_Collections/vec_methods/vec_shrink_to/README.md) — the same request with a floor
- [Stack and heap](../../18_Ownership/stack_and_heap/README.md) — where the block came from
- [What `unsafe` turns off](../what_unsafe_turns_off/README.md) — why the contract above is a safety contract

[`Allocator::shrink` in the standard library ↗](https://doc.rust-lang.org/std/alloc/trait.Allocator.html#method.shrink) · [`allocator_api` tracking issue ↗](https://github.com/rust-lang/rust/issues/32838)
