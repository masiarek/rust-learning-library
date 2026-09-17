# False sharing

**Level:** 301 · deep dive

**One line:** Two threads writing two different atomics can slow each other down when both values sit on one cache line — nothing is shared in the source, the CPU cache shares it anyway, and the fix is a layout change (`#[repr(align(64))]`, `CachePadded`) rather than a change to the code.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The cache line: `sysctl hw.cachelinesize` prints `64` on this machine (macOS); what does Linux report, and where does it come from?
- The experiment: two threads, each calling `fetch_add` a hundred million times on its own `AtomicU64`, with the two counters adjacent in one struct against padded apart. Wall time for each layout, release build, several runs, median reported.
- `#[repr(align(64))]` on a wrapper raises its alignment and size to 64, so consecutive wrappers in an array start 64 bytes apart. What does that cost in memory for a `Vec` of per-thread counters?
- [`crossbeam_utils::CachePadded` ↗](https://docs.rs/crossbeam-utils/latest/crossbeam_utils/struct.CachePadded.html) assumes 128-byte cache lines on x86-64, aarch64 and powerpc64 and 64 on most other targets, because "spatial prefetcher is pulling pairs of 64-byte cache lines at a time". Does 64 or 128 make the difference in the experiment above?
- The memory operations underneath, from *Rust for Rustaceans* ch. 10 and [Rust Atomics and Locks ch. 7 ↗](https://marabos.nl/atomics/hardware.html#cache-coherence): a write invalidates the line in every other core's cache, so the next access there has to fetch it again. Is a `load` on the neighbour affected as much as a `store`?
- Where it hides in ordinary code: per-thread counters in one `Vec<AtomicU64>`, a hot counter declared next to a `Mutex`, the fields of a struct shared through `Arc`. `repr(Rust)` may reorder fields — can the compiler move two hot fields together without being asked?
- How to see it without guessing: which profiler counters point at a contended cache line rather than a contended lock?

## The trap it exists for

Giving every thread its own counter to remove contention, storing the counters side by side in one array, and measuring no speedup. The locks are gone and the threads still interfere, through a cache line the source code never mentions.

## Where this sits

- [Concurrency or parallelism](../concurrency_or_parallelism/README.md) covers why a speedup falls short of the core count in general; [Type layout](../type_layout/README.md) covers `repr(align)` as a layout rule; [`RwLock` and atomics](../rwlock_and_atomics/README.md) covers contention on a lock. This page covers only interference through the cache.

## See also

- [Type layout](../type_layout/README.md) — `repr(align(N))`, and why `repr(Rust)` promises no field order
- [`compare_exchange` and the retry loop](../compare_and_exchange/README.md) — lock-free code, which is where false sharing costs most
- [Concurrency or parallelism](../concurrency_or_parallelism/README.md) — the speedup this eats into
- [`RwLock` and atomics](../rwlock_and_atomics/README.md) — `AtomicU64` counters, the usual victims
- [Timing a block](../../33_Time_and_Benchmarking/timing_a_block/README.md) · [`black_box` is a hint](../../33_Time_and_Benchmarking/black_box_is_a_hint/README.md) — keeping the benchmark honest

## If you are coming from another language

- **C++.** `std::hardware_destructive_interference_size` (C++17) is a named constant for exactly this, used with `alignas` on the struct — the counterpart of `#[repr(align(64))]`. The C++ library's [The cache changes the answer ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/the_cache_changes_the_answer/) measures the single-threaded half of the story: the same loop at different speeds depending on which cache level its data fits in.
- **Java.** The JDK marks its own hot fields with `@Contended` (in `jdk.internal.vm.annotation`), including the cells `LongAdder` spreads one counter across; outside the JDK the annotation is ignored unless the JVM runs with `-XX:-RestrictContended`. Java cannot control field layout otherwise, which is the part Rust hands to you.
- **Go.** `golang.org/x/sys/cpu` provides `CacheLinePad`, a struct one cache line long that you place between hot fields. The gc compiler lays fields out in declaration order, so the padding stays where you put it — in Rust only `repr(C)` gives that promise.
- **C.** `_Alignas(64)` on a member, or `aligned_alloc(64, …)` for per-thread blocks; the effect and the experiment are identical, and `pthread` code hits it through arrays of per-thread structs.
- Concepts: [false sharing ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/hazards/false_sharing/) · [cache coherence ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/parallelism/cache_coherence/) · [contention ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/hazards/contention/)

## Po polsku

**Fałszywe współdzielenie** (*false sharing*) to spowolnienie, którego nie widać w kodzie: dwa wątki zapisują dwie różne zmienne atomowe, ale obie leżą w tej samej **linii pamięci podręcznej** (*cache line*, zwykle 64 bajty), więc każdy zapis unieważnia tę linię w pamięci podręcznej drugiego rdzenia. Blokad nie ma, a wątki i tak sobie przeszkadzają. Lekarstwem jest zmiana układu w pamięci — `#[repr(align(64))]` albo `CachePadded` z `crossbeam-utils`, który na x86-64 zakłada nawet 128 bajtów.

**Szukaj po polsku:** fałszywe współdzielenie · linia pamięci podręcznej · wyrównanie struktury · `rust false sharing repr align 64` · `crossbeam CachePadded`
