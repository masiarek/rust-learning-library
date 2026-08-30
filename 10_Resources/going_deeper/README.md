# Going deeper

**Level:** reference

**One line:** The domain shelves — unsafe, embedded, async, macros, testing, performance — each of which is a subject rather than a chapter, and none of which you need until you need it.

All links checked 2026-08-23. The organising rule: these are for when you have a *specific* problem. Reading the async book before you have written a program that waits for something is a way of learning vocabulary rather than skills.

## `unsafe` and FFI

- **[The Rustonomicon ↗](https://doc.rust-lang.org/nomicon/intro.html)** — the official book on unsafe Rust, and it opens by telling you not to read it. Correct: it documents the rules you must uphold when the compiler stops checking, and those rules only make sense once you know what is being relaxed.
- **[An unsafe mental model ↗](https://ia0.github.io/unsafe-mental-model/introduction.html)** — a shorter, more recent attempt at the *model* rather than the rules. A good first read before the Nomicon.
- **[The Nomicon's FFI chapter ↗](https://doc.rust-lang.org/nomicon/ffi.html)** — calling C, and being called by it. (The widely-linked Michael Bryan FFI guide is dead as of 2026-08-23; this covers the same ground and is maintained.)
- **[bindgen ↗](https://rust-lang.github.io/rust-bindgen/)** — generates Rust bindings from C headers, so you do not transcribe a struct by hand and get the padding wrong.

The reason this section is first: `unsafe` is the one place where a mistake is not a compile error, so it is the one place where reading before writing genuinely pays.

## Embedded

- **[The Embedded Rust Book ↗](https://docs.rust-embedded.org/book/)** — the canonical starting point: `no_std`, cross-compilation, peripherals, interrupts.
- **[Pico with Rust ↗](https://pico.implrust.com/index.html)** — a project-driven book on the Raspberry Pi Pico, which is the cheapest way to have real hardware in front of you.
- **[Nanosat workshop ↗](https://aerorust.org/nanosat-workshop/Prerequisite_1.html)** — AeroRust's satellite workshop, for when the Pico gets boring.

## Async

- **[The Async Book ↗](https://rust-lang.github.io/async-book/)** — futures, executors, `async`/`await`, and what a runtime actually is.

Worth one warning: async Rust is a distinct dialect with its own difficulties (pinning, `Send` bounds across await points, lifetimes in futures), and almost none of them are the difficulties of ordinary Rust. Learn ordinary Rust first, and reach for async when you have a program that is genuinely waiting on I/O rather than because it sounds modern.

## Macros

- **[proc-macro-workshop ↗](https://github.com/dtolnay/proc-macro-workshop)** — David Tolnay's exercises, and effectively the standard curriculum. Five projects, each building a real derive macro. Hard, and the fastest route to actually understanding them.

## Testing

- **[Test organization ↗](https://doc.rust-lang.org/book/ch11-03-test-organization.html)** — The Book's chapter, and the one that answers "unit or integration, and which file does it go in".
- **[Advanced testing workshop ↗](https://rust-exercises.com/advanced-testing/)** ([repo ↗](https://github.com/mainmatter/rust-advanced-testing-workshop)) — for after `#[test]`: fixtures, property testing, snapshots.
- **[The fuzz book ↗](https://rust-fuzz.github.io/book/introduction.html)** — `cargo-fuzz`, for the inputs you would not have thought of.

See also this library's [cargo-nextest](../../05_Tooling/nextest/README.md) page for the runner, which is a different question from what to test.

## Performance and benchmarking

- **[The Rust Performance Book ↗](https://nnethercote.github.io/perf-book/)** — by one of the people who made rustc itself faster; specific, measured, and short.
- **[Its benchmarking chapter ↗](https://nnethercote.github.io/perf-book/benchmarking.html)** — the methodology, and the reason a single timing is not a measurement.
- **[The Criterion book ↗](https://bheisler.github.io/criterion.rs/book/criterion_rs.html)** — the library's own documentation, which this library exercises in [ex03 ↗](https://github.com/masiarek/rust-practice) and writes up on [Compile times](../../05_Tooling/compile_times/README.md)'s neighbour pages.

## Networking and interop

- **[Quinn's networking introduction ↗](https://quinn-rs.github.io/quinn/networking-introduction.html)** — QUIC in Rust, and an unusually good explanation of the protocol regardless of language.
- **[Rust–Python interop ↗](https://rust-exercises.com/rust-python-interop/)** — PyO3 and maturin: writing a Python extension in Rust. The most likely first *professional* use of Rust for someone whose day job is Python.
- **[Telemetry ↗](https://rust-exercises.com/telemetry/)** — logging, tracing and metrics, which is the part nobody teaches and everybody needs by week three of running something.

## Video

- **[Jon Gjengset ↗](https://www.youtube.com/@jonhoo)** — long-form, live-coded, and the deepest free Rust teaching that exists. Not for beginners; extraordinary once you can read Rust.
- **[Decrusted ↗](https://www.youtube.com/playlist?list=PLqbS7AVVErFirH9armw8yXlE6dacF-A6z)** — his series reading the *source* of crates you already depend on, which is a rare and useful thing to watch someone do.
- **[No Boilerplate ↗](https://www.youtube.com/@NoBoilerplate)** — fast, opinionated, well-argued. Several pages in this library's [toolchain section](../../TOOLCHAIN.md) exist because of one of its videos, including where they disagree with it.

## See also

- [Books](../books/README.md) — the general shelf
- [Exercises](../exercises/README.md) — practice tracks

## Po polsku

To jest półka „na później”, i to nie w znaczeniu „trudniejsze”, tylko **dziedzinowe**: każdy dział tutaj jest osobnym tematem, a nie kolejnym rozdziałem kursu. Reguła porządkująca całą stronę brzmi: sięgasz po nie, kiedy masz konkretny problem, a nie w kolejności spisu treści. Przeczytanie książki o `async`, zanim napisało się program, który faktycznie na coś czeka, daje **słownictwo, a nie umiejętność** — różnica jest niewidoczna dla samego czytającego i doskonale widoczna dla kompilatora przy pierwszej próbie napisania czegokolwiek.

Jeden wyjątek od reguły „najpierw pisz”, i dlatego stoi na tej stronie pierwszy: `unsafe`. To jedyne miejsce w Ruscie, w którym pomyłka nie jest błędem kompilacji, tylko błędem w programie, który się zbudował i działa — więc tu czytanie przed pisaniem naprawdę się opłaca. Warto przy okazji rozbroić odruch tłumaczeniowy: `unsafe` **nie znaczy „niebezpieczny kod”**. To deklaracja odpowiedzialności — „biorę na siebie niezmienniki, których kompilator w tym miejscu już nie sprawdza”. Blok `unsafe` niczego nie wyłącza i nie osłabia; przenosi obowiązek dowodu z kompilatora na autora. Samego słowa nie tłumacz nawet w myślach, bo w kodzie, w komunikatach i w każdym wyniku wyszukiwania i tak stoi `unsafe`.

Podobne ostrzeżenie dotyczy `async`: to osobny dialekt języka, z własnymi trudnościami (`Pin`, ograniczenia `Send` w poprzek punktów `await`, czasy życia wewnątrz `Future`), z których prawie żadna nie jest trudnością zwykłego Rusta. Naucz się najpierw zwykłego. Na koniec praktyczna uwaga o tym, jak Rust wchodzi do pracy: najczęściej nie przez „projekt w Ruscie”, tylko przez jeden moduł — rozszerzenie do istniejącego programu w Pythonie napisane w PyO3. To decyzja o jednym module, a nie o języku całego zespołu, więc bywa jedyną, którą da się przeforsować.

**Szukaj po polsku:** niebezpieczny Rust · programowanie asynchroniczne · `rustonomicon` · `rust async await tokio` · `pyo3 python rust`
