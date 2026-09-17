# Concurrency or parallelism

**Level:** 201 · working knowledge

**One line:** Concurrency is how a program is structured — several tasks in progress at once, possibly on one core — and parallelism is whether they execute at the same instant; the first decides whether the answer is right, the second only how soon it arrives.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The two words kept apart: two threads on a one-core machine are concurrent and never parallel, and a vectorised loop is parallel with no concurrency anywhere in the source. Which of the two does [`thread::spawn` ↗](https://doc.rust-lang.org/std/thread/fn.spawn.html) give you, and what decides whether you also get the other?
- Correctness belongs to concurrency. A lost update needs two tasks interleaving, not two cores — does the race in [Data races](../../31_C_and_Cpp/data_races/README.md) exist on a single core, and why does parallelism only make it show up more often?
- Performance belongs to parallelism, and Amdahl's law caps it: if 10% of the work stays serial, no number of cores gets past a 10× speedup. The finished page measures speedup against thread count for one real split and shows where the curve flattens.
- [`std::thread::available_parallelism` ↗](https://doc.rust-lang.org/std/thread/fn.available_parallelism.html) returns `io::Result<NonZero<usize>>`, an estimate rather than a core count: its docs say container limits and affinity masks can make it diverge, and that it is recomputed on every call, so it does not belong in hot code. What does it print on this machine, and under a CPU-limited container?
- I/O-bound against CPU-bound: more threads than cores helps a program that waits on the network and slows one that adds numbers. Where is the crossover for each, and what is oversubscription costing in the second case?
- Concurrency without extra threads is the async model — named here, taught in the async section, not on this page.
- A speedup is measured, not assumed: release build, several runs, the median — the discipline from [Timing a block](../../33_Time_and_Benchmarking/timing_a_block/README.md).

## The trap it exists for

Spawning eight threads to make a program faster when its time goes to one lock, one file or one serial step. The wall time stays the same, the program gets harder to reason about, and the conclusion drawn is that threads do not help — when the measurement that would have predicted it is one division.

## Where this sits

- [Spawning a thread](../spawning_a_thread/README.md) and [Channels](../channels/README.md) cover the mechanics; [`rayon`](../rayon/README.md) and [Worker pools](../worker_pools/README.md) cover the tools. This page covers only the vocabulary and the arithmetic of speedup.

## See also

- [Spawning a thread](../spawning_a_thread/README.md) — the concurrency primitive this page puts a name on
- [`rayon`](../rayon/README.md) — data parallelism as a library, and when it is slower
- [Worker pools](../worker_pools/README.md) — a fixed number of threads, sized by the question above
- [False sharing](../false_sharing/README.md) — one reason the measured speedup is lower than the core count
- [Compile times](../../05_Tooling/compile_times/README.md) — Amdahl's law already at work in this library, on a build
- [Data races](../../31_C_and_Cpp/data_races/README.md) — the concurrency bug that needs no second core
- [Letting the compiler reorder a float sum](../../19_Numbers/letting_the_compiler_reorder/README.md) — parallelism inside one loop, and the permission it needs

## If you are coming from another language

- **Python.** The distinction arrives already sharpened: in the default CPython build, threads give you concurrency and the GIL takes away parallelism for CPU-bound Python code, so `multiprocessing` is the parallel tool and `threading` the concurrent one. Rust threads are both at once, which removes the workaround and the accidental cover: in the Concurrency library's runs of [Is `total += n` safe on two threads? ↗](https://masiarek.github.io/concurrency-learning-library/02_Shared_State/the_lost_update/), a bare `total += 1` under the GIL lost nothing and `total = total + one()` lost tens or hundreds of thousands, while in Rust the unsynchronised version does not compile at all. `os.cpu_count()` is the counterpart of `available_parallelism`, with the same caveat about containers.
- **Go.** Go's own vocabulary for this is Rob Pike's talk [*Concurrency is not parallelism* ↗](https://go.dev/blog/waza-talk): goroutines are the structure, `GOMAXPROCS` decides how much of it runs at once. Rust has no runtime between the two, so a spawned thread is an OS thread and the parallelism is whatever the OS gives. [Goroutines are cheap ↗](https://masiarek.github.io/go-learning-library/01_Goroutines/goroutines_are_cheap/) shows the cost difference that explains why Go programs spawn freely and Rust programs size a pool.
- **Java.** `Runtime.getRuntime().availableProcessors()` and the common `ForkJoinPool` are the same two ideas — a number to size by and a pool to run on. What carries over unchanged is Amdahl; what changes is that a Java parallel stream will happily share a mutable collection, and the Rust equivalent does not compile.
- **ABAP.** Parallel processing through asynchronous RFC into a server group is parallelism with the ceiling set by Basis: the number of free dialog work processes, not the number of cores, bounds the speedup. A step every task has to pass through one at a time — an enqueue lock on the same object, say — is the serial fraction, which is Amdahl's law in an SAP system.
- The [Splitting a sum across workers ↗](https://masiarek.github.io/concurrency-learning-library/07_Parallelism/splitting_a_sum_across_workers/) lesson runs the parallel version in six languages; the concept pages for [concurrency ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/foundations/concurrency/), [parallelism ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/foundations/parallelism/), [speedup and Amdahl's law ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/foundations/speedup_and_amdahls_law/) and [I/O-bound and CPU-bound ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/foundations/io_bound_and_cpu_bound/) carry the definitions across languages.

## Po polsku

Po polsku oba słowa brzmią podobnie — **współbieżność** (*concurrency*) i **równoległość** (*parallelism*) — i w potocznej mowie bywają zamienne, co na tej stronie jest błędem. Współbieżność to struktura programu: kilka zadań „w toku”, nawet na jednym rdzeniu; równoległość to wykonanie dokładnie w tej samej chwili. Wyścig danych (*data race*) jest problemem współbieżności i istnieje także na jednym rdzeniu, a przyspieszenie jest sprawą równoległości i ogranicza je **prawo Amdahla**: jeśli 10% pracy musi zostać wykonane sekwencyjnie, żadna liczba rdzeni nie da więcej niż dziesięciokrotnego przyspieszenia.

**Szukaj po polsku:** współbieżność a równoległość · prawo Amdahla · zadania ograniczone przez wejście-wyjście · `concurrency vs parallelism rust` · `rust available_parallelism container`
