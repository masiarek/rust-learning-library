# `compare_exchange` and the retry loop

**Level:** 301 · deep dive

**One line:** `compare_exchange(current, new, …)` writes `new` only if the atomic still holds `current`, and reports what it found either way — the primitive most lock-free code is built from, and the reason the unit of correctness is the retry loop around it, not the call.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The signature: `compare_exchange(&self, current, new, success: Ordering, failure: Ordering) -> Result<T, T>` — `Ok(previous)` when it wrote, `Err(actual)` when it did not. Why does it hand back the value it found, and how does a loop use that `Err` value as its next `current`?
- Two orderings, one of them constrained: a failed exchange does not write, so the failure ordering may not be `Release` or `AcqRel`, and rustc's deny-by-default `invalid_atomic_ordering` lint refuses the program. The orderings themselves are [their own page](../atomic_orderings/README.md).
- `compare_exchange_weak` "is allowed to spuriously fail even when the comparison succeeds, which can result in more efficient code on some platforms". Which platforms, and why is a spurious failure harmless inside a loop and a bug outside one? Miri fails weak exchanges 80% of the time by default (`-Zmiri-compare-exchange-weak-failure-rate`), which is how a missing retry gets caught.
- The `fetch_*` family: on the integer atomics `fetch_add`, `fetch_sub`, `fetch_and`, `fetch_nand`, `fetch_or`, `fetch_xor`, `fetch_max`, `fetch_min`; on `AtomicBool` the bitwise ones plus `fetch_not`. When does one of them replace a CAS loop outright?
- The closure versions: `fetch_update` is marked "Deprecating in 1.99.0: renamed to `try_update`", and `try_update` and the infallible `update` are stable since 1.95.0. What may the closure do, given that it can run more than once?
- The ABA problem: a CAS that succeeds because the value went A → B → A while this thread was not looking. Why does a counter never care, and why does an `AtomicPtr` to a node that was freed and reallocated at the same address? What do hazard pointers and epoch-based reclamation (`crossbeam-epoch`) do about it?
- Spinning: [`std::hint::spin_loop` ↗](https://doc.rust-lang.org/std/hint/fn.spin_loop.html) inside the retry, and the contention level at which a `Mutex` beats the loop.
- `compare_and_swap` still exists and is "Deprecated since 1.50.0: Use `compare_exchange` or `compare_exchange_weak` instead" — what did it get wrong?

## The trap it exists for

`if a.load(…) == expected { a.store(new, …) }`. Each call is atomic and the pair is not, so two threads both see `expected` and both store — the lost update, written with atomics. The fix is one `compare_exchange` in a loop or one `fetch_*`, and the second trap is writing that `compare_exchange_weak` once, without the loop, on a machine where it happened never to fail spuriously during the tests.

## Where this sits

- [`RwLock` and atomics](../rwlock_and_atomics/README.md) covers when an atomic is the right tool at all, and why two atomics are not atomic together; [`atomic::Ordering`](../atomic_orderings/README.md) covers the ordering arguments. This page covers only the compare-and-exchange operation and the loop around it.

## See also

- [`RwLock` and atomics](../rwlock_and_atomics/README.md) — the one-word rule this page's loop lives inside
- [`atomic::Ordering`](../atomic_orderings/README.md) — the `success` and `failure` arguments
- [Testing concurrent code](../testing_concurrent_code/README.md) — Miri's forced spurious failures, and `loom`
- [False sharing](../false_sharing/README.md) — why a CAS loop on two hot atomics can be slower than it should be
- [Data races](../../31_C_and_Cpp/data_races/README.md) — the unsynchronised version the compiler refuses
- [Keeping every update ↗](https://masiarek.github.io/concurrency-learning-library/02_Shared_State/keeping_every_update/) — `fetch_add` beside a lock and a channel, in six languages

## If you are coming from another language

- **Go.** `atomic.Int64.CompareAndSwap(old, new)` returns a `bool` and not the value it found, has no weak variant and takes no ordering. [Atomic counters ↗](https://masiarek.github.io/go-learning-library/04_Sync/atomic_counters/) uses it for "first one wins", which is the one-shot case where no loop is needed: a failure means someone else won, and that is the answer.
- **C.** C11's `atomic_compare_exchange_strong(&obj, &expected, desired)` and `_weak` are the same pair, returning `bool` and writing the value it found back through `expected`. Rust returns that value in the `Err` instead of overwriting your variable, which makes the loop's data flow visible.
- **C++.** `std::atomic<T>::compare_exchange_weak(expected, desired)` and `compare_exchange_strong` take `expected` by reference and update it on failure, and accept one or two `memory_order` arguments. The standard permits the weak form to fail spuriously for the same reason Rust's docs give; the rest of the API maps across nearly name for name.
- **Java.** `AtomicInteger.compareAndSet` returns `boolean`; `compareAndExchange` (Java 9) returns the witness value, like Rust's `Err`; `weakCompareAndSetPlain` is the spurious-failure variant. `updateAndGet` with a lambda is `update`, with the same warning that the function may be applied more than once.
- Concepts: [compare-and-swap ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/lock_free/compare_and_swap/) · [ABA problem ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/hazards/aba_problem/) · [atomic variable ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/lock_free/atomic_variable/) · [lock-free ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/lock_free/lock_free/) · [hazard pointers ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/lock_free/hazard_pointers/)

## Po polsku

**Porównaj i zamień** (*compare-and-swap*, CAS) zapisuje nową wartość tylko wtedy, gdy zmienna atomowa wciąż ma wartość oczekiwaną — w Ruscie to `compare_exchange`, które zwraca `Ok(poprzednia)` albo `Err(zastana)`. Jednostką poprawności jest **pętla ponawiania** (*retry loop*), a nie pojedyncze wywołanie: wariant `compare_exchange_weak` może zawieść pozornie (*spurious failure*) nawet przy zgodnej wartości. Problem ABA to CAS, który się udaje, bo wartość zdążyła przejść A → B → A — dla licznika bez znaczenia, dla wskaźnika na zwolniony i ponownie przydzielony węzeł katastrofalny.

**Szukaj po polsku:** porównaj i zamień · problem ABA · struktury bez blokad · `rust compare_exchange_weak loop` · `rust fetch_update try_update`
