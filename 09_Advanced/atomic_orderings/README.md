# `atomic::Ordering`

**Level:** 301 · deep dive

**One line:** The `Ordering` every atomic call takes is `std::sync::atomic::Ordering`, not the `std::cmp::Ordering` you sort with, and each variant is a promise about which other operations this one is synchronised with — `SeqCst` promises the most, which makes it the hardest to review rather than the safest.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Two enums, one name. `use std::cmp::Ordering;` and `use std::sync::atomic::Ordering;` in one module is `E0252` ("the name `Ordering` is defined multiple times"); importing only the atomic one and matching on `Ordering::Less` is `E0599`. Which spelling avoids both — `atomic::Ordering::SeqCst`, or a rename with `as`?
- The variants in Rust 1.98: [`Relaxed`, `Release`, `Acquire`, `AcqRel`, `SeqCst` ↗](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html), on a `#[non_exhaustive]` enum whose docs map each one to a C++20 `memory_order`. What does each promise for a load, a store, and an operation that does both?
- The combinations rustc refuses: its deny-by-default `invalid_atomic_ordering` lint rejects `Acquire` or `AcqRel` on a store ("atomic stores cannot have `Acquire` or `AcqRel` ordering") and `Release` or `AcqRel` as a `compare_exchange` failure ordering.
- Happens-before as the working question: why `Relaxed` is correct for a statistics counter nobody makes a decision from, and what `Release` on the store paired with `Acquire` on the load buys a spin lock or a once-initialised flag.
- Why `SeqCst` is not "safer". The Nomicon calls sequential consistency "definitely the right choice if you're not confident"; [Rust Atomics and Locks ↗](https://marabos.nl/atomics/memory-ordering.html#common-misconceptions) calls it "a warning sign", because it tells a reader the operation depends on the order of every other `SeqCst` operation in the program. The finished page shows a program that uses `SeqCst` everywhere and is still wrong, and says which advice this library follows.
- [`std::sync::atomic::fence` ↗](https://doc.rust-lang.org/std/sync/atomic/fn.fence.html) and `compiler_fence`, stable in 1.98: synchronisation with no value attached, and the signal-handler case the second exists for. The unstable book notes that ThreadSanitizer "does not support atomic fences".
- Does the ordering change the generated code at all on x86-64, a strongly ordered platform, and does it on aarch64?

## The trap it exists for

`SeqCst` "to be safe", and the question considered closed. No ordering can repair a load, a decision and a store made in three calls, and every `SeqCst` in the file now asks the reviewer to reason about the whole program's order. The mirror image is `Relaxed` "because it is only a flag" on the flag another thread reads to decide whether to proceed.

## Where this sits

- [`RwLock` and atomics](../rwlock_and_atomics/README.md) introduces atomics and the one-word rule; [`compare_exchange`](../compare_and_exchange/README.md) covers the operation these orderings are passed to; [The comparison traits](../../12_Traits/comparison_traits/README.md) owns the other `Ordering`. This page covers only the memory-ordering argument.

## See also

- [`RwLock` and atomics](../rwlock_and_atomics/README.md) — where atomics enter, and the default this page questions
- [`compare_exchange` and the retry loop](../compare_and_exchange/README.md) — the call with two orderings
- [Catching a signal](../catching_a_signal/README.md) — a `SeqCst` store inside a signal handler, the case `compiler_fence` is documented for
- [The comparison traits](../../12_Traits/comparison_traits/README.md) — `std::cmp::Ordering`, the enum with the same name
- [Bringing names in with `use`](../../27_Modules/the_use_declaration/README.md) — renaming one of the two imports
- [Testing concurrent code](../testing_concurrent_code/README.md) — Miri's weak-memory emulation and `loom`'s C11 memory model, the tools that exercise an ordering choice

## If you are coming from another language

- **C++.** The same model under the same names: `std::memory_order_relaxed`, `_acquire`, `_release`, `_seq_cst`, and every `std::atomic` member function defaults its order argument to `memory_order_seq_cst`. Rust has no default, so the choice is written at every call site — which is where this page's argument about reviewing it comes from.
- **Go.** `sync/atomic` takes no ordering. The [Go memory model ↗](https://go.dev/ref/mem) says atomic operations "behave as though executed in some sequentially consistent order", and names C++'s sequentially consistent atomics and Java's `volatile` as the same semantics — so every Go atomic is `SeqCst`, and there is nothing to tune or to misjudge. [Atomic counters ↗](https://masiarek.github.io/go-learning-library/04_Sync/atomic_counters/) is the Go page.
- **Java.** A `volatile` field and the `java.util.concurrent.atomic` classes give the sequentially consistent behaviour by default; `VarHandle`'s plain, opaque, acquire/release and volatile access modes are Java's version of choosing a weaker ordering explicitly.
- **C.** `<stdatomic.h>` has `memory_order_relaxed` through `memory_order_seq_cst` and the `_explicit` function variants that take one; the plain functions use sequential consistency.
- Concepts: [weak memory model ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/hazards/weak_memory_model/) · [happens-before ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/lock_free/happens_before/) · [sequential consistency ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/safety_in_languages/sequential_consistency/)

## Po polsku

W Ruscie są dwa typy `Ordering` i łatwo je pomylić: `std::cmp::Ordering` służy do porównań i sortowania, a `std::sync::atomic::Ordering` określa **uporządkowanie pamięci** (*memory ordering*) przy operacjach atomowych — zaimportowanie obu naraz kończy się błędem `E0252`. Każdy wariant to obietnica o tym, z jakimi innymi operacjami ta operacja jest zsynchronizowana (relacja *happens-before*). `SeqCst` obiecuje najwięcej, ale nie jest „bezpieczniejszy”: nie naprawi algorytmu, który odczytuje, decyduje i zapisuje w trzech osobnych wywołaniach, a recenzentowi każe rozumować o kolejności wszystkich operacji w programie.

**Szukaj po polsku:** uporządkowanie pamięci · spójność sekwencyjna · słaby model pamięci · `rust atomic ordering SeqCst vs Relaxed` · `rust cmp Ordering atomic Ordering conflict`
