# Testing concurrent code

**Level:** 301 · deep dive

**One line:** A concurrent test that passes has shown one schedule out of a great many, so the tools worth using change the schedule on purpose — a stress loop runs many, `loom` explores them systematically, Miri checks each run for data races, and ThreadSanitizer instruments the real binary.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why green proves little: the interleaving a bug needs may never happen on a quiet CI runner. The Go library's [race detector page ↗](https://masiarek.github.io/go-learning-library/07_Testing_Concurrent_Code/the_race_detector/) shows the same instrumented binary exiting 0 on the run where the second goroutine stayed away.
- What safe Rust already rules out — the unsynchronised shared write is a compile error — and so which bugs the tests are left to find: deadlocks, lost updates across two locks, check-then-act across two messages, and anything behind an `unsafe impl Send`.
- Stress tests: the operation run thousands of times across [`available_parallelism()`](../concurrency_or_parallelism/README.md) threads, released together by a [`Barrier` ↗](https://doc.rust-lang.org/std/sync/struct.Barrier.html). What does that raise the odds of, and what can it still never promise?
- [`loom` ↗](https://docs.rs/loom): `loom::model(|| …)` runs a test under the interleavings valid "under the C11 memory model", with state reduction to keep the count down. The crate's docs switch `std::sync` for `loom::sync` behind `#[cfg(loom)]` and run with `RUSTFLAGS="--cfg loom"`. What can it not model, and how large a test does it finish?
- Miri: `cargo +nightly miri test` reports "Data race detected between …" as undefined behaviour, emulates some weak-memory effects, and fails `compare_exchange_weak` most of the time on purpose. `-Zmiri-seed` and `-Zmiri-many-seeds` change the schedule — how many schedules is that, against `loom`'s?
- ThreadSanitizer: `RUSTFLAGS=-Zsanitizer=thread` with `-Zbuild-std` and an explicit `--target`, nightly only. The unstable book lists `x86_64-apple-darwin` and `x86_64-unknown-linux-gnu` among the supported targets, warns that uninstrumented code can produce false positives, and says it "does not support atomic fences".
- The harness itself: `cargo test` runs tests on several threads in one process, `--test-threads=1` serialises them, and [nextest](../../05_Tooling/nextest/README.md) gives each test a process. Which of those hides a concurrency bug, and which exposes one?
- Deterministic tests: separating the logic from the spawning so the interesting part runs on one thread. Go's `testing/synctest` makes time virtual inside a bubble — what, if anything, is the Rust counterpart?

## The trap it exists for

`thread::sleep` added until a flaky test goes green. The sleep moves the program onto a schedule where the bug does not fire, the bug is still in the code, and the test now takes longer and proves less.

## Where this sits

- [Data races](../../31_C_and_Cpp/data_races/README.md) covers the race safe Rust refuses and ThreadSanitizer on the C version; [Miri](../miri/README.md) covers Miri as an undefined-behaviour checker in general; [The Testing section](../../28_Testing/README.md) covers the harness. This page covers only strategies for tests of concurrent code.

## See also

- [Data races](../../31_C_and_Cpp/data_races/README.md) — ThreadSanitizer on C, and the lock that protects each access but not the decision
- [Miri](../miri/README.md) — the interpreter, its flags, and what it cannot see
- [What a test asserts](../../28_Testing/what_a_test_asserts/README.md) — a test is a function that panics, which is what a stress loop repeats
- [cargo-nextest](../../05_Tooling/nextest/README.md) — one process per test
- [`rustup default nightly`](../../05_Tooling/nightly/README.md) — Miri and the sanitizers both live there
- [`compare_exchange` and the retry loop](../compare_and_exchange/README.md) — the loop Miri's forced failures test
- [`atomic::Ordering`](../atomic_orderings/README.md) — the choice `loom` and Miri's weak-memory emulation exercise

## If you are coming from another language

- **Go.** `go test -race` is ThreadSanitizer built into the toolchain with no nightly and no rebuilt standard library, which is the gap between Go's experience and Rust's today. [The race detector ↗](https://masiarek.github.io/go-learning-library/07_Testing_Concurrent_Code/the_race_detector/) and [`synctest` makes time virtual ↗](https://masiarek.github.io/go-learning-library/07_Testing_Concurrent_Code/synctest_makes_time_virtual/) are the two Go pages; Rust's compiler removes the first tool's most common finding before any test runs.
- **C and C++.** `-fsanitize=thread` is the same runtime Rust's `-Zsanitizer=thread` uses. The [C and C++ section](../../31_C_and_Cpp/README.md) records it catching the data race and printing nothing on the forgotten unlock, whose process never exited to report.
- **Java.** OpenJDK's jcstress harness runs small concurrent actors many times and tabulates the outcomes it saw — a stress test with the results counted, which is the shape this page's stress section should copy. There is no Miri or `loom` counterpart in the JDK.
- Concepts: [model checking ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/testing_and_tools/model_checking/) · [race detector ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/testing_and_tools/race_detector/) · [stress testing ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/testing_and_tools/stress_testing/) · [deterministic testing ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/testing_and_tools/deterministic_testing/) · [heisenbug ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/hazards/heisenbug/)

## Po polsku

Zielony test współbieżny pokazał jeden z bardzo wielu możliwych **przeplotów** (*interleavings*), więc sam w sobie dowodzi niewiele. Narzędzia, które się opłacają, celowo zmieniają harmonogram: **test obciążeniowy** (*stress test*) uruchamia wiele przeplotów, `loom` przegląda je systematycznie (*model checking*), Miri sprawdza każde wykonanie pod kątem wyścigów danych, a ThreadSanitizer instrumentuje prawdziwy plik binarny. `thread::sleep` dopisany do niestabilnego testu niczego nie naprawia — przesuwa program na harmonogram, w którym błąd się akurat nie ujawnia.

**Szukaj po polsku:** testowanie kodu współbieżnego · sprawdzanie modelowe · wykrywacz wyścigów · `rust loom model checking` · `rust miri data race` · `rust thread sanitizer nightly`
