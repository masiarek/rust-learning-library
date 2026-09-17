# How `cargo test` runs your tests

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `cargo test` builds one test binary for your library and one for each file directly in `tests/`, runs the tests inside each binary on parallel threads of a single process, and then compiles and runs your doc tests — so link time grows with the number of test files, and tests that share global state can interfere with each other.

## What it has to cover

- **What gets built:** the lib's unit tests, each binary target's, one crate per file in `tests/`, and the doc tests — read from `cargo test`'s own output, in the order it runs them
- The cost of many files in `tests/`: each is compiled and linked separately, and the one-crate layout (`tests/it/main.rs` with modules) that trades that for one link
- **Threads, not processes.** Tests in one binary run in parallel by default, so a test that sets an environment variable, changes the working directory or writes a fixed file path can break a different test — intermittently. `--test-threads=1` hides it; it does not fix it
- Output capture: why `println!` in a passing test prints nothing, and the flag that shows it
- Filtering by name, `--ignored`, and `--include-ignored`
- Doc tests: compiled from the documentation, and what the 2024 edition's [merged doc tests ↗](https://doc.rust-lang.org/edition-guide/rust-2024/rustdoc-doctests.html) changed about their build time
- A panicking test, and what happens to the other tests running at the same moment

## The trap it exists for

Two tests that each set the same environment variable. Each passes alone, both pass most of the time together, and the suite fails once a week in CI — a data race on process state that no test ever mentions.

## See also

- [Where a test goes](../where_a_test_goes/README.md) — the three kinds, and the `tests/` rule this page explains the cost of
- [cargo-nextest](../../05_Tooling/nextest/README.md) — one process per test, which removes the shared-state problem and changes the cost
- [Temp dirs in tests](../../04_Files/temp_dirs_in_tests/README.md) — the fix for the fixed-file-path version of the trap
- [cargo test ↗](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [the test harness ↗](https://doc.rust-lang.org/rustc/tests/index.html) — the reference

## Po polsku

`cargo test` buduje osobny plik wykonywalny z testami dla biblioteki, dla każdego pliku w katalogu `tests/` i dla testów dokumentacyjnych, a testy wewnątrz jednego pliku uruchamia **równolegle, w wątkach jednego procesu**. Z tego wynikają dwie rzeczy: czas linkowania rośnie z liczbą plików testowych, a testy zmieniające wspólny stan procesu — zmienne środowiskowe, katalog roboczy, stałą ścieżkę pliku — mogą sobie nawzajem przeszkadzać, i to tylko czasami.

**Szukaj po polsku:** uruchamianie testów w Ruscie · testy równoległe · `cargo test test-threads` · `rust integration tests compile time single binary`
