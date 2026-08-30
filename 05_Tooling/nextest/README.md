# cargo-nextest: one process per test, and what that actually buys

**Level:** 201 · working knowledge

**One line:** [nextest ↗](https://nexte.st/) runs each test in its own process rather than as a thread inside one — which is faster on a large suite, but the reason to care is that a test which *aborts the process* becomes one reported failure instead of taking the whole run down with it.

```sh
cargo install --locked cargo-nextest
cargo nextest run
```

**Should you have it?** Honestly: **not urgently, and not for a handful of exercises.** `cargo test` is fine at that size, it is already installed, and it runs doctests, which nextest does not. This page is here so you can tell when the answer changes — and it does change, sharply, once a suite is big or lives in CI.

## What is different

`cargo test` compiles your tests into a binary and runs them as **threads in one process**. nextest runs each as **its own process**. Everything below follows from that one decision:

| | `cargo test` | `cargo nextest run` |
|---|---|---|
| Isolation | threads in one process | a process per test |
| A test that aborts or segfaults | takes down the run | one reported failure |
| A test that sets a global, or `set_current_dir` | leaks into its neighbours | cannot |
| Output when tests fail | interleaved, captured per-thread | per-test, printed after the summary |
| Per-test timing | no | yes, with slow-test detection |
| Retries, CI sharding | no | `--retries`, `--partition` |
| Doctests | **yes** | **no** — run `cargo test --doc` separately |

The bottom row is the one that bites, and it bites quietly: switch a project to nextest and its doctests stop running, with nothing to tell you. If your crate documents itself with examples — and a Rust library should — the honest CI line is both:

```sh
cargo nextest run
cargo test --doc
```

## The isolation is the feature

Speed is what gets advertised, and it is real on a large suite. But the property worth adopting a tool for is the one you meet on a bad day.

A test that panics is handled fine by both runners. A test that **aborts the process** — a C library calling `abort()`, a stack overflow, a double panic during unwinding, an `exit()` somewhere in a dependency — is not a panic, and under `cargo test` it kills the process that was running every other test in that binary. You get a truncated run and no clear culprit. Under nextest that test is one line marked failed, and everything else still reports.

The same reasoning covers shared mutable state. Tests as threads share one process's globals, environment variables and current directory; a test that changes any of them changes them for whatever runs concurrently. Process isolation makes that class of flake impossible rather than merely discouraged — which is worth knowing before you spend an afternoon on a test that only fails when run with others.

## When it earns its keep

- **A suite big enough that you notice waiting.** 12 million downloads is not a fashion; it is people whose test runs got long.
- **CI**, where `--partition` shards a suite across runners and `--retries` distinguishes flaky from broken instead of turning the build red.
- **Any suite with a known flake**, because per-test timing and clean per-test output are how you find it.
- **A crate calling into C**, where "the process aborted" is a real thing that happens.

For an exercise with three `#[test]` functions, none of that applies, and `cargo test` is one less thing installed.

## If you are coming from another language

- **Python** — nearest to `pytest-xdist` for the parallelism, but the isolation model is stronger: `xdist` gives you worker processes shared by many tests, nextest gives one process per test. The Python habit of relying on fixtures to undo global state is exactly what process isolation makes unnecessary.
- **ABAP** — ABAP Unit runs in one session and shares the same state problem; the discipline of a `teardown` doing the cleanup is the manual version of what nextest buys structurally.

## See also

- [bacon](../bacon/README.md) — press `t` and this is what runs, if it is installed
- [A tree of practice projects](../practice_workspace/README.md) — `cargo nextest run` at the workspace root runs every exercise's tests at once
- [Strict clippy lints](../strict_lints/README.md) — where `unwrap` is allowed again, because in a test it *is* the assertion

## Po polsku

Cała różnica sprowadza się do jednej decyzji: `cargo test` kompiluje testy do jednego pliku wykonywalnego i uruchamia je jako **wątki w jednym procesie**, a `cargo nextest run` daje **każdemu testowi osobny proces**. Szybkość na dużym zestawie testów jest tego skutkiem ubocznym; prawdziwy powód widać dopiero w zły dzień. I tu uwaga na rozróżnienie, które po polsku regularnie się zaciera, bo o wszystkim mówi się „program się wysypał”: **panika to nie jest przerwanie procesu**. Panikę oba narzędzia obsłużą tak samo — test jest raportowany jako nieudany, reszta leci dalej. Ale `abort()` z biblioteki C, przepełnienie stosu, podwójna panika w trakcie odwijania stosu (*unwinding*) czy `exit()` gdzieś w zależności paniką nie są: pod `cargo test` zabijają proces, w którym akurat działały wszystkie pozostałe testy, i zostajesz z uciętym przebiegiem bez wskazania winnego. Pod nextestem to jedna linijka oznaczona jako failed, a cała reszta i tak się zaraportuje.

Ta sama izolacja rozwiązuje drugą klasę kłopotów. Testy będące wątkami dzielą zmienne globalne, zmienne środowiskowe i bieżący katalog, więc `set_current_dir` w jednym teście zmienia go sąsiadom; osobny proces czyni to **niemożliwym**, a nie tylko odradzanym — warto o tym wiedzieć, zanim stracisz popołudnie na test, który przewraca się wyłącznie przy uruchomieniu całego zestawu. Jest za to koszt, i to cichy: **nextest nie uruchamia doctestów**, czyli testów z przykładów w dokumentacji, i nie mówi o tym ani słowem. Po przełączeniu projektu twoje przykłady spod `///` przestają być sprawdzane, a CI dalej świeci na zielono — dlatego uczciwa para poleceń brzmi `cargo nextest run` **i** `cargo test --doc`. Przy trzech funkcjach `#[test]` w ćwiczeniu nic z tego się jednak nie stosuje: `cargo test` już masz i w zupełności wystarcza.

**Szukaj po polsku:** testy jednostkowe w Ruscie · niestabilne testy (flaky) · `cargo nextest run` · `cargo test --doc` · `nextest doctests`
