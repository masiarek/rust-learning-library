# `clone_into`: refill a buffer instead of buying a new one

**Level:** 201 · working knowledge

**One line:** `to_owned()` hands you a new `String`; `clone_into(&mut s)` writes into the `String` you already have — over a loop, that is one allocation per row against none.

```rust
fn main() {
    let rows = ["Ada Lovelace", "Ben Carter", "Cara Ng"];
    let mut buf = String::with_capacity(32);
    for row in rows {
        row.clone_into(&mut buf);          // no allocation: buf's bytes are overwritten
        println!("{}", buf.len());         // 12 / 10 / 7
    }
}
```

Swap `row.clone_into(&mut buf)` for `buf = row.to_owned()` and the program buys three heap buffers and frees three, for the same three lines of output.

## The pair, in the trait

```rust
pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;                 // required
    fn clone_into(&self, target: &mut Self::Owned) {   // provided
        *target = self.to_owned();                     // the default just allocates
    }
}
```

`to_owned` returns a value and is `#[must_use]`. `clone_into` returns `()` and its whole product is the mutation. Implementing the trait means writing `to_owned`; `clone_into` arrives free, and the free version saves nothing — the default body is the allocating call with an assignment on top. The saving exists only where an impl overrode it, which for `str` is:

```rust
fn clone_into(&self, target: &mut String) {
    target.clear();
    target.push_str(self);
}
```

