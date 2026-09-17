# How `cargo test` runs your tests

**Level:** 201 · working knowledge

**One line:** `cargo test` is several programs, not one: a test binary per target, run one after another, and the tests inside one binary are threads of one process, so they share its working directory, its environment and its statics. Each extra file in `tests/` is one more binary to link.

```text title="cargo test on a package with a lib, a main.rs, src/bin/report.rs, three test targets and two doc tests; one run, summary lines only"
     Running unittests src/lib.rs (target/debug/deps/demo-49d8333cf2857dce)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src/main.rs (target/debug/deps/demo-62f840904fb82c44)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src/bin/report.rs (target/debug/deps/report-5688da16fd1f5365)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/api.rs (target/debug/deps/api-1b3d713e6eb3776b)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/cli.rs (target/debug/deps/cli-b4f4aaaaa13a752d)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/end_to_end/main.rs (target/debug/deps/end_to_end-3e6567dfe4c3251e)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests demo
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
all doctests ran in 2.55s; merged doctests compilation took 0.38s
```

Each `Running` line is a separate executable in `target/debug/deps`, started as its own process.

| Where the test is written | Test binaries | The tests in it run as |
|---|---|---|
| `src/lib.rs` and every module under it | 1 | threads of one process |
| `src/main.rs`, and each `src/bin/*.rs` | 1 each | threads of one process |
| each file directly in `tests/` | 1 each | threads of one process |
| `tests/<dir>/main.rs` | 1 | threads of one process |
| `tests/common/mod.rs`, with no `main.rs` beside it | **none** | a module that `tests/api.rs` pulls in with `mod common;` |
| doc tests, edition 2024 | 1, merged | **a process each** |
| doc tests, edition 2021 | 1 per doc test | a process each |

The package above had `tests/common/mod.rs` too, and it has no `Running` line. The two doc-test rows were checked with nightly's `--persist-doctests`: edition 2021 left one `rust_out` per doc test, and edition 2024 left one `merged_doctest_2024_0/rust_out` for both. Two doc tests that wrote their process id to a file wrote two different ids under both editions. The [rustdoc book ↗](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) says the same: compatible doc tests are merged, each still runs in its own process, and a fence marked `standalone_crate` is kept out of the merge. The [edition guide ↗](https://doc.rust-lang.org/edition-guide/rust-2024/rustdoc-doctests.html) has the build-time reasoning.

## Tests in one binary share a process

```rust
#[test]
fn a_moves_into_the_temp_dir() {
    std::env::set_current_dir(std::env::temp_dir()).unwrap();
}

#[test]
fn b_reads_a_relative_path() {
    assert!(std::path::Path::new("cwd_trap.rs").exists());
}
```

Each test passes when it runs alone:

```text title="rustc --edition 2024 --test cwd_trap.rs -o cwd_trap && ./cwd_trap b_reads"
running 1 test
test b_reads_a_relative_path ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

Run them together and `b` fails, because `a` moved the whole process:

```text title="./cwd_trap --test-threads=1, exit status 101"
running 2 tests
test a_moves_into_the_temp_dir ... ok
test b_reads_a_relative_path ... FAILED

failures:

---- b_reads_a_relative_path stdout ----

thread 'b_reads_a_relative_path' (29083032) panicked at cwd_trap.rs:8:5:
assertion failed: std::path::Path::new("cwd_trap.rs").exists()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    b_reads_a_relative_path

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

With one thread the order is fixed. libtest's source says tests are sorted by name at compile time, so `a_` runs first every time. With the default thread count the result depends on which thread gets there first. Five runs of `./cwd_trap` in a row gave `ok`, `ok`, `FAILED`, `FAILED`, `FAILED`. That is the flaky test you meet in real code: it passes on your machine, fails in CI, and passes again when you rerun it to see the failure. `--test-threads=1` makes it fail every time here, and in other suites it makes the failure disappear. Either way it has not fixed anything.

