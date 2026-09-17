# A harness of your own

**Level:** 301 · deep dive

**One line:** `#[test]` works because rustc writes a `main` for the test binary and hands every test to libtest. `harness = false` tells cargo not to, so the test file becomes an ordinary program that passes by exiting 0, and discovery, filtering, `--list` and catching panics are yours to write or to borrow from `libtest-mimic`.

```toml
# Cargo.toml
[[test]]
name = "prices"        # tests/prices.rs
harness = false
```

```rust
// tests/prices.rs
fn main() {
    // run whatever you like; exit 0 for a pass, anything else for a failure
}
```

## What `#[test]` turns into

A file with one test, expanded by nightly's `-Zunpretty=expanded` flag:

```rust
#[test]
fn parses_a_number() {
    assert_eq!("42".parse::<u32>(), Ok(42));
}
```

```text title="rustc +nightly --edition 2024 --test -Zunpretty=expanded two.rs (1.100.0-nightly 2026-08-27), abridged: the prelude lines, most attributes and TestDesc fields, and the assert_eq! body removed"
extern crate test;
#[rustc_test_marker = "parses_a_number"]
pub const parses_a_number: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("parses_a_number"),
            ignore: false,
            should_panic: test::ShouldPanic::No,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(parses_a_number())),
    };
fn parses_a_number() { … }
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&parses_a_number])
}
```

`#[test]` registers a description of the function, and `--test` adds a `main` that passes the list of every registered test to libtest. The harness is that `main` and the library behind it. Everything `cargo test` seems to do inside one binary happens in `test_main_static`: parsing `--list` and the name filter, running tests on threads, catching each panic, printing `test … ok`, and exiting with 101 when something failed.

## Switching it off

With `harness = false`, cargo compiles the target as an ordinary binary and runs it. Two things it still does, both checked with cargo 1.98.0:

- **It passes everything after `--` to your `main`, unchanged.**
- **It judges the run by the exit status alone**, and exits with that same status itself.

```text title="cargo test --test custom -- --list parses, where main prints its arguments and exits 3 (the executable's path shortened)"
     Running harness_tests/custom.rs (target/debug/deps/custom-603ad85e71ff5488)
my harness got ["--list", "parses"]
error: test failed, to rerun pass `--test custom`

Caused by:
  process didn't exit successfully: `…/target/debug/deps/custom-603ad85e71ff5488 --list parses` (exit status: 3)
```

`cargo test` then exited with status 3. Any non-zero status is a failure; 101 is only libtest's convention.

One silent trap. The target is still compiled with `--cfg test`, so `cfg!(test)` is `true` inside it, but without `--test`. A `#[test]` function written in that file is **quietly dropped**: no warning, never run.

## What you now owe

| libtest did this | With `harness = false` |
|---|---|
| found every `#[test]` function | you keep the list yourself, by hand or by scanning a directory |
| `cargo test -- parses` runs only matching names | your `main` receives `"parses"` and must decide what it means |
| `--list` for IDEs and scripts | not implemented unless you implement it |
| catches each test's panic and keeps going | the first panic ends the process, and the tests after it never run |
| runs tests on parallel threads | sequential unless you spawn threads |
| exits 101 when anything failed | whatever `main` returns or passes to `process::exit` |

The example below writes the first four rows in about fifty lines, then removes `catch_unwind` to show the fourth row going wrong. `rejects_a_word` never runs, and the output just stops.

## Why anyone does it

- **Setup once for the whole binary.** Start a database container before the first test and stop it after the last. Inside a normal test binary there is no hook for that; with your own `main` it is two lines around the loop. The example's `Suite` runs once per run of the binary, not once per test.
- **One test per data file or table row.** A `#[test]` with a loop over a table stops at the first bad row and reports one failure. Generated trials name every row, run every row, and let the filter pick one. The practice below builds exactly that. This library's own checker, [`tools/run_examples.py`](../../tools/run_examples.py), is the same idea in Python: one check per example file, found by scanning.
- **A different runner model altogether**: running each test in a subprocess, or with a timeout.

