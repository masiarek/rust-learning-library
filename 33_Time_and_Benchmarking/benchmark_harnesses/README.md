# Benchmark harnesses

**Level:** 201 · working knowledge

**One line:** `Instant::now()` around a loop is one measurement; a harness such as Criterion or Divan runs the code many times, warms it up, reports a distribution and compares against the last run — the parts of benchmarking std leaves out.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `cargo bench` and `benches/`: what `harness = false` means, and why the built-in `#[bench]` still needs nightly (verify on 1.98.0)
- Criterion: warm-up, sample count, the confidence interval it prints, and the "change: -3.1%… No change in performance detected" line — read one real report
- Divan: attribute-based benches, `black_box`, and allocation counting
- Instruction-count harnesses (`iai-callgrind`, and its successor if it has one — check crates.io) for CI machines too noisy for wall-clock time
- What a harness cannot fix: benchmarking a `dev` build, measuring the optimizer deleting your code, a benchmark input unlike production
- Comparing two implementations fairly: same input, same machine state, and a difference bigger than the noise

## The trap it exists for

Trusting a 3% improvement from one run on a laptop with a browser open. The noise between two identical runs is often larger than that. A harness reports that noise, and a single `Instant` measurement hides it.

## Where this sits

[Timing a block](../timing_a_block/README.md) is the one-measurement version with `Instant`. [`black_box` is a hint](../black_box_is_a_hint/README.md) is what keeps the optimizer honest inside a harness. [Release profiles](../../05_Tooling/release_profiles/README.md) is the build a benchmark must use.

## See also

- [Timing a block](../../33_Time_and_Benchmarking/timing_a_block/README.md) — one measurement with `Instant`
- [`black_box` is a hint](../../33_Time_and_Benchmarking/black_box_is_a_hint/README.md) — stopping the optimizer from deleting the benchmark
- [Two clocks](../../33_Time_and_Benchmarking/two_clocks/README.md) — why a benchmark uses `Instant`
- [Release profiles](../../05_Tooling/release_profiles/README.md) — the build profile a benchmark runs under
- [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) — what the harness is measuring

## If you are coming from another language

- **C++.** Google Benchmark is Criterion's counterpart. The C++ library's [one number is not a measurement ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/one_number_is_not_a_measurement/) and [the optimizer deletes your benchmark ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/the_optimizer_deletes_your_benchmark/) are the same two lessons from the C++ side.
- **Python.** `timeit` repeats and reports the best of several runs; `pytest-benchmark` adds the statistics and the comparison with a saved run, which is Criterion's feature set.
- **Go.** `go test -bench` is built in and `benchstat` does the comparison — the split Rust gets from Criterion alone.

## Po polsku

`Instant::now()` wokół pętli to jeden pomiar; harness benchmarków, taki jak Criterion czy Divan, uruchamia kod wiele razy, rozgrzewa go, podaje rozkład wyników i porównuje z poprzednim przebiegiem. Pułapka: poprawa o 3% z jednego uruchomienia na laptopie jest zwykle mniejsza niż szum między dwoma identycznymi przebiegami.

**Szukaj po polsku:** benchmark w Ruście · pomiar wydajności · `rust criterion tutorial` · `rust divan benchmark` · `cargo bench stable`
