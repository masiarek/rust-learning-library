# `RwLock` and atomics

**Level:** 301 · deep dive

**One line:** Between a `Mutex` and a channel sit two more answers — an `RwLock`, which lets many readers in at once but only one writer, and an atomic, which is a single value updated without any lock at all — and picking between the three is a question about your access pattern, not about performance folklore.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `RwLock`: `read()` and `write()`, both returning guards, both returning a `Result` for the same poisoning reason a `Mutex` does
- When `RwLock` actually wins — many readers, rare writes, and a critical section long enough for the extra bookkeeping to be worth it. Under a short section a `Mutex` is often faster, which is the opposite of the folklore
- Writer starvation, and the fact that the standard library does not promise a fairness policy — it is the OS primitive's policy
- `AtomicUsize`, `AtomicBool` and friends: `load`, `store`, `fetch_add`, `compare_exchange`
- `Ordering` — `Relaxed`, `Acquire`, `Release`, `SeqCst` — as the question *what else is this operation allowed to reorder around?*, with `SeqCst` as the safe default and `Relaxed` as fine for a counter nobody synchronises on
- Why an atomic is not "a faster Mutex": it protects **one word**, and two atomics updated together are not atomic together
- The `Arc<AtomicUsize>` counter as the worked example, against the `Arc<Mutex<usize>>` version

## The trap it exists for

Reading two atomics and assuming the pair is consistent. Each load is atomic; the pair is not, so a reader can see the new value of one and the old value of the other. The moment two values must agree, you wanted a lock — and the bug this produces is intermittent, load-dependent, and does not reproduce under a debugger.

## See also

- [Lock poisoning](../mutex_poisoning/README.md) — the `Mutex` these two sit either side of, and the `Result` both share
- [`Send` and `Sync`](../send_and_sync/README.md) — why these types are the ones allowed to cross a thread boundary
- [Spawning a thread](../spawning_a_thread/README.md) · [Channels](../channels/README.md) — the "share nothing" alternative to all three
- [Sharing across threads: `Arc`](../../18_Ownership/sharing_across_threads/README.md) — the pointer every one of these lives behind
- [`std::sync::atomic` ↗](https://doc.rust-lang.org/std/sync/atomic/) · [Comprehensive Rust: Shared State ↗](https://google.github.io/comprehensive-rust/concurrency/shared-state.html)