`clone_into` was added in [PR 41009 ↗](https://github.com/rust-lang/rust/pull/41009) in April 2017 and stabilized in **1.63.0** in August 2022. Five years, and the objection that held it up was raised in the pull request that introduced it.

## What it saves, counted

The run below wraps the global allocator in a counter, so none of these numbers is inferred:

| | allocations |
|---|---|
| 4 rows, a fresh `String` each time | 4 |
| 4 rows, one buffer refilled | 0 |
| `Vec<String>` of 4 names, `to_owned` | 5 |
| `Vec<String>` of 4 names, `clone_into` into roomy slots | 0 |

The `Vec<String>` row is the one worth stopping on. `[T]`'s impl does not merely reuse the `Vec`'s own buffer — for the overlapping prefix it calls `clone_from_slice`, so every inner `String` is refilled in place too:

```rust
default fn clone_into(&self, target: &mut Vec<T, A>) {
    target.truncate(self.len());
    let (init, tail) = self.split_at(target.len());
    target.clone_from_slice(init);      // reuse the contained values' allocations
    target.extend_from_slice(tail);
}
```

Five buffers become zero: the `Vec`, and one per element. That body is the general case of an internal `SpecCloneIntoVec`, which is what `default` marks: the case that specializes is elements whose clone is a bitwise copy — for a `Vec<u8>` or `Vec<i32>` std clears the target and `extend_from_slice`s, there being no inner buffers to reuse.

**Roomy** is two conditions, not one. The impl truncates the target to the source's length, refills the overlapping prefix, and pushes whatever is left over — so four rows into four 4-byte slots reallocate four times, and four rows into two roomy slots still buy two `String`s and grow the `Vec`. Section 5 of the run below counts both. A `Vec<String>` reaches zero only when the target has enough slots *and* each `String` in them has enough capacity.

## Three ways it does not pay

**The target has to have room.** An empty `String` allocates on the first `clone_into` exactly as `to_owned` would; a four-byte one reallocates to grow. The saving is opportunistic, and it is real only from the second call onward — which is why the pattern is a buffer hoisted *out* of a loop, never a fresh one inside it.

**The buffer keeps its high-water mark.** After a 300-byte row, the buffer's capacity is 300, and a subsequent 4-byte row leaves it there. A long-lived buffer holds the largest row it ever saw, so this trades memory for allocator traffic rather than winning both.

**`#[derive(Clone)]` gives you none of it.** The derive emits `clone` and nothing else, so `clone_from` on a derived struct falls through to the trait's default `*self = source.clone()` — and a `#[derive(Clone)] struct Row { name: String }` allocates a fresh `String` on every `clone_from`, with the destination's existing capacity untouched. That is measured in section 4 of the run below. It is also deliberate. Making the derive forward to each field's `clone_from` was [proposed and implemented ↗](https://github.com/rust-lang/rust/pull/98445), and libs-api [declined it in July 2022 ↗](https://github.com/rust-lang/rust/pull/98445#issuecomment-1190681305) — *people can hand-implement `clone_from` if they need the performance, but we shouldn't do so by default* — which closed [the issue ↗](https://github.com/rust-lang/rust/issues/98374) as wontfix. The argument that reached the meeting was not only about performance: the default `clone_from` builds the whole new value and then assigns it, so a panic partway through leaves the destination untouched, while a field-by-field version can leave it half-updated. So on your own types the reuse is opt-in, one hand-written `clone_from` at a time — and what you are opting into is a weaker guarantee, not just a faster path.

## Which types the reuse reaches

Twenty-four `Clone` impls in std as of 1.98 override `clone_from`: the collections (`String`, `Vec`, `VecDeque`, `LinkedList`, `BinaryHeap`, `HashMap`, `HashSet`, `BTreeSet`), the owned path and OS strings (`PathBuf`, `OsString`), and the wrappers that forward to something owning a buffer (`Box`, `Option`, `Result`, `Cow`, `RefCell`, `[T; N]`). Forwarding is the interesting half. `Box<String>` and `Some(String)` reuse the `String` inside them — `Box<T>`'s own documentation asserts it, *"And no allocation occurred"* — while a `None` target has nothing to forward to and allocates.

An override that forwards to a type *without* one buys nothing, and `BTreeSet` is that case: its `clone_from` calls `BTreeMap`'s, `BTreeMap` has none, so four `String`s land in a freshly built tree. It allocates five times where the same four in a `HashSet` allocate four — and those four are the elements, since reusing a container's own buffer says nothing about the values inside it. Same distinction section 5 draws for `Vec<String>`.

A `Copy` type is not on the list and cannot be: the default `clone_from` is `*self = source.clone()`, and for a `u64` that is a register move of a value that owns no buffer. There is nothing to save, so nothing to override. Section 9 of the run below counts all six cases.

## The direction is backwards, and everyone knew before it shipped

The receiver is the **source**:

```rust
src.clone_into(&mut dst);   // ToOwned::clone_into — data moves left to right
dst.clone_from(&src);       // Clone::clone_from   — data moves right to left
```

Which of the two you can call is a question about types, not taste. `clone_from` takes `&Self`, so both sides must already be the same owned type; `&str` → `String` and `&[T]` → `Vec<T>` cross a borrow boundary and have only `clone_into`. Where both spellings exist they are the same call — the blanket `impl<T: Clone> ToOwned for T` defines `clone_into` as `target.clone_from(self)` — and clippy picks the spelling for you, rewriting `s = src.to_owned()` to `src.clone_into(&mut s)` and `v = other.clone()` to `v.clone_from(&other)`.

Two methods that do the same job in the same standard library, pointing opposite ways. scottmcm flagged it [in the PR that introduced the method ↗](https://github.com/rust-lang/rust/pull/41009#issue-113803675) — *"the directionality is weird … and that means that autoref doesn't work well, usually forcing you to write `&mut`"* — and the same words open [the tracking issue ↗](https://github.com/rust-lang/rust/issues/41263).

The reason it shipped anyway is a coherence problem, and it is worth reading as a case of the language shaping an API. The method sits on `ToOwned` so that it can have a default body written in terms of `to_owned`. But `ToOwned`'s `Self` is the *borrowed* type, so method syntax puts the borrowed value in the receiver slot and the owned one in the argument — hence `clone_into` rather than an `owned_from` that would read the right way round. Moving it to a trait of its own would fix the name and lose the default: a blanket impl providing it would overlap the specific ones. Stabilization was proposed in March 2018, formally blocked on that concern, and cancelled that August for wanting *"someone to champion a new API proposal"*. Nobody did. It was revived in the libs-api meeting of March 2022 for a reason unrelated to the objection — `Cow::clone_from` needs it, and third-party `ToOwned` impls should be able to override it:

```rust
fn clone_from(&mut self, source: &Self) {
    match (self, source) {
        (&mut Owned(ref mut dest), &Owned(ref o)) => o.borrow().clone_into(dest),
        (t, s) => *t = s.clone(),
    }
}
```

The concern was never answered. It was outvoted by a use case.

### What the weird direction costs in practice

Clippy's [`assigning_clones` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#assigning_clones) lint rewrites `name = src.to_owned()` into `src.clone_into(&mut name)`. The two are equivalent in value and not in borrows: `=` evaluates the right-hand side first, so any borrow it took has ended by the time the assignment happens, while `clone_into` needs `&mut name` *during* a call whose receiver may itself borrow from `name`.

```rust
let mut name = String::from("Lovelace, Ada");
let parts: Vec<&str> = name.split(", ").collect();
let last = parts[parts.len() - 1];
name = last.to_owned();                 // fine
// last.clone_into(&mut name);          // E0502 — `last` still borrows `name`
```

```text
error[E0502]: cannot borrow `name` as mutable because it is also borrowed as immutable
  |
3 |     let parts: Vec<&str> = name.split(", ").collect();
  |                            ---- immutable borrow occurs here
5 |     last.clone_into(&mut name);
  |          ---------- ^^^^^^^^^ mutable borrow occurs here
  |          |
  |          immutable borrow later used by call
```

That was filed as [clippy#12444 ↗](https://github.com/rust-lang/rust-clippy/issues/12444) in March 2024, labelled `I-suggestion-causes-error`, and fixed by teaching the lint to bail out when the source borrows from the target. Verified on the pinned toolchain: clippy no longer suggests the rewrite for the snippet above. Two notes for reading the lint in the wild. It is **pedantic**, so it fires only if you asked for it — but it shipped in **1.78** as a warn-by-default `perf` lint and was moved in **1.80**, on the stated grounds that it *"suggests to make your code less readable for a small performance gain"* ([clippy#12779 ↗](https://github.com/rust-lang/rust-clippy/pull/12779), [#12778 ↗](https://github.com/rust-lang/rust-clippy/issues/12778)). Anyone who remembers it firing unasked is remembering 1.79 or earlier. And the hedge is this lint's own: *"assigning the result of `Clone::clone()` **may** be inefficient"* is the only hedged efficiency claim clippy makes, and its neighbours in `perf` say it flat (*"swapping with a temporary value is inefficient"*, *"calling `.bytes()` is very inefficient"*). The three ways above are what the *may* is carrying.

### Where you will actually meet the lint

A test fixture overriding one field is the canonical `assigning_clones` site:

```rust
#[derive(Default)]
struct Data {
    field: String,
}

fn test_data() -> Data {
    Data { field: "default_value".to_string() }
}

#[test]
fn test() {
    let mut data = test_data();
    data.field = "override_value".to_owned();   // the flagged shape
}
```

**The lint stays silent here.** Clippy skips `assigning_clones` in test code: inside a `#[test]` function, and anywhere inside a `#[cfg(test)]` module — a plain helper down there, carrying no attribute of its own, is skipped too. It is the `cfg` that decides it, not the name: the identical statement in a module merely *called* `tests` is reported. So a codebase can hold hundreds of these and see nothing, which is fine, since test setup is where the saving matters least.

**Applying the suggestion would buy nothing.** `"default_value"` is 13 bytes and `"override_value"` is 14, so the fixture hands over a buffer one byte too small, and the rewrite trades one allocation for one reallocation. Widen the default to 14 and the same rewrite is free. Section 8 of the run below measures both. Nothing about the source says which case you are in — two string literals decide it — which is what *"may be inefficient"* in the lint's own wording is being careful about.

**The `#[derive(Default)]` is unused, and it is hiding a louder lint.** `test_data()` builds every field by hand, so nothing calls it. Write `Data::default()` there instead and clippy reports [`field_reassign_with_default` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#field_reassign_with_default) — *"field assignment outside of initializer for an instance created with `Default::default()`"* — which is **warn-by-default**, unlike the pedantic one above. A fixture function suppresses it by putting a function call between the default and the override.

## If you are coming from another language

- **Python.** `lst[:] = other` against `lst = list(other)` is the same distinction, and Python makes it visible the same way: `id(lst)` survives the first and changes under the second. Everything that holds a reference to that list sees the update in the first case and keeps the stale object in the second — so in Python the choice is usually about aliasing and only incidentally about allocation. Rust inverts the emphasis. The aliasing question is settled by the borrow checker before you get here, so `clone_into` is left arguing purely about cost, which is why it is a `perf` idea rather than a correctness one. There is no counterpart for text at all: `str` is immutable, so a Python string buffer cannot be refilled, and the pattern to reach for is `io.StringIO` or a `bytearray`. And note what Python does not have — a `list.clear(); list.extend(other)` reuses the list object but the *elements* are references, so nothing corresponds to `[T]`'s reuse of the contained values' own buffers.
- **ABAP.** `lt_target = lt_source` deep-copies the internal table, and the kernel frees the old contents; the reuse spelling is `CLEAR lt_target. APPEND LINES OF lt_source TO lt_target.` — literally `clear` then `push_str`, which is `str`'s impl. What transfers is the instinct, since anyone who has tuned an ABAP loop already knows to hoist the work area and write into it rather than rebuild it. What changes is that ABAP lets you *forget*: `lt_target = lt_source` in a loop is legal, silent, and slow, and the only feedback is SAT or a runtime measurement. Rust hands you the same choice with the cost written into the call — and `#[must_use]` on `to_owned` means the allocating one at least cannot be called for no reason. The nearer neighbour to the whole idea is `ASSIGNING <fs>` and `REFERENCE INTO`: not copying at all. Rust's version of that is `&` and `Cow`, and it beats both spellings on this page whenever the data does not have to be owned.
- **C++.** `dst = src` on a `std::string` already reuses the destination's capacity — copy-assignment is the reusing operation, and `std::string::assign` is its named form. So the C++ reader's surprise is not that `clone_into` exists; it is that the reusing behaviour is not what `=` does in Rust. Assignment here drops the old value and moves a new one in, because Rust has no copy-assignment operator to overload — `Clone` is a method, not `operator=`. `clone_from` and `clone_into` are the standard library handing back, as opt-in methods, the optimization C++ gets by default and cannot easily decline.

---

## The verified output

<!-- output:clone_into -->
*Verified output of [`clone_into.rs`](examples/clone_into.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Same result, different cost
   s.to_owned()                               alloc 1   realloc 0
   s.clone_into(&mut buf)  (buf has room)     alloc 0   realloc 0
   both produced "Ada Lovelace" — one of them bought a new buffer to do it

2. Where it pays: a loop with a reusable buffer
   4 rows, a fresh String each time           alloc 4   realloc 0
   4 rows, one buffer refilled                alloc 0   realloc 0
   the second loop allocates once — before the loop, not in it

3. It is not magic: the buffer has to be big enough
   clone_into an empty String                 alloc 1   realloc 0
   clone_into a 4-byte String                 alloc 0   realloc 1
   an empty target allocates; a too-small one grows. Neither is free.

4. The saving exists only where the impl overrides the default
   #[derive(Clone)] struct: clone_from        alloc 1   realloc 0
   hand-written clone_from: clone_from        alloc 0   realloc 0
   the derive was asked to do this in 2022 and libs-api said no —
   write clone_from by hand if you want it (rust#98374, wontfix)

5. On a slice of Strings, the INNER buffers are reused too
   Vec<String> to_owned                       alloc 5   realloc 0
   Vec<String> clone_into (roomy slots)       alloc 0   realloc 0
   Vec<String> clone_into (slots too small)   alloc 0   realloc 4
   Vec<String> clone_into (2 slots, 4 rows)   alloc 2   realloc 1
   to_owned bought 5 buffers: the Vec, then one String each.
   clone_into bought none — [T]'s impl clones into the slots in place.
   4-byte slots grow instead; rows past the end of the target are
   pushed as new Strings, and the Vec itself grows to hold them.

6. What you trade for it: the buffer keeps its high-water mark
   after a 300-byte row: capacity 300
   after a 4-byte row:   capacity 300 , len 4
   a long-lived buffer holds the largest row it ever saw.

7. The two spellings point opposite ways
   "source".clone_into(&mut dst) -> dst = "source"
   dst.clone_from(&other)        -> dst = "other"

8. The shape clippy actually flags — and whether it pays
   fixture default "default_value" is 13 bytes
   override        "override_value" is 14 bytes
   capacity the fixture handed over: 13
   assignment  (one byte short)               alloc 1   realloc 0
   clone_into  (one byte short)               alloc 0   realloc 1
   assignment  (default already fits)         alloc 1   realloc 0
   clone_into  (default already fits)         alloc 0   realloc 0
   one byte short, the rewrite trades an alloc for a realloc.
   give the default room and the same rewrite is free.

9. Which types the reuse reaches at all
   Box<String>   dst.clone_from(&src)         alloc 0   realloc 0
   Some(String)  dst.clone_from(&src)         alloc 0   realloc 0
   None          dst.clone_from(&src)         alloc 1   realloc 0
   u64 (Copy)    dst.clone_from(&src)         alloc 0   realloc 0
   HashSet<String>  clone_from                alloc 4   realloc 0
   BTreeSet<String> clone_from                alloc 5   realloc 0
   Box and Some forward to the String inside and reuse its 64 bytes.
   None has nothing to forward to, so it allocates. n = 9: a Copy
   type owns no buffer, so there was never anything to save.
   HashSet reuses its table and buys a String per element; BTreeSet's
   override forwards to BTreeMap, which has none, so it buys the lot.
```
<!-- /output -->

## Practice

**Four loops, and only two of them reuse the buffer.** Write a loop over five `&str` rows that writes each one into a `String` declared before the loop, four ways: `buf = row.to_owned()`, `row.clone_into(&mut buf)`, `buf.clear()` then `buf.push_str(row)`, and `buf = String::from(row)`. Predict how many heap allocations each performs before you count them, then count them with a wrapping global allocator.

Two of the four write through `let mut` and still allocate five times. Say what `mut` actually buys, and why it is not what the loop needed. Then predict the same number for a `#[derive(Clone)] struct Row { name: String }` whose `clone_from` is called with the destination already holding a 64-byte buffer, and say what you would have to write to make that one free.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:clone_into_kata -->
*[`clone_into_kata.rs`](examples/clone_into_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: which of these four loops actually reuses the buffer?
//!
//!   rustc --edition 2024 clone_into_kata.rs -o /tmp/cik && /tmp/cik

use std::alloc::{GlobalAlloc, Layout, System};
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

fn measure<T>(label: &str, work: impl FnOnce() -> T) -> T {
    let (a0, r0) = (ALLOCS.load(Relaxed), REALLOCS.load(Relaxed));
    let out = work();
    let (a1, r1) = (ALLOCS.load(Relaxed), REALLOCS.load(Relaxed));
    println!("   {label:<38} alloc {:<3} realloc {}", a1 - a0, r1 - r0);
    out
}

const ROWS: [&str; 5] = ["Ada", "Ben", "Cara", "Dev", "Eve"];

fn main() {
    println!("1. Four loops. Two of them reuse the buffer, two only look like it.");

    let mut a = String::with_capacity(16);
    measure("buf = row.to_owned()", || {
        for row in ROWS {
            a = row.to_owned();
            let _ = a.len();
        }
    });

    let mut b = String::with_capacity(16);
    measure("row.clone_into(&mut buf)", || {
        for row in ROWS {
            row.clone_into(&mut b);
            let _ = b.len();
        }
    });

    let mut c = String::with_capacity(16);
    measure("buf.clear(); buf.push_str(row)", || {
        for row in ROWS {
            c.clear();
            c.push_str(row);
            let _ = c.len();
        }
    });

    let mut d = String::with_capacity(16);
    measure("buf = String::from(row)", || {
        for row in ROWS {
            d = String::from(row);
            let _ = d.len();
        }
    });

    println!("   Assignment REPLACES the String — the old buffer is freed and a new");
    println!("   one bought. Only 2 and 3 write into the buffer that is already there,");
    println!("   and 3 is literally what str's clone_into impl does:");
    println!("       fn clone_into(&self, target: &mut String) {{");
    println!("           target.clear();");
    println!("           target.push_str(self);");
    println!("       }}");

    println!();
    println!("2. `mut` on the binding is not reuse of the buffer");
    // The thing that makes loop 1 look like it should be free is `let mut`.
    // But `mut` is about the NAME being rebindable; the heap buffer behind it
    // is a separate question, and `=` answers it the expensive way.
    println!("   loop 1 and loop 4 both wrote through `let mut`, and both paid.");
    println!("   the buffer is reused by the METHOD you call, not by the binding.");

    println!();
    println!("3. The trap: a derived Clone gives you none of this");
    #[derive(Clone)]
    struct Derived {
        name: String,
    }
    struct Tuned {
        name: String,
    }
    impl Clone for Tuned {
        fn clone(&self) -> Self {
            Tuned { name: self.name.clone() }
        }
        fn clone_from(&mut self, source: &Self) {
            self.name.clone_from(&source.name);
        }
    }

    let long = "a name long enough to live on the heap".to_string();
    let src_d = Derived { name: long.clone() };
    let mut dst_d = Derived { name: String::with_capacity(64) };
    measure("derived:      dst.clone_from(&src)", || dst_d.clone_from(&src_d));

    let src_t = Tuned { name: long.clone() };
    let mut dst_t = Tuned { name: String::with_capacity(64) };
    measure("hand-written: dst.clone_from(&src)", || dst_t.clone_from(&src_t));

    println!("   both landed the same text: {:?}", dst_d.name);
    println!("   #[derive(Clone)] emits `clone` and nothing else, so `clone_from`");
    println!("   falls back to the default `*self = source.clone()` and allocates.");
    println!("   The 64 bytes sitting in dst were never touched.");
}
```
<!-- /source -->

<!-- output:clone_into_kata -->
*Verified output of [`clone_into_kata.rs`](examples/clone_into_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Four loops. Two of them reuse the buffer, two only look like it.
   buf = row.to_owned()                   alloc 5   realloc 0
   row.clone_into(&mut buf)               alloc 0   realloc 0
   buf.clear(); buf.push_str(row)         alloc 0   realloc 0
   buf = String::from(row)                alloc 5   realloc 0
   Assignment REPLACES the String — the old buffer is freed and a new
   one bought. Only 2 and 3 write into the buffer that is already there,
   and 3 is literally what str's clone_into impl does:
       fn clone_into(&self, target: &mut String) {
           target.clear();
           target.push_str(self);
       }

2. `mut` on the binding is not reuse of the buffer
   loop 1 and loop 4 both wrote through `let mut`, and both paid.
   the buffer is reused by the METHOD you call, not by the binding.

3. The trap: a derived Clone gives you none of this
   derived:      dst.clone_from(&src)     alloc 1   realloc 0
   hand-written: dst.clone_from(&src)     alloc 0   realloc 0
   both landed the same text: "a name long enough to live on the heap"
   #[derive(Clone)] emits `clone` and nothing else, so `clone_from`
   falls back to the default `*self = source.clone()` and allocates.
   The 64 bytes sitting in dst were never touched.
```
<!-- /output -->

</details>

---

## See also

- [`ToOwned`](../to_owned/README.md) — the trait this method lives on, and why `str`'s owned twin is a different type
- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — the counting-allocator technique the run above uses, and what `with_capacity` buys
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md) — whose `clone_from` is the reason `clone_into` was stabilized at all
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — `clone_from` is the method this page's pair generalizes
- [A name is not a place](../../18_Ownership/a_name_is_not_a_place/README.md) — why rebinding a `mut` name does not reuse the buffer behind it
- [When `String` is too slow](../../14_Strings/when_string_is_too_slow/README.md) — the other allocation-shaped answers, including not owning the data

## Sources

The design argument is all in two threads and worth reading in order: [PR 41009 ↗](https://github.com/rust-lang/rust/pull/41009), whose own description lists the alternative (`owned_from` on a separate trait) and why it was not taken, and [tracking issue 41263 ↗](https://github.com/rust-lang/rust/issues/41263), five years of a known objection outliving two attempts to stabilize and losing to a use case in the end.

On the derive: [rust#98374 ↗](https://github.com/rust-lang/rust/issues/98374) (*derived `Clone` implementations don't utilize `clone_from`*), closed wontfix in 2024 after the libs-api decision quoted above. On the lint: [clippy#12444 ↗](https://github.com/rust-lang/rust-clippy/issues/12444), and [clippy#12779 ↗](https://github.com/rust-lang/rust-clippy/pull/12779), which moved it out of `perf` two releases after it shipped there.
