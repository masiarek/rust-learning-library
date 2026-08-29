# The global allocator

**Level:** 301 · working knowledge

**One line:** Every heap byte in a Rust program is handed out by one global allocator, and `#[global_allocator]` lets you replace it — which is the cheapest honest way to turn *"this feels slow"* into a number.

```rust
#[global_allocator]
static GLOBAL: Counting = Counting;   // your type, wrapping System

// ...and now this prints `alloc 1  realloc 2`
let mut s = String::new();
s.push_str("aa");
s.push_str("bbbbbbb");
s.push_str("cccccccccc");
```

[The anatomy of a `String`](../../14_Strings/anatomy_of_a_string/README.md) shows capacity going `0 → 8 → 16 → 32` and says the jumps are reallocations. This page is how you *watch* them happen instead of inferring them from a number that survived.

## The default is `System`, and that is newer than it looks

`std::alloc::System` is whatever the platform provides — `malloc`/`free` through libc on Unix, `HeapAlloc` on Windows. Nothing clever, and nothing Rust wrote.

It is worth knowing this was not always true: Rust shipped **jemalloc** as the default until 1.32 (January 2019), then removed it. So benchmark advice written before 2019 may be measuring an allocator your program no longer uses — the same trap as the `to_string`/`to_owned` benchmark that [expired in 2016](../../12_Traits/to_owned/README.md#the-argument-you-will-meet-and-its-expiry-date), one layer down. If you want jemalloc back you add it as a crate and declare it, which is the same mechanism this page is about.

## Who actually calls it

| goes to the allocator | does not |
|---|---|
| `String`, `Vec`, `HashMap` — anything that grows | a `let n: i64` — that is stack |
| `Box::new`, `Rc::new`, `Arc::new` | `&str` pointing at a literal — already in the binary |
| every `format!` and `to_string` | `[u8; 32]` — an array is its size |
| growing past capacity — a **realloc**, not a new alloc | pushing *within* capacity — free |

The right-hand column is the reason `with_capacity` is advice at all, and the reason a `&'static str` costs nothing to pass around.

## Replacing it

Three requirements, and the third is the interesting one:

- A type implementing [`GlobalAlloc` ↗](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) — two required methods, `alloc` and `dealloc`.
- Exactly **one** `#[global_allocator]` static in the whole program, binary and all its dependencies included. It is a program-wide decision, which is why a library must not make it.
- `unsafe impl`, because `GlobalAlloc` is a contract rather than an interface: you are promising that the pointer you return is valid for the requested `Layout`, and every collection in the program is about to trust that without checking.

**Override `realloc` even though it is provided.** The default implementation is `alloc` + copy + `dealloc`, so leaving it out does two things you probably did not intend: a growing `String` shows up in your counter as an ordinary allocation, hiding exactly the event you installed the counter to see, and you force a copy that the real allocator can sometimes avoid by extending the block where it already sits.

## The measurement discipline

A counting allocator is easy; a counting allocator that tells the truth needs three rules, and the run below obeys all three.

- **Never print inside the measured region.** `println!` allocates. A counter that includes its own reporting is measuring itself.
- **Warm up stdout first.** Its one-time setup would otherwise be charged to whichever region you happened to measure first.
- **Do not record process-wide totals.** They include the runtime's startup and stdout's buffers, which differ between macOS and Linux — an answer key full of those numbers fails in CI for a reason that has nothing to do with the program. Every number in the run below is a *delta around a known region*, which is stable everywhere because `String`'s growth policy is Rust's, not the C library's.

That last rule is not hypothetical: this page's example printed process totals in its first draft, and they would have gone red the moment CI ran it on Ubuntu.

## What it settles that argument could not

The [five spellings](../../14_Strings/making_a_string/README.md) all cost **one allocation of the same size** — `format!` included, because `std` has a fast path for a format string with no arguments that compiles down to `.to_owned()`. Allocation count cannot distinguish them at all, which is the strongest available form of *"choose on readability"*: there is no hidden cost to trade the readable spelling against.

And the accidental clone becomes a column rather than a claim: `&owned` is `alloc 0`, `owned.to_string()` is `alloc 1`.

## The `Allocator` API is a different, unstable thing

`#[global_allocator]` is stable and program-wide. The [`Allocator` trait ↗](https://doc.rust-lang.org/std/alloc/trait.Allocator.html) is the **per-container** version — `Vec::new_in(my_arena)`, a different allocator for one data structure — and it is **nightly only**, behind `#![feature(allocator_api)]`, [tracked since 2018 ↗](https://github.com/rust-lang/rust/issues/32838). Do not plan around it on stable; use the global hook, or a crate that owns its own memory.

## If you are coming from another language

**Python.** You have no equivalent hook. CPython layers `pymalloc` over `malloc` for small objects and you observe it after the fact with `tracemalloc` or `sys.getsizeof` — measurement, but never substitution. Rust gives you the substitution, which is why the counter above is fifteen lines rather than a profiler.

**ABAP.** Memory is the kernel's business: work process areas, roll and paged memory, and quotas set by Basis rather than by you. `ST02`/`SM04` tell you what happened after it happened. The nearest feeling to this page is reading an ABAP memory dump — except here the accounting is yours to write, and it runs inside the program.

**C++.** The closest match of the three: overriding `operator new`/`operator delete` globally is nearly the same move as `#[global_allocator]`, and `std::pmr::memory_resource` is the per-container version that Rust's `Allocator` trait is still trying to stabilize. The difference is who is trusted — C++ lets any translation unit replace `operator new`, while Rust permits exactly one `#[global_allocator]` in the whole program and rejects a second at link time.

---

## Practice

**Predict, then count.** Build one summary line out of six ballots four different ways: a `format!` per row collected into a `Vec<String>` and joined; a single `String::new()` written into with `write!`; and the same loop twice more with `String::with_capacity` — once with a number you guessed, once with the exact length.

Write your predicted allocation and reallocation counts down *before* you run it. Then say which of your four predictions was wrong and why, and answer the question the fourth row exists to ask: what does `with_capacity` actually buy when the number is one byte short?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:the_global_allocator_kata -->
*[`the_global_allocator_kata.rs`](examples/the_global_allocator_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
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

const BALLOTS: [(&str, u8); 6] = [
    ("Ada", 5), ("Ben", 2), ("Cara", 0),
    ("Dan", 4), ("Eve", 3), ("Fay", 1),
];

/// Worst: a String per row, a Vec to hold them, then a seventh buffer to join.
fn via_collect() -> String {
    BALLOTS
        .iter()
        .map(|(name, score)| format!("{name}={score}"))
        .collect::<Vec<String>>()
        .join(", ")
}

/// Better: one buffer that grows. Every row is a `write!`, no per-row String.
fn via_push() -> String {
    let mut out = String::new();
    for (i, (name, score)) in BALLOTS.iter().enumerate() {
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
    for (i, (name, score)) in BALLOTS.iter().enumerate() {
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
    println!("Six ballots, one line of output, four ways to build it.");
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
```
<!-- /source -->

<!-- output:the_global_allocator_kata -->
*Verified output of [`the_global_allocator_kata.rs`](examples/the_global_allocator_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Six ballots, one line of output, four ways to build it.
Predict the allocation counts before reading them.

   via_collect    alloc  8  realloc  0   len 41
   via_push       alloc  1  realloc  3   len 41
   guessed (40)   alloc  1  realloc  1   len 41
   exact (41)     alloc  1  realloc  0   len 41

via_collect pays for a String PER ROW, plus the Vec holding them,
plus the final joined buffer — and every one of those is freed
immediately. The work is real; the result is thrown away.

via_push keeps one buffer and lets it grow: no per-row String at
all, and the reallocs are the 8/16/32 ladder doing its job.

with_capacity only helps if the number is right. 40 was a guess,
the line is 41 bytes, and being ONE byte short bought back the
reallocation the call existed to avoid. 41 costs one allocation
and nothing else.

The lesson is not 'always call with_capacity'. It is that the
difference between these four was a guess until something counted
— including the guess that with_capacity had worked.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:the_global_allocator -->
*Verified output of [`the_global_allocator.rs`](examples/the_global_allocator.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Every heap byte in the program comes from ONE place
   The default is std::alloc::System — malloc/free here. Swapping in
   a wrapper that counts is the whole of `#[global_allocator]`.

2. A String growing, as ALLOCATION EVENTS rather than capacity numbers
   String::new() + 3 push_str         alloc 1  realloc 2  free 0  bytes 32
   final len 19 capacity 32
   capacity went 0 -> 8 -> 16 -> 32: one alloc to buy the first
   buffer, then a realloc at each jump. The empty String cost nothing.

3. with_capacity collapses the ladder to a single purchase
   String::with_capacity(32) + 3      alloc 1  realloc 0  free 0  bytes 32
   final len 19 capacity 32
   Same text, same result, one trip to the allocator instead of three.

4. What does and does not go to the heap
   let n: i64 = 42                    alloc 0  realloc 0  free 0  bytes 0
   let s: &str = "a literal"          alloc 0  realloc 0  free 0  bytes 0
   Box::new(42_i64)                   alloc 1  realloc 0  free 0  bytes 8
   vec![0_u8; 100]                    alloc 1  realloc 0  free 0  bytes 100
   A literal is already in the binary; an i64 lives on the stack.
   Neither one asks the allocator for anything.

5. The accidental clone, now countable
   &owned  (a view)                   alloc 0  realloc 0  free 0  bytes 0
   owned.to_string()                  alloc 1  realloc 0  free 0  bytes 13
   `.to_string()` on a String is a second buffer for the same bytes.
   The borrow is free. That is the whole argument, in one column.

6. The five spellings, priced
   "equal vote".to_owned()            alloc 1  realloc 0  free 0  bytes 10
   String::from("equal vote")         alloc 1  realloc 0  free 0  bytes 10
   "equal vote".to_string()           alloc 1  realloc 0  free 0  bytes 10
   let _: String = "...".into()       alloc 1  realloc 0  free 1  bytes 10
   format!("equal vote")              alloc 1  realloc 0  free 0  bytes 10
   Five identical rows. Even format!, because std has a fast path
   for a format string with no arguments: it compiles down to the
   same .to_owned(). Allocation count cannot tell these apart at
   all, so `to_string` vs `to_owned` really is a documentation
   question — this is the column that retires the 2015 benchmark.
   The `free 1` is not a fact about `.into()`: that closure drops
   its String inside the measured region while the others hand
   theirs back. Where a value dies decides who is charged for it.

7. What this counter can and cannot tell you
   Deltas around a known region: trustworthy, and the numbers above
   are the same on every platform because String's growth is Rust's
   policy, not the C library's.
   Process-wide totals: NOT printed here on purpose. They include
   the runtime's own startup and stdout's buffers, which differ
   between macOS and Linux — an answer key full of those would fail
   in CI for a reason that has nothing to do with your program.
   And it counts calls, not live bytes: frees trail allocs while
   values are still alive, so this is not a leak detector.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 09_Advanced/the_global_allocator/examples/the_global_allocator.rs -o /tmp/tga && /tmp/tga
```

## See also

- [The anatomy of a `String`](../../14_Strings/anatomy_of_a_string/README.md) — the capacity ladder this page turns into events
- [When `String` is too slow](../../14_Strings/when_string_is_too_slow/README.md) — the page whose "measure first" advice this one makes possible
- [Building a `String`](../../14_Strings/building_a_string/README.md) — `push_str` vs `format!` in a loop, which is the kata above in lesson form
- [Making a `String`](../../14_Strings/making_a_string/README.md) — the five spellings, now priced
- [`clone_into`](../../12_Traits/clone_into/README.md) — this page's counter put to work: refilling one buffer instead of buying a new one per row
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md) — not allocating at all, which beats allocating cheaply
- [What an address shows](../../18_Ownership/what_an_address_shows/README.md) — the other way to watch memory, from the value's side
- [`std::alloc` ↗](https://doc.rust-lang.org/std/alloc/) · [`GlobalAlloc` ↗](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) — the module and the contract