## Borrow the hard part: `libtest-mimic`

[`libtest-mimic` ↗](https://docs.rs/libtest-mimic) parses the same command line libtest does and prints the same output, so IDEs, CI and your own habits keep working. You supply the list of trials. Checked against 0.8.2:

```rust
use libtest_mimic::{Arguments, Failed, Trial};

fn parses_a_price() -> Result<(), Failed> {
    if my_crate::price("250") == Some(250) { Ok(()) } else { Err("250 did not parse".into()) }
}

fn main() {
    let args = Arguments::from_args();
    let tests = vec![
        Trial::test("parses_a_price", parses_a_price),
        Trial::test("panics", || panic!("boom")),
    ];
    libtest_mimic::run(&args, tests).exit();
}
```

In that run, `cargo test --test mimic -- price` ran one trial and reported the rest as filtered out, `-- --list` printed `parses_a_price: test` lines, and the panicking trial was reported as `test panicked: boom` while the others still ran.

## The limits

- **Nothing collects tests for you.** A harness gets a list, not a crate scan. Build it by hand, from a directory of files, or with a registration crate such as [`inventory` ↗](https://docs.rs/inventory) or [`linkme` ↗](https://docs.rs/linkme).
- **A harness owns one test binary, not the run.** Setup in `tests/api.rs`'s harness does not happen for `tests/cli.rs`, because [each is its own process](../how_cargo_test_runs/README.md). Setup across the whole suite needs coordination between processes, or a runner that replaces `cargo test`, such as [cargo-nextest](../../05_Tooling/nextest/README.md).
- **It is more code to keep correct.** A harness that ignores the filter makes `cargo test -- one_name` run everything, which nobody expects. Reach for `libtest-mimic` before writing the table above by hand.

## If you are coming from another language

- **Go.** `func TestMain(m *testing.M)` is what Rust does not have, and what most `harness = false` targets are really written to get: setup, `m.Run()`, teardown, while Go keeps discovery and `-run` filtering. In Rust you cannot wrap libtest's `main`; you replace it. `libtest-mimic` gets you back to Go's position, apart from discovery.
- **Python.** pytest separates collection from running, and both are extensible: a session-scoped fixture is the once-per-run setup, and `pytest_generate_tests` or `@pytest.mark.parametrize` produces one named test per row. That is everything this page builds by hand, without giving up discovery. In Rust the default harness has no hooks, so those features either come from attribute macros on individual tests (`rstest`'s cases, for example) or from replacing the harness.
- **Java.** JUnit 5's `@TestFactory` returns dynamic tests built at run time, which is the one-test-per-row half. `@BeforeAll` is the setup half, per class. Writing a whole JUnit Platform `TestEngine` is the real equivalent of `harness = false`, and like it, rarely the first thing to try.
- **ABAP.** ABAP Unit has `CLASS_SETUP` and `CLASS_TEARDOWN` for once-per-class setup, and the framework always owns discovery and running. There is no supported way to replace the runner, which is a fair summary of how often you should want to.

---

## The verified output

<!-- output:a_harness_of_your_own -->
*Verified output of [`a_harness_of_your_own.rs`](examples/a_harness_of_your_own.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. cargo test --test prices
   [setup] once, before any test (start a container, seed a database)
   running 3 tests
   test parses_a_price ... ok
   test accepts_one_decimal ... FAILED
   test rejects_a_word ... ok
   ---- accepts_one_decimal ---- assertion `left == right` failed; left: None; right: Some(250)
   test result: FAILED. 2 passed; 1 failed; 0 filtered out
   [teardown] once, after the last test
   exit status 101

2. cargo test --test prices -- parses
   [setup] once, before any test (start a container, seed a database)
   running 1 test
   test parses_a_price ... ok
   test result: ok. 1 passed; 0 failed; 2 filtered out
   [teardown] once, after the last test
   exit status 0

3. cargo test --test prices -- --list
   parses_a_price: test
   accepts_one_decimal: test
   rejects_a_word: test
   exit status 0

4. Setup ran 2 times for 2 runs that executed tests
   Once per run of the binary, not once per test, and never for --list.

5. The same tests, with no catch_unwind around each one
   test parses_a_price ... ok
   test accepts_one_decimal ...
   exit status Some(101); rejects_a_word never ran, and no summary line says so
```
<!-- /output -->

## Practice

**One test per row.** A table of five `(input, expected)` pairs tests `price_in_cents`, and two of the rows are wrong. Written as one `#[test]` with a loop, it reports one failure and hides the other.

Turn the table into five trials, each with its own name, run all of them even when some fail, and report which rows broke. Then make the name filter select rows, and explain why filtering on `2.5` runs two of them. (libtest's answer to that is `--exact`.)

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:a_harness_of_your_own_kata -->
*[`a_harness_of_your_own_kata.rs`](examples/a_harness_of_your_own_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one test per row.
//!
//!   rustc --edition 2024 a_harness_of_your_own_kata.rs -o /tmp/ahooyk && /tmp/ahooyk

use std::panic;

fn price_in_cents(text: &str) -> Option<u32> {
    let (whole, cents) = text.split_once('.')?;
    if cents.len() != 2 {
        return None;
    }
    Some(whole.parse::<u32>().ok()? * 100 + cents.parse::<u32>().ok()?)
}

/// The table. Two rows expect something the function does not do.
const ROWS: [(&str, Option<u32>); 5] = [
    ("2.50", Some(250)),
    ("0.05", Some(5)),
    ("2.5", Some(250)),   // wrong: one decimal is rejected
    ("two", None),
    ("1,000.00", Some(100_000)), // wrong: the comma is not understood
];

/// Before: one #[test] with a loop. It stops at the first row that fails.
fn one_test_with_a_loop() {
    for (input, expected) in ROWS {
        assert_eq!(price_in_cents(input), expected, "row {input:?}");
    }
}

/// After: a trial per row, each with its own name. A closure captures the row,
/// so a trial holds a `Box<dyn Fn()>` rather than a plain `fn()`.
struct Trial {
    name: String,
    run: Box<dyn Fn() + panic::RefUnwindSafe>,
}

fn trials() -> Vec<Trial> {
    ROWS.iter()
        .map(|&(input, expected)| Trial {
            name: format!("price_in_cents::{input}"),
            run: Box::new(move || assert_eq!(price_in_cents(input), expected)),
        })
        .collect()
}

fn first_line(payload: Box<dyn std::any::Any + Send>) -> String {
    let text = payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default();
    text.lines().map(str::trim).collect::<Vec<_>>().join("; ")
}

fn run(filter: Option<&str>) -> i32 {
    let all = trials();
    let mut failed = 0;
    let mut ran = 0;
    for t in all.iter().filter(|t| filter.is_none_or(|f| t.name.contains(f))) {
        ran += 1;
        match panic::catch_unwind(|| (t.run)()) {
            Ok(()) => println!("   test {} ... ok", t.name),
            Err(p) => {
                failed += 1;
                println!("   test {} ... FAILED  {}", t.name, first_line(p));
            }
        }
    }
    println!("   {} passed; {failed} failed; {} filtered out", ran - failed, all.len() - ran);
    if failed == 0 { 0 } else { 101 }
}

fn main() {
    let quiet = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    println!("1. One #[test] with a loop over the table");
    match panic::catch_unwind(one_test_with_a_loop) {
        Ok(()) => println!("   ok"),
        Err(p) => println!("   FAILED  {}", first_line(p)),
    }
    println!("   One failure reported. The comma row is broken too, and nothing says so");
    println!("   until the first one is fixed.");

    println!();
    println!("2. A trial per row");
    let status = run(None);
    println!("   exit status {status}; both broken rows named, every row run");

    println!();
    println!("3. The filter now selects rows: -- 2.5");
    let status = run(Some("2.5"));
    println!("   exit status {status}; \"2.5\" is a substring of \"2.50\" too, so two rows ran");

    panic::set_hook(quiet);
}
```
<!-- /source -->

<!-- output:a_harness_of_your_own_kata -->
*Verified output of [`a_harness_of_your_own_kata.rs`](examples/a_harness_of_your_own_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One #[test] with a loop over the table
   FAILED  assertion `left == right` failed: row "2.5"; left: None; right: Some(250)
   One failure reported. The comma row is broken too, and nothing says so
   until the first one is fixed.

2. A trial per row
   test price_in_cents::2.50 ... ok
   test price_in_cents::0.05 ... ok
   test price_in_cents::2.5 ... FAILED  assertion `left == right` failed; left: None; right: Some(250)
   test price_in_cents::two ... ok
   test price_in_cents::1,000.00 ... FAILED  assertion `left == right` failed; left: None; right: Some(100000)
   3 passed; 2 failed; 0 filtered out
   exit status 101; both broken rows named, every row run

3. The filter now selects rows: -- 2.5
   test price_in_cents::2.50 ... ok
   test price_in_cents::2.5 ... FAILED  assertion `left == right` failed; left: None; right: Some(250)
   1 passed; 1 failed; 3 filtered out
   exit status 101; "2.5" is a substring of "2.50" too, so two rows ran
```
<!-- /output -->

</details>

## See also

- [How `cargo test` runs your tests](../how_cargo_test_runs/README.md): the binaries a harness runs inside, and the exit status 101
- [Where a test goes](../where_a_test_goes/README.md): the three kinds of test the default harness runs
- [What an attribute is](../../27_Modules/what_an_attribute_is/README.md): `#[test]` and `#[cfg(test)]` as the building blocks
- [cargo-nextest](../../05_Tooling/nextest/README.md): replacing the runner instead of the harness
- [Testing: courses and links](../resources/README.md): the Advanced Rust testing course's test-harness and test-macro sections, which this page follows
- [Every testing error, and its fix](../testing_errors/README.md): the compiler errors around tests, each with a fix that compiles
- [Lints around tests](../testing_lints/README.md): rustc and clippy on test code, bad and good

## Po polsku

*Test harness* to po polsku najczęściej „mechanizm uruchamiający testy”. W Ruście jest nim zwykła funkcja `main`, którą kompilator dopisuje sam, gdy budujesz z flagą `--test`: `#[test]` rejestruje opis funkcji, a `main` przekazuje listę wszystkich testów do biblioteki `test` (libtest). To ona parsuje filtr i `--list`, uruchamia testy w wątkach, łapie panikę każdego z nich i kończy proces kodem 101. Ustawienie `harness = false` w `Cargo.toml` wyłącza to wszystko: plik z testami staje się zwykłym programem, `cargo` przekazuje mu argumenty po `--` bez zmian i ocenia wynik wyłącznie po kodzie wyjścia. Pułapka: cel nadal kompiluje się z `--cfg test`, ale bez `--test`, więc funkcja oznaczona `#[test]` w takim pliku znika po cichu, bez ostrzeżenia.

Robi się to zwykle z dwóch powodów: żeby raz na cały przebieg przygotować i posprzątać coś kosztownego (kontener z bazą danych), albo żeby każdy wiersz tabeli przypadków był osobnym, nazwanym testem, zamiast jednej pętli, która zatrzymuje się na pierwszym błędzie. Cena to wszystko, co wcześniej było za darmo: filtr, `--list`, łapanie paniki i równoległość. Bez `catch_unwind` pierwszy nieudany test kończy proces, a testy po nim nigdy się nie wykonują. Dlatego zanim napiszesz własny, sięgnij po `libtest-mimic`, który zachowuje się dokładnie jak `cargo test` i wymaga od ciebie tylko listy testów.

**Szukaj po polsku:** własny mechanizm uruchamiania testów · testy sterowane danymi · `rust harness = false` · `rust libtest-mimic` · `rust test setup teardown once` · `rust custom test harness`
