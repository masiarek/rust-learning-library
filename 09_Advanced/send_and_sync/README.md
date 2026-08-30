# `Send` and `Sync`

**Level:** 301 · deep dive

**One line:** Two traits with no methods decide what may cross a thread boundary — `Send` means the value may **move** to another thread, `Sync` means `&T` may be **shared** with one — and you almost never write either, because the compiler derives them structurally and the interesting cases are the types that lack them.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The definitions in one line each, and the relationship: `T: Sync` if and only if `&T: Send`
- **Auto traits**: implemented automatically for any type whose fields all have them, which is why nobody writes `impl Send`. The opt-out is negative, and it is `PhantomData` or an `unsafe impl`
- The list worth memorising, because it is short — **not `Send`**: `Rc`, and raw pointers. **Not `Sync`**: `Rc`, `Cell`, `RefCell`, and `&mut` to anything not `Sync`
- Why `Rc` is neither and `Arc` is both: one number updated non-atomically, and the data race that produces
- Where the error actually appears — on `thread::spawn`, as a `T: Send` bound that is not satisfied, naming a type three layers inside the closure's capture
- `Mutex<T>: Sync` whenever `T: Send`, which is the sentence that explains why wrapping in a `Mutex` fixes a `Sync` error
- The rare legitimate `unsafe impl Send`, and the proof obligation you are signing

## The trap it exists for

The compiler reports the failure at the `spawn`, but the cause is a field you cannot see — a `Rc` buried inside a struct inside a closure capture. Reading the error means reading it backwards, from the bound to the type that broke it, and the type named is usually not the one you passed.

## See also

- [Marker traits](../../12_Traits/marker_traits/README.md) — the category these two belong to, and `PhantomData`
- [Sharing across threads: `Arc`](../../18_Ownership/sharing_across_threads/README.md) — the `Rc`/`Arc` swap this page explains
- [Interior mutability](../interior_mutability/README.md) — `Cell` and `RefCell`, the two everyday types that are not `Sync`
- [Lock poisoning](../mutex_poisoning/README.md) — the `Mutex` whose `Sync` is doing the work
- [`Send` ↗](https://doc.rust-lang.org/std/marker/trait.Send.html) · [Comprehensive Rust: `Send` and `Sync` ↗](https://google.github.io/comprehensive-rust/concurrency/send-sync.html)
