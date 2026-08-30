# Sharing across threads: `Arc`

**Level:** 201 · working knowledge

**One line:** `Arc<T>` is `Rc<T>` with an atomic counter, and the difference is not a performance note — it is the reason one of them compiles across a thread boundary and the other does not.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The refusal first: move an `Rc` into `thread::spawn` and read the error — the bound is `Send`, the type is not, and the compiler names the marker rather than the mistake
- What "atomic" is paying for — two threads incrementing a plain integer can lose an increment and free a live value; the atomic count cannot, and that is the whole of the difference
- `Arc<Mutex<T>>` as the standard shape for shared *mutable* state: `Arc` grants the ownership, `Mutex` grants the write, and neither substitutes for the other
- `Arc::clone` per thread before the spawn, since the closure needs its own owner — the idiom that makes the clone count equal the thread count
- What it still does not buy: `Arc<T>` is a shared reference, so `T` must supply its own interior mutability, and a cycle leaks here exactly as it does with `Rc`
- When to skip it entirely — scoped threads borrow, so a value that outlives the scope needs no counting at all

## The trap it exists for

`Arc` reads like the safe default and `Rc` like the optimisation, so the habit is to reach for `Arc` everywhere and stop thinking. The atomic operations are not free, and single-threaded code paying for them is invisible: nothing warns, nothing fails, and the profile is the only place it shows.

## See also

- [`Rc`: the clone that copies a pointer](../reference_counting/README.md) — the single-threaded original, and the counter this page makes atomic
- [Marker traits](../../12_Traits/marker_traits/README.md) — `Send` itself: `assert_send::<Arc<i32>>()` compiles and `assert_send::<Rc<i32>>()` does not, on two values of identical size
- [Mutex poisoning](../../09_Advanced/mutex_poisoning/README.md) — what happens to `Arc<Mutex<T>>` when a thread panics while holding the lock
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — the trait both counters are implementing
- [Ownership and moves](../ownership_and_moves/README.md) — the rule a thread boundary enforces most visibly
