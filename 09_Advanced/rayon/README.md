# `rayon`

**Level:** 201 · working knowledge

**One line:** [`rayon` ↗](https://docs.rs/rayon) turns `iter()` into `par_iter()` and runs the same chain on a pool of worker threads that steal work from each other — a one-word change with a contract behind it, and on a small input a slower program.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- It is a crate, not std: `cargo add rayon`, then `use rayon::prelude::*`. What does the compiler say about `par_iter` when the prelude import is missing, and why is that [a trait not in scope](../../12_Traits/trait_in_scope/README.md) rather than a missing method?
- `par_iter`, `par_iter_mut`, `into_par_iter`, and the closure bounds: [`ParallelIterator::for_each` ↗](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.for_each) takes `OP: Fn(Self::Item) + Sync + Send`. Why `Fn` and not `FnMut`, and what error does a closure that increments a captured counter produce?
- [`rayon::join(a, b)` ↗](https://docs.rs/rayon/latest/rayon/fn.join.html) "potentially runs them in parallel": inside the pool the calling thread runs `a` itself and advertises `b` for another thread to steal. How is work stealing different from every worker taking jobs off one shared queue, as in [Worker pools](../worker_pools/README.md)?
- When it is slower: a sum over a thousand integers, a closure that takes a `Mutex`, a chain whose per-item work is smaller than the cost of splitting. The finished page measures the input size where `par_iter` starts to win.
- Order: does `collect::<Vec<_>>()` keep input order, does `for_each` with a `println!` inside, and why does a parallel `f64` sum regroup the additions so the total can differ from the serial one?
- Pool size: `ThreadPoolBuilder::num_threads` documents that the default comes from the `RAYON_NUM_THREADS` environment variable if set, otherwise the number of logical CPUs. What does that do inside a container limited to two CPUs?
- A panic inside one item of a `par_iter`: what happens to the other items, and what does the caller see?
- `thread::scope` is the std way to split one slice across a few threads with borrows — when is that enough, and when is `rayon` worth the dependency?

## The trap it exists for

Putting `par_` in front of a loop whose per-item work is a few nanoseconds, measuring a slowdown, and concluding parallelism does not pay. Or putting it in front of a loop whose closure locks one `Mutex` — the threads queue on the lock, and the program is serial with overhead added.

## Where this sits

- [Concurrency or parallelism](../concurrency_or_parallelism/README.md) covers the vocabulary and Amdahl's law; [Worker pools](../worker_pools/README.md) builds a pool by hand; [Spawning a thread](../spawning_a_thread/README.md) covers `thread::scope`. The [Iterators section](../../24_Iterators/README.md) lists parallel iterators as a gap and points here. This page covers only data parallelism with `rayon`.

## See also

- [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) — `cargo add rayon`, and what the version string in the manifest permits
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — the sequential chain `par_iter` mirrors
- [The three closure traits](../../23_Closures/three_closure_traits/README.md) — why `Fn` is the bound a parallel closure gets
- [Letting the compiler reorder a float sum](../../19_Numbers/letting_the_compiler_reorder/README.md) — the same regrouping, done by the optimizer instead of by threads
- [Spawning a thread](../spawning_a_thread/README.md) — `thread::scope`, the std alternative for a fixed split
- [Concurrency or parallelism](../concurrency_or_parallelism/README.md) — how much speedup to expect before measuring
- [Timing a block](../../33_Time_and_Benchmarking/timing_a_block/README.md) — measuring the crossover honestly

## If you are coming from another language

- **Java.** `list.parallelStream()` is the same one-word change on the same kind of machinery: the common `ForkJoinPool` is a work-stealing pool, as `rayon`'s is. [Splitting a sum across workers ↗](https://masiarek.github.io/concurrency-learning-library/07_Parallelism/splitting_a_sum_across_workers/) shows a parallel `reduce` whose float total changed with the size of that pool. What Rust adds is the `Sync + Send` bound — a parallel stream that mutates a shared `ArrayList` compiles in Java, and its `rayon` twin does not.
- **Python.** `multiprocessing.Pool.map` is the nearest drop-in, and it pays for its parallelism by pickling each item into another process. `rayon` shares the slice by reference across threads, so a large input costs no copying. `concurrent.futures.ThreadPoolExecutor.map` has the shape but, in the default CPython build, not the parallelism for CPU-bound work.
- **C++.** `std::reduce(std::execution::par, …)` and `std::for_each` with an execution policy are the standard-library version of a parallel iterator; the policy is a request, and data races inside the callable are your problem. The same splitting lesson records libc++ and libstdc++ grouping a float sum differently.
- **Go.** There is no parallel iterator in the standard library; the idiom is a `sync.WaitGroup` fan-out over chunks, which is closer to `thread::scope` than to `rayon`. [Fan-out, fan-in ↗](https://masiarek.github.io/go-learning-library/06_Patterns/fan_out_fan_in/) shows it, including the order it loses.
- Concepts: [data parallelism ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/parallelism/data_parallelism/) · [work stealing ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/scheduling/work_stealing/) · [fork–join ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/parallelism/fork_join/) · [parallel iterators ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/parallelism/parallel_iterators/)

## Po polsku

`rayon` to zewnętrzna skrzynka (*crate*), nie część biblioteki standardowej: zamienia `iter()` na `par_iter()` i wykonuje ten sam łańcuch na puli wątków, które **podkradają sobie pracę** (*work stealing*). Zmiana jednego słowa ma jednak kontrakt — domknięcie musi być `Fn + Sync + Send` — i nie zawsze przyspiesza: przy małej pracy na element koszt podziału przewyższa zysk, a domknięcie biorące `Mutex` robi z równoległej pętli sekwencyjną. Suma liczb zmiennoprzecinkowych liczona równolegle grupuje dodawania inaczej, więc wynik może się różnić od sekwencyjnego.

**Szukaj po polsku:** równoległość danych w Ruscie · podkradanie pracy · `rust rayon par_iter slower` · `rayon join work stealing` · `RAYON_NUM_THREADS`
