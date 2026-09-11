# Time and benchmarking

**Level:** 201 · working knowledge

**One line:** Rust has two clocks where C++ has three, one `Duration` where C++ has a type per unit, and a benchmarking hint the optimizer is asked — not told — to respect; this section is the Rust twin of the first chapter of the [C++ learning library ↗](https://masiarek.github.io/cpp-learning-library/).

Both chapters follow Matt Godbolt's C++Now 2026 keynote, [*Benchmarking: It's About Time* ↗](https://youtu.be/EU_nQh8wg5A) ([slides ↗](https://hudson-trading.github.io/its-about-time/)). The C++ pages measure `std::chrono`; these measure `std::time`. Each lesson links its twin, so either language can come first.

| Lesson | What it answers | C++ twin |
|---|---|---|
| [Two clocks: `Instant` and `SystemTime`](two_clocks/README.md) | Which clock answers *how long* and which answers *what time*, what each one promises, and why C++'s third clock has no Rust counterpart | [Three clocks ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/three_clocks/index.html) |
| [An `Instant` is not a `SystemTime`](an_instant_is_not_a_system_time/README.md) | The talk's *Type safety* slide in Rust — and the one subtraction Rust refuses that C++ allows | [A time point knows its clock ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/a_time_point_knows_its_clock/index.html) |
| [A `Duration` cannot be negative](a_duration_cannot_be_negative/README.md) | One unsigned type for every unit, and what you get instead when a subtraction would go below zero | [A duration is a count and a unit ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/a_duration_is_a_count_and_a_unit/index.html) |
| [Timing a block](timing_a_block/README.md) | `Instant::now()` and `elapsed()`, why never `SystemTime`, and what a single timing cannot tell you | [Timing a block ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/timing_a_block/index.html) |
| [`black_box` is a hint](black_box_is_a_hint/README.md) | What `std::hint::black_box` protects — a value, not the work that produced it — and why C++ has no standard answer at all | [The optimizer deletes your benchmark ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/the_optimizer_deletes_your_benchmark/index.html) |

The order is the order of dependence: the second page subtracts the two types the first one introduces, the third is the type every subtraction produces, and the last two spend all three.

## What this section does not cover

The talk goes further than `std` does, and so does [the C++ chapter ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/index.html):

- **What a clock is.** C++ defines a clock as a set of requirements, *Cpp17Clock*, that any type can meet, and the C++ chapter gives that a lesson of its own. Rust's std has no `Clock` trait — `Instant` and `SystemTime` are two unrelated structs — so that lesson has no twin here.
- **The rest of the talk**, which the C++ chapter carries as outlines: the vDSO that lets `clock_gettime` answer without entering the kernel, `rdtsc`, what asking the time costs, branch predictors, caches, and the statistics that turn many timings into one measurement.

The last of those is what a benchmarking crate does for you in Rust. [Criterion ↗](https://bheisler.github.io/criterion.rs/book/criterion_rs.html) runs the code many times and reports the spread; every example in this library compiles with `rustc` alone, so none of them can use it. The method is written down in the [Rust Performance Book's benchmarking chapter ↗](https://nnethercote.github.io/perf-book/benchmarking.html).

## See also

- [What the optimizer does](../20_Compilers/what_the_optimizer_does/README.md) — constant folding, the reason a benchmark can measure nothing
- [Operators are traits](../12_Traits/operators_are_traits/README.md) — why `Instant - Instant` is allowed to be a `Duration`
- [C and C++: the bugs Rust is a reply to](../31_C_and_Cpp/README.md) — the library's other chapter written against C++
- [Going deeper](../10_Resources/going_deeper/README.md) — the performance and benchmarking shelf

## Po polsku

Ten dział to rustowy odpowiednik pierwszego rozdziału biblioteki C++; oba opierają się na tym samym wykładzie Matta Godbolta z C++Now 2026. Temat jest jeden — **pomiar czasu** (*timing*) — ale Rust rozkłada go inaczej niż C++: ma dwa zegary zamiast trzech, jeden typ na **czas trwania** (*duration*) zamiast osobnego typu dla każdej jednostki i funkcję `std::hint::black_box`, która jest dla optymalizatora **podpowiedzią** (*hint*), a nie rozkazem.

Najważniejsza para pojęć: `Instant` to po polsku po prostu **chwila** — punkt w czasie, który można tylko porównać z innym albo od niego odjąć — odczytana z **zegara monotonicznego** (*monotonic clock*), który nigdy się nie cofa. `SystemTime` to **czas systemowy** (*system time*, potocznie *wall-clock time*): data i godzina, które administrator, NTP albo użytkownik mogą przestawić, także wstecz. Z tej jednej różnicy wynika reszta działu — łącznie z tym, że Rust nie skompiluje nawet `SystemTime - SystemTime`.

Dział kończy się tam, gdzie kończy się biblioteka standardowa. Reszta wykładu — koszt samego pytania o godzinę, `rdtsc`, pamięci podręczne (*cache*), statystyka wielu pomiarów — jest w bibliotece C++, a po stronie Rusta w crate'ach takich jak Criterion.

**Szukaj po polsku:** pomiar czasu w Ruscie · zegar monotoniczny · czas uniksowy · `rust Instant SystemTime` · `rust benchmark black_box`