What one binary's tests share is everything that belongs to a process:

- **The working directory.** Any relative path, in your code or in a dependency, resolves against it.
- **The environment.** [`std::env::set_var` ↗](https://doc.rust-lang.org/std/env/fn.set_var.html) is `unsafe` in edition 2024. Its docs say that outside Windows, the only sound choice in a multi-threaded program is not to call it. A test binary is always multi-threaded: libtest runs every test on a thread of its own, even with `--test-threads=1`, and names the thread after the test, which is the `thread 'b_reads_a_relative_path'` in the panic above.
- **Statics.** A `static`, a `OnceLock`, a `LazyLock` or a global connection pool is one value per process. A `static Mutex` used to serialise tests does that inside `tests/api.rs` and does nothing about `tests/cli.rs`, which is another process.

The example below checks all three from one program: one thread's `set_current_dir` moves another thread, a child process's does not, and a static counts separately in each process.

## What to do about it

Three fixes, in the order to try them:

1. **Pass it in.** A function that takes the directory, the base URL or the config as a parameter can be given a different one by each test. [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md) is this fix for files, and the practice below applies it to the test pair above.
2. **Give each test its own copy.** Use a fresh temp directory, a random port, or a separate logical database per test. `#[sqlx::test]` and `wiremock`'s `MockServer` are built on this idea, and [the testing reading list](../resources/README.md) says where each is taught.
3. **Give each test its own process.** [cargo-nextest](../../05_Tooling/nextest/README.md) does this. It is the fix for code you cannot change, such as a C library that reads the environment.

## Output is captured, and shown only for failures

```rust
#[test]
fn prints_and_passes() {
    println!("you will not see this");
}

#[test]
fn prints_and_fails() {
    println!("you will see this");
    assert_eq!(1 + 1, 3);
}
```

```text title="cargo test --test cli, one run, cut after the failure report"
running 2 tests
test prints_and_passes ... ok
test prints_and_fails ... FAILED

failures:

---- prints_and_fails stdout ----
you will see this

thread 'prints_and_fails' (30040205) panicked at tests/cli.rs:9:5:
assertion `left == right` failed
  left: 2
 right: 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The harness collects each test's printed output and throws it away when the test passes, which is why a `println!` added to debug a green test seems to do nothing. Two flags, both after the `--`:

| Flag | What you see |
|---|---|
| `cargo test -- --show-output` | the same run, plus a `successes:` section with each passing test's output under its name |
| `cargo test -- --nocapture` | everything as it is printed, interleaved across tests running at the same time |

`--show-output` is the one to reach for first, because each block is labelled with the test that printed it.

## Choosing what runs

- `cargo test parse` runs the tests whose names contain `parse`, in every binary, and counts the rest as `filtered out`. Add `-- --exact` to match a whole name.
- `cargo test --test cli` runs one test binary, and `--lib`, `--bins` and `--doc` pick the others.
- `#[ignore]`d tests are compiled and skipped. `-- --ignored` runs **only** those (1 test ran and 4 were filtered out, on a file with one ignored test among five), and `-- --include-ignored` runs everything (all 5). [Where a test goes](../where_a_test_goes/README.md) has the attribute itself.

## When a test panics, and when it aborts

A failing test is a panic on that test's own thread. libtest catches it, records the failure, and the tests running beside it carry on: in the transcript above, `prints_and_passes` still passed.

An **abort** is different. `std::process::abort()`, a stack overflow, a double panic, or a C library calling `abort()` ends the process, and with it every test in that binary:

```text title="cargo test --test cli, where test a calls std::process::abort() and test b is empty (the executable's path shortened)"
     Running tests/cli.rs (target/debug/deps/cli-b4f4aaaaa13a752d)

running 2 tests
error: test failed, to rerun pass `--test cli`

Caused by:
  process didn't exit successfully: `…/target/debug/deps/cli-b4f4aaaaa13a752d` (signal: 6, SIGABRT: process abort signal)
```

No `test … ok` line, no `failures:` list, no `test result:` line, and no name for the test that did it. `cargo test` exited 101. This is the case [cargo-nextest](../../05_Tooling/nextest/README.md) exists for: with a process per test, the abort is one failed test.

## Every file in `tests/` is another link

Each file directly in `tests/` is compiled and **linked** as its own executable, and each one links your library again. Twenty such files, each holding one test, against the same twenty tests as modules of one `tests/it/main.rs`:

| Layout | Test binaries built | `cargo test --no-run` after editing `src/lib.rs` (three runs) |
|---|---|---|
| `tests/f01.rs` … `tests/f20.rs` | 20 | 1.23 s, 1.19 s, 1.17 s |
| `tests/it/main.rs` with `mod f01;` … `mod f20;` | 1 | 0.39 s, 0.41 s, 0.45 s |

Timings are from an Intel Mac and the ratio is what carries over, not the numbers. The one-binary layout also means one process, so everything above about shared state now applies to the whole integration suite at once. That is the price of the faster build, and whether it is worth paying depends on how much process state your tests touch.

## Order, failure, and the exit status

- cargo runs the test binaries **one at a time**, in the order of the first transcript. Inside a binary, tests run in parallel on [`available_parallelism()` ↗](https://doc.rust-lang.org/std/thread/fn.available_parallelism.html) threads; `--test-threads=N` or the `RUST_TEST_THREADS` variable changes that.
- **The first failing binary stops the run.** If `tests/api.rs` fails, cargo prints ``error: test failed, to rerun pass `--test api` `` and stops, so `tests/cli.rs` and the doc tests never start. `cargo test --no-fail-fast` runs every binary and finishes with `error: 1 target failed`.
- **A failed run exits with status 101.** After its report, libtest exits with `ERROR_EXIT_CODE`, which its source defines as `101`, and cargo exits with the same status. 101 is also what any Rust program exits with when its main thread panics, as section 6 of the output shows.

## If you are coming from another language

- **Python.** `pytest` runs every test in one interpreter process, one after another, so `os.chdir` or `os.environ[...] = ...` in one test leaks into every test after it. That is why pytest ships `monkeypatch.chdir`, `monkeypatch.setenv` and `tmp_path`: each one undoes itself after the test. Two things change in Rust. The default is parallel threads, so a leak becomes a race rather than an ordering bug, and "it fails when run after test X" turns into "it fails some of the time". And there is no monkeypatch to reach for, because changing the environment while other threads run is unsound rather than just untidy. Module-level globals map to statics: per process in both. `pytest-xdist` workers are separate processes, which is the nextest end of the table. pytest's `-s` is `--nocapture`, and `-k` is the name filter.
- **ABAP.** ABAP Unit runs a test class's methods one after another in one session, so `CLASS-DATA` set in one test method is still set in the next. That is a Rust static inside one test binary. `SETUP` and `TEARDOWN` are the discipline that keeps it tidy. What ABAP developers rarely meet is the parallel half: two test methods do not run at the same time, so the leak shows up as an order dependency and never as a flake. In Rust, write tests as if another test is running beside them, because one is.
- **Java.** JUnit runs a module's tests in one JVM, so static fields are shared exactly as statics are here. Maven Surefire's `forkCount` and `reuseForks` are the knobs for more processes, which is cargo-nextest's territory. JUnit 5 runs tests in parallel only when you switch it on. `cargo test` has parallel on by default.
- **Go.** `go test` builds one test binary per package and runs a package's tests one after another unless they call `t.Parallel()`. Go reached the same conclusion about the environment that `set_var` did: `t.Setenv` refuses to run in a parallel test, and `t.Chdir` exists so a test can move directory and have it undone afterwards.

---

## The verified output

<!-- output:how_cargo_test_runs -->
*Verified output of [`how_cargo_test_runs.rs`](examples/how_cargo_test_runs.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One test binary per target
   src/lib.rs and its modules      1 binary
   src/main.rs, each src/bin/*.rs  1 binary each
   each file directly in tests/    1 binary each
   tests/<dir>/main.rs             1 binary
   tests/<dir>/mod.rs, no main.rs  none: a helper module
   doc tests, edition 2024         1 merged binary, a process per test
   cargo runs these binaries one after another. Inside each one,
   the harness runs the tests as threads of a single process.

2. Threads of one process share its working directory
   one thread called set_current_dir; the main thread moved too: true
   A relative path in any other test now resolves against that
   directory, and whether it does depends on which thread got there first.

3. A second process has its own
   the child said "moved"; this process's directory is unchanged: true

4. A static is shared by one binary's tests, and by nobody else
   two threads each opened one: this process counts 2
   a new process opened one:    it counts 1
   A static Mutex serialises the tests in tests/api.rs against each
   other and does nothing at all about tests/cli.rs.

5. The environment is copied into a child, never shared back
   Command::env gave the child HOW_CARGO_TEST_RUNS_MODE=test
   this process still has it: false
   Inside one binary there is no safe way to vary it per test:
   std::env::set_var is `unsafe` in edition 2024, and its docs say the
   only sound option in a multi-threaded program (outside Windows) is
   not to call it. A test binary is always multi-threaded.

6. A failed test is a panic, and a failed test binary is exit status 101
   a process whose main thread panicked exited with Some(101)
   libtest exits with the same 101 when any test failed, and cargo
   passes that status on. It also stops at the first failing binary:
   the targets after it are not run unless you pass --no-fail-fast.
```
<!-- /output -->

## Practice

**The test that passes alone.** A function reads `prices.txt` from the current directory. Two tests each create a fixture directory with a different price list, move into it with `set_current_dir`, and check the price of an apple. Each passes alone; together they fail some of the time.

First, make the failure happen every time. Run the two tests as two threads and use two `Barrier`s to force the order: A moves, then B moves, then A reads. Then change the function so both tests pass under that same order, without removing the `set_current_dir` calls from the replay. Say in one sentence which part of the program is now allowed to decide where the file is.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:how_cargo_test_runs_kata -->
*[`how_cargo_test_runs_kata.rs`](examples/how_cargo_test_runs_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the test that passes alone.
//!
//!   rustc --edition 2024 how_cargo_test_runs_kata.rs -o /tmp/hctrk && /tmp/hctrk
//!   rustc --edition 2024 --test how_cargo_test_runs_kata.rs -o /tmp/hctrkt && /tmp/hctrkt
//!
//! `main` replays the two tests on two threads with the interleaving forced by
//! barriers, so the race that only sometimes happens under `cargo test` happens
//! every time here.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Barrier};
use std::thread;

/// Before: the price list is wherever the process happens to be standing.
fn price_before(item: &str) -> Option<u32> {
    lookup(&fs::read_to_string("prices.txt").ok()?, item)
}

/// After: the caller says where the price list is.
fn price_after(root: &Path, item: &str) -> Option<u32> {
    lookup(&fs::read_to_string(root.join("prices.txt")).ok()?, item)
}

fn lookup(text: &str, item: &str) -> Option<u32> {
    text.lines()
        .filter_map(|line| line.split_once('='))
        .find(|(name, _)| *name == item)
        .and_then(|(_, cents)| cents.parse().ok())
}

/// A fixture directory holding one `prices.txt`, removed when dropped.
struct Fixture(PathBuf);

impl Fixture {
    fn new(name: &str, prices: &str) -> Fixture {
        let dir = env::temp_dir().join(format!("hctr-kata-{}-{name}", std::process::id()));
        fs::create_dir_all(&dir).expect("fixture dir");
        fs::write(dir.join("prices.txt"), prices).expect("fixture file");
        Fixture(dir)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The fixed pair: nothing process-wide is touched, so they can run in any
    // order, on any thread, alongside anything.
    #[test]
    fn apples_cost_three_in_the_spring_list() {
        let spring = Fixture::new("spring", "apple=3\npear=4\n");
        assert_eq!(price_after(&spring.0, "apple"), Some(3));
    }

    #[test]
    fn apples_cost_five_in_the_winter_list() {
        let winter = Fixture::new("winter", "apple=5\npear=6\n");
        assert_eq!(price_after(&winter.0, "apple"), Some(5));
    }
}

/// Runs "test A" and "test B" on two threads: A moves into its directory, then
/// B moves into its own, then A reads. `read` is the only thing that varies.
fn forced_race(read: fn(&Path) -> Option<u32>) -> (Option<u32>, Option<u32>) {
    let spring = Arc::new(Fixture::new("race-spring", "apple=3\n"));
    let winter = Arc::new(Fixture::new("race-winter", "apple=5\n"));
    let a_moved = Arc::new(Barrier::new(2));
    let b_moved = Arc::new(Barrier::new(2));
    let original = env::current_dir().expect("cwd");

    let a = {
        let (dir, a_moved, b_moved) = (Arc::clone(&spring), Arc::clone(&a_moved), Arc::clone(&b_moved));
        thread::spawn(move || {
            env::set_current_dir(&dir.0).expect("chdir");
            a_moved.wait();
            b_moved.wait();
            read(&dir.0)
        })
    };
    let b = {
        let (dir, a_moved, b_moved) = (Arc::clone(&winter), Arc::clone(&a_moved), Arc::clone(&b_moved));
        thread::spawn(move || {
            a_moved.wait();
            env::set_current_dir(&dir.0).expect("chdir");
            b_moved.wait();
            read(&dir.0)
        })
    };
    let result = (a.join().expect("a"), b.join().expect("b"));
    env::set_current_dir(original).expect("restore");
    result
}

fn main() {
    println!("1. Each test passes on its own");
    let spring = Fixture::new("alone", "apple=3\n");
    let original = env::current_dir().expect("cwd");
    env::set_current_dir(&spring.0).expect("chdir");
    println!("   test A alone, before the fix: apple = {:?}", price_before("apple"));
    env::set_current_dir(original).expect("restore");
    drop(spring);

    println!();
    println!("2. Together, the second set_current_dir wins for both threads");
    let (a, b) = forced_race(|_| price_before("apple"));
    println!("   test A expected Some(3) and read {a:?}");
    println!("   test B expected Some(5) and read {b:?}");
    println!("   Under cargo test this interleaving happens only sometimes, which is");
    println!("   why the failure looks like a flake rather than a bug.");

    println!();
    println!("3. With the directory passed in, the same interleaving is harmless");
    let (a, b) = forced_race(|root| price_after(root, "apple"));
    println!("   test A expected Some(3) and read {a:?}");
    println!("   test B expected Some(5) and read {b:?}");
    println!("   The threads still call set_current_dir. Nothing reads it any more.");

    println!();
    println!("4. The rule the fix follows");
    println!("   Only main() decides where the files are. It resolves the path");
    println!("   once and hands it down, so a test can hand down a different one.");
}
```
<!-- /source -->

<!-- output:how_cargo_test_runs_kata -->
*Verified output of [`how_cargo_test_runs_kata.rs`](examples/how_cargo_test_runs_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Each test passes on its own
   test A alone, before the fix: apple = Some(3)

2. Together, the second set_current_dir wins for both threads
   test A expected Some(3) and read Some(5)
   test B expected Some(5) and read Some(5)
   Under cargo test this interleaving happens only sometimes, which is
   why the failure looks like a flake rather than a bug.

3. With the directory passed in, the same interleaving is harmless
   test A expected Some(3) and read Some(3)
   test B expected Some(5) and read Some(5)
   The threads still call set_current_dir. Nothing reads it any more.

4. The rule the fix follows
   Only main() decides where the files are. It resolves the path
   once and hands it down, so a test can hand down a different one.
```
<!-- /output -->

</details>

## See also

- [Where a test goes](../where_a_test_goes/README.md): unit, integration and doc tests, and which of them can see private items
- [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md): the pass-it-in fix, for the filesystem
- [cargo-nextest](../../05_Tooling/nextest/README.md): one process per test, and the doc tests it does not run
- [A harness of your own](../a_harness_of_your_own/README.md): what libtest is, and what `harness = false` hands back to you
- [Other kinds of test](../other_kinds_of_test/README.md): where property, snapshot and compile-fail tests fit in the run
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md): the mechanism a failing test is built on
- [Standard error, and exit status](../../02_Errors/stderr_and_exit_status/README.md): what 101 means to the shell that ran it
- [`cargo test` ↗](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [the test harness ↗](https://doc.rust-lang.org/rustc/tests/index.html): the reference
- [Testing: courses and links](../resources/README.md): the Advanced Rust testing course, whose "testing system" interlude covers this page's ground

## Po polsku

`cargo test` to nie jeden program, tylko kilka: osobny plik wykonywalny testów dla biblioteki, dla każdego binarium i dla każdego pliku leżącego bezpośrednio w `tests/`, uruchamiane kolejno, jeden po drugim. Testy **wewnątrz** jednego takiego pliku to wątki jednego procesu, a proces ma jeden katalog roboczy, jedno środowisko i jeden egzemplarz każdej zmiennej `static`. Stąd klasyczny „niestabilny test” (*flaky test*): test, który przechodzi uruchomiony osobno i pada w komplecie, bo inny test zawołał `set_current_dir` i przeniósł cały proces. W pięciu kolejnych przebiegach tej samej pary testów wyszło tu dwa razy `ok` i trzy razy `FAILED`. `--test-threads=1` niczego nie naprawia, najwyżej ukrywa. Testy dokumentacyjne są wyjątkiem: w edycji 2024 kompilują się razem do jednego pliku, ale każdy nadal biegnie we własnym procesie.

Lekarstwo nie polega na pilnowaniu kolejności, tylko na tym, żeby nie było czego pilnować. Ścieżkę, adres serwera czy konfigurację przekazuje się jako parametr, a o prawdziwej wartości decyduje tylko `main()`. Każdy test dostaje własny katalog tymczasowy, własny port albo własną bazę. Gdy kodu nie da się zmienić, `cargo nextest` uruchamia każdy test w osobnym procesie. Zmiennych środowiskowych nie zmienia się w testach wcale: `std::env::set_var` jest w edycji 2024 funkcją `unsafe`, a jej dokumentacja mówi wprost, że poza Windowsem w programie wielowątkowym jedynym bezpiecznym wyjściem jest jej nie wołać.

Trzy rzeczy praktyczne. Wydruk z testu, który przechodzi, jest połykany; `cargo test -- --show-output` pokazuje go z podpisem testu. Panika w jednym teście nie zatrzymuje pozostałych, ale `abort()` (także przepełnienie stosu) zabija cały plik wykonywalny bez żadnego podsumowania. A każdy plik w `tests/` to osobne linkowanie: dwadzieścia plików budowało się tu około trzy razy dłużej niż te same testy jako moduły jednego `tests/it/main.rs`. Nieudany przebieg kończy się statusem 101, tym samym, którym kończy się każdy program w Ruście po panice w wątku głównym.

**Szukaj po polsku:** uruchamianie testów w Ruście · niestabilne testy · testy równoległe · katalog roboczy procesu · `cargo test test-threads` · `rust cargo test show-output nocapture` · `rust integration tests compile time single binary`
