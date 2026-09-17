# Temporary directories in tests

**Level:** 201 → 301 · deep dive

**One line:** A test that writes to a fixed path cannot run beside another test that writes there too. The usual fix is a directory per test that removes itself when dropped, and it removes itself the moment you stop holding the handle.

```rust
use std::fs;
use tempfile::TempDir;

#[test]
fn reads_the_editor_from_config() {
    let dir = TempDir::new().unwrap();                // a new, empty directory
    let config = dir.path().join("config.txt");
    fs::write(&config, "editor = hx\n").unwrap();
    assert_eq!(default_editor(&config).unwrap().as_deref(), Some("hx"));
}                                                     // `dir` dropped: directory removed
```

That test compiles and passes against [`tempfile` ↗](https://docs.rs/tempfile) 3.27.0, added as a `[dev-dependencies]` entry. The example on this page uses a 30-line stand-in with the same shape (`TempDir::new()`, `.path()`, removal in `Drop`), so its recorded output needs no crate.

## Why a fixed path fails

`cargo test` runs the tests in a binary [as threads of one process, in parallel](../../28_Testing/how_cargo_test_runs/README.md). Two tests that write `config.txt` into the same directory race each other. Section 1 of the output forces the bad order and gets it every time: test A writes `vim`, test B writes `hx`, and A reads back `hx`. Under `cargo test` the same thing happens only some of the time.

A fixed path fails a second way even without a second test: a run that crashed halfway leaves the file behind, and the next run starts from it. A test can pass because of a leftover file that no current code writes.

A directory per test fixes both (section 2). Nothing is shared, so the order does not matter, and there is nothing left over for the next run to read.

## The function has to take the path

A per-test directory only helps if the code under test can be told about it:

```rust
use std::io;
use std::path::Path;

// Hard to test: the path is decided inside, and it is relative to the working directory.
fn default_editor_fixed() -> io::Result<Option<String>> {
    default_editor(Path::new("config.txt"))
}

// Testable: the caller decides.
fn default_editor(config: &Path) -> io::Result<Option<String>> {
    todo!("open {}, find the `editor = ` line", config.display())
}
```

Nothing in the first signature says it touches the filesystem, and the only way a test can steer it is `set_current_dir`, which [moves every other test in the binary too](../../28_Testing/how_cargo_test_runs/README.md#tests-in-one-binary-share-a-process). Hard-code the real path as close to `main` as you can, and pass it down from there. `main` becomes a thin layer that is not worth testing, and everything under it takes a path a test can choose.

When several files find each other by relative path, such as a workspace list naming member directories, the same rule applies one level up. Take a **root directory** and build every path from it. A test creates the whole layout inside one `TempDir` and passes that as the root (section 6).

## The directory lives exactly as long as its owner

`TempDir`'s `Drop` removes the directory and everything in it. That is the feature, and it is also the trap:

```rust
let path = TempDir::new()?.path().to_owned();   // the directory is already gone
fs::write(path.join("config.txt"), "…")?;       // Err(NotFound)
```

`TempDir::new()?` is a temporary, and a temporary is dropped at the end of its statement. `.to_owned()` copied the path out first, so `path` is a correct name for a directory that no longer exists. Section 4 of the output shows both lines, and the same two lines against the real crate gave `exists() == false` and `NotFound` too.

The error reads like a permissions or filesystem problem, and it comes from code that created the directory one line earlier. It is neither. The ownership rules did what [they always do](../../18_Ownership/scope_is_about_names/README.md), in a place where you were thinking about files rather than about ownership.

Two more forms of the same trap:

| You write | What happens to the directory |
|---|---|
| `let dir = TempDir::new()?;` | kept until the end of the block |
| `let _dir = TempDir::new()?;` | kept: `_dir` is a name, just one the compiler will not warn about |
| `let _ = TempDir::new()?;` | **removed at once**: `_` binds nothing, so the value is a dropped temporary |
| a helper that makes a `TempDir` and returns only the path | removed when the helper returns (the practice below) |

The compiler catches only the borrowed version. Returning `dir.path()` from a function that owns `dir` is error E0515. A `PathBuf` is an independent value, and nothing ties it to the `TempDir` whose `Drop` deletes what it names.

## A failing test still cleans up

A failed `assert!` panics, the panic unwinds the test's thread, and unwinding runs `Drop`, so the directory is removed on failure too (section 5). Two things skip it: a build with `panic = "abort"`, and [`std::process::exit` ↗](https://doc.rust-lang.org/std/process/fn.exit.html), whose docs say no destructors run. Anything left behind stays in the system temp directory. [`std::env::temp_dir` ↗](https://doc.rust-lang.org/std/env/fn.temp_dir.html) makes no promise that it is ever cleaned, so do not count on the OS.

## Sometimes the file is not the point

If the logic is *read lines and find a setting*, it does not need a file at all:

```rust
use std::io::{self, BufRead};

fn default_editor_from(reader: impl BufRead) -> io::Result<Option<String>> {
    for line in reader.lines() {
        if let Some(value) = line?.strip_prefix("editor = ") {
            return Ok(Some(value.to_string()));
        }
    }
    Ok(None)
}

// a test hands it bytes:  default_editor_from(&b"editor = hx\n"[..])
```

That is [`Read` and `Write`](../../12_Traits/read_and_write/README.md) applied to a test, and it removes most of the temp directories a suite would need. It does not remove all of them. Something still opens the file, and something still decides what a missing file means. That code has branches, and at least one test has to give it a real path.

## Which `tempfile` type

| Type | What you get | Removed |
|---|---|---|
| [`TempDir` ↗](https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html) | a directory, with a path | when dropped |
| [`NamedTempFile` ↗](https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html) | one file, with a path to hand to the code under test | when dropped |
| [`tempfile()` ↗](https://docs.rs/tempfile/latest/tempfile/fn.tempfile.html) | an open `File` with no path to hand to anyone | by the OS, when the last handle closes |

The last row is the one that survives `process::exit`: its docs say it does not rely on Rust destructors. It is useless for code that takes a `&Path`, and a good fit for code that takes `impl Read + Write + Seek`.

Why not write a stand-in like this page's example in real code: the stand-in names each directory with the process id and a counter, which anyone can predict. `temp_dir`'s own docs warn that the temp directory may be shared with other users and that predictable names are a security problem. `tempfile` picks random names and creates them safely. Use it.

## What to assert

Assert the behaviour: the file now contains the record, or the function returned `hx`. The path itself differs on every run and every machine, so a test that compares paths either fails everywhere else or ends up comparing a path with itself.

## If you are coming from another language

- **Python.** pytest's `tmp_path` fixture is this page's `TempDir`, injected by name. `tempfile.TemporaryDirectory()` used as a context manager is the same thing written by hand: the directory is removed when the `with` block ends, much as it is when `dir` goes out of scope here. The trap carries over too: on CPython 3.14.7, `tempfile.TemporaryDirectory().name` is the name of a directory that no longer exists. Reference counting finalises the unbound object at once, just as Rust drops the temporary. The difference is what guarantees it. In CPython the timing is an implementation detail, and a garbage-collected Python such as PyPy may keep the directory a while longer. In Rust the end of the statement is the language rule. What does carry over is the refactor: pytest users who swap `open("config.txt")` for a function taking a `Path` or a file object are making the same move as this page.
- **ABAP.** Application-server files through `OPEN DATASET` are the usual ABAP filesystem, and an ABAP Unit test that writes one has this page's problem: the path is shared by every run on that server, including a colleague's run at the same moment. The ABAP habit is to avoid the file in the test and inject the data instead, usually as a table or a test double for the reader class. That is the `impl BufRead` section. Rust makes the other option cheap too: a private directory per test, removed automatically, which ABAP has no built-in equivalent for.
- **Java.** JUnit 5's `@TempDir Path dir` parameter is `TempDir` handed in by the framework, and removed after the test. There is no early-drop trap, because the framework owns it for the test's whole duration. That is what the fixed helper in the practice below builds by hand.
- **Go.** `t.TempDir()` creates a directory and registers its removal with `t.Cleanup`, so it lasts until the test ends, however it is called. Go ties the lifetime to the test; Rust ties it to a variable, which is more flexible and is the reason this page has a trap section.

---

## The verified output

<!-- output:temp_dirs_in_tests -->
*Verified output of [`temp_dirs_in_tests.rs`](examples/temp_dirs_in_tests.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Two tests, one fixed path
   test A wrote vim and read Some("hx")
   test B wrote hx  and read Some("hx")
   The second write replaced the first before A read it back.

2. A directory per test
   test A wrote vim and read Some("vim")
   test B wrote hx  and read Some("hx")
   Same threads, same interleaving. Nothing is shared, so nothing races.

3. The directory lives exactly as long as its owner
   while `dir` is alive, the directory exists
   after drop(dir), it is gone (and the file in it with it)

4. The trap: the handle was a temporary
   let path = TempDir::new()?.path().to_owned();
   the directory is gone before the next line runs
   writing a file into it: Err(NotFound)
   let _ = TempDir::new()?;     directories alive now: 0
   let _dir = TempDir::new()?;  directories alive now: 1

5. A failing test still removes its directory
   the test panicked: true; its directory is gone
   Unwinding runs Drop. panic = "abort" and process::exit do not.

6. Several files: paths relative to a root, never to the working directory
   workspace_members(root) = ["shop-api", "shop-cli"]
   The layout is built inside the root, so relative paths between the
   files still hold, and set_current_dir is never called.

7. Or no file at all
   default_editor_from(bytes)       = Some("hx")
   default_editor_from(empty bytes) = None
   Opening the file, and the error when it is missing, still need one
   test that uses a real path.

   directories still alive at the end: 0
```
<!-- /output -->

## Practice

**The fixture helper that returned a path to nowhere.** You are testing `count_errors(log: &Path) -> io::Result<usize>`. To avoid repeating setup, you write `fn log_fixture(contents: &str) -> PathBuf`: make a `TempDir`, write `app.log` into it, return the file's path. It compiles. Every test that uses it fails with `NotFound`.

Explain why in one sentence, and why the compiler said nothing. Then change the helper so the tests pass and the directory is still removed when each test ends.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:temp_dirs_in_tests_kata -->
*[`temp_dirs_in_tests_kata.rs`](examples/temp_dirs_in_tests_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the fixture helper that returned a path to nowhere.
//!
//!   rustc --edition 2024 temp_dirs_in_tests_kata.rs -o /tmp/tditk && /tmp/tditk
//!   rustc --edition 2024 --test temp_dirs_in_tests_kata.rs -o /tmp/tditkt && /tmp/tditkt

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);

/// The same stand-in for `tempfile::TempDir` as the lesson's example.
struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new() -> io::Result<TempDir> {
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        let path = env::temp_dir().join(format!("temp_dirs_in_tests_kata-{}-{n}", std::process::id()));
        fs::create_dir(&path)?;
        Ok(TempDir { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// The code under test: how many lines of a log file mention an error.
fn count_errors(log: &Path) -> io::Result<usize> {
    Ok(fs::read_to_string(log)?.lines().filter(|l| l.contains("ERROR")).count())
}

/// Broken: `dir` is dropped at the closing brace, and takes the file with it.
/// It compiles, because a `PathBuf` borrows nothing from the directory it names.
fn log_fixture_broken(contents: &str) -> PathBuf {
    let dir = TempDir::new().expect("dir");
    let log = dir.path().join("app.log");
    fs::write(&log, contents).expect("write");
    log
}

/// Fixed: the caller receives the owner, so the directory lives as long as they keep it.
struct LogFixture {
    _dir: TempDir, // never read; held only so that dropping the fixture removes it
    log: PathBuf,
}

fn log_fixture(contents: &str) -> LogFixture {
    let dir = TempDir::new().expect("dir");
    let log = dir.path().join("app.log");
    fs::write(&log, contents).expect("write");
    LogFixture { _dir: dir, log }
}

const LOG: &str = "INFO start\nERROR disk full\nINFO retry\nERROR disk full\n";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_two_errors() {
        let fixture = log_fixture(LOG);
        assert_eq!(count_errors(&fixture.log).unwrap(), 2);
    }

    #[test]
    fn an_empty_log_has_none() {
        let fixture = log_fixture("");
        assert_eq!(count_errors(&fixture.log).unwrap(), 0);
    }
}

fn main() {
    println!("1. The broken helper");
    let log = log_fixture_broken(LOG);
    println!("   log_fixture_broken(..) returned a path; the file exists: {}", log.exists());
    match count_errors(&log) {
        Ok(n) => println!("   count_errors = {n}"),
        Err(e) => println!("   count_errors = Err({:?})", e.kind()),
    }
    println!("   NotFound, from code that wrote the file one line earlier. It is not");
    println!("   a permissions problem: the directory was dropped with the helper's");
    println!("   stack frame, before the test got its path.");

    println!();
    println!("2. The fixed helper returns the owner along with the path");
    let fixture = log_fixture(LOG);
    println!("   the file exists: {}", fixture.log.exists());
    println!("   count_errors = {:?}", count_errors(&fixture.log).map_err(|e| e.kind()));
    let log = fixture.log.clone();
    drop(fixture);
    println!("   after the fixture is dropped, the file exists: {}", log.exists());

    println!();
    println!("3. Why the compiler did not stop the broken one");
    println!("   A borrow would have been caught: returning dir.path() is a &Path");
    println!("   into a local, and that is error E0515. to_owned() or join() makes");
    println!("   an independent PathBuf, which is only a name. Nothing ties a name");
    println!("   to the value whose Drop deletes the thing it names.");
}
```
<!-- /source -->

<!-- output:temp_dirs_in_tests_kata -->
*Verified output of [`temp_dirs_in_tests_kata.rs`](examples/temp_dirs_in_tests_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The broken helper
   log_fixture_broken(..) returned a path; the file exists: false
   count_errors = Err(NotFound)
   NotFound, from code that wrote the file one line earlier. It is not
   a permissions problem: the directory was dropped with the helper's
   stack frame, before the test got its path.

2. The fixed helper returns the owner along with the path
   the file exists: true
   count_errors = Ok(2)
   after the fixture is dropped, the file exists: false

3. Why the compiler did not stop the broken one
   A borrow would have been caught: returning dir.path() is a &Path
   into a local, and that is error E0515. to_owned() or join() makes
   an independent PathBuf, which is only a name. Nothing ties a name
   to the value whose Drop deletes the thing it names.
```
<!-- /output -->

</details>

## See also

- [How `cargo test` runs your tests](../../28_Testing/how_cargo_test_runs/README.md): why the tests are threads of one process, and what else they share
- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md): the schedule a value actually dies on, temporaries included
- [Drop and RAII](../../12_Traits/drop_and_raii/README.md): the trait that does the removing
- [`Read` and `Write`](../../12_Traits/read_and_write/README.md): taking `impl Read` so a test can hand over bytes instead of a file
- [`Path` and `PathBuf`](../path_and_pathbuf/README.md): the owned name that outlives the directory
- [Testing a command](../../03_Command_Line/testing_a_command/README.md): the tests that need a filesystem of their own
- [Testing: courses and links](../../28_Testing/resources/README.md): the Advanced Rust testing course's filesystem-isolation section, which this page follows

## Po polsku

`cargo test` uruchamia testy jednego pliku wykonywalnego równolegle, w wątkach jednego procesu, więc stała ścieżka w rodzaju `/tmp/config.txt` to wyścig: drugi test nadpisuje plik, zanim pierwszy go odczyta. Plik pozostały po przerwanym przebiegu potrafi też sprawić, że test przechodzi z niewłaściwego powodu. Lekarstwo ma dwie części i obie są potrzebne. Funkcja musi **przyjmować ścieżkę** jako parametr, zamiast wymyślać ją w środku (o prawdziwej ścieżce decyduje `main()`), a każdy test dostaje własny katalog tymczasowy, `tempfile::TempDir`, który kasuje się sam w `Drop`, także wtedy, gdy test padnie na asercji. Nie kasuje się tylko przy `panic = "abort"` i po `process::exit`, a na sprzątanie katalogu tymczasowego przez system operacyjny liczyć nie warto.

Ostra krawędź jest jedna, ale w kilku przebraniach: `TempDir::new()?.path().to_owned()` zwraca ścieżkę do katalogu, którego już nie ma, bo wartość tymczasowa żyje tylko do średnika. `let _ = TempDir::new()?;` nic nie utrzymuje, bo `_` nie jest nazwą, podczas gdy `let _dir = …` wiąże wartość do końca bloku. Tak samo zachowuje się funkcja pomocnicza, która tworzy katalog i zwraca samą ścieżkę. Kompilator milczy, bo `PathBuf` to niezależna wartość, tylko nazwa, niezwiązana z właścicielem katalogu. Wynikiem jest `NotFound` z kodu, który linijkę wcześniej ten plik zapisał. Wygląda to na kłopot z uprawnieniami, a są to po prostu reguły własności. Gdy logika nie potrzebuje pliku, lepiej przyjąć `impl BufRead` i podać w teście bajty; jeden test z prawdziwą ścieżką i tak zostaje, dla otwierania pliku i obsługi jego braku.

**Szukaj po polsku:** testy równoległe w Ruście · katalog tymczasowy w testach · czas życia wartości tymczasowej · `rust tempfile TempDir` · `rust temporary value dropped while borrowed` · `rust test fixture tempdir`
