# From one `.rs` file to a Cargo project: `cargo init`

**Level:** 101 · for newcomers

**One line:** Put `ok.rs` in a folder named `ok`, run `cargo init` there, and Cargo takes the file as the program. It does that only because the file has the package's name. Any other name gets a hello-world `src/main.rs` written beside it, with the same two lines of output, and `cargo run` prints `Hello, world!`.

Every transcript on this page is a real run of Cargo 1.98.0 in the `rust:1.98-slim` container, in `/tmp`. That is the folder *Rust in Action* uses, so the paths match the book. Differences on macOS are named where they occur.

## The four steps

The file, compiled with `rustc` so far:

```rust title="ok.rs"
fn main() {
    println!("OK");
}
```

Move it into a folder of its own and make that folder a package:

```sh
mkdir ok && mv ok.rs ok && cd ok && cargo init
```

```text title="Real output — cargo init, then what it wrote"
    Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ ls -A
.git
.gitignore
Cargo.toml
ok.rs
$ cat Cargo.toml
[package]
name = "ok"
version = "0.1.0"
edition = "2024"

[dependencies]

[[bin]]
name = "ok"
path = "ok.rs"
$ cat .gitignore
/target
```

`ok.rs` stays where it is, and there is no `src/`.

| What was written | Why |
|---|---|
| `name = "ok"` | the folder's name |
| `[[bin]]` with `path = "ok.rs"` | the program is not at `src/main.rs`, where Cargo looks by default, so the manifest says where it is |
| `edition = "2024"` | `cargo init` writes the newest edition. Plain `rustc` compiled the same file as [edition 2015](../../15_First_Programs/rustc_without_cargo/README.md#the-trap-rustcs-default-edition-is-2015) |
| `.git/` and a `.gitignore` holding `/target` | a new repository, *unless the folder is already inside one*. On this Mac, `cargo init` under a folder that had had `git init` wrote only `Cargo.toml` |

Run `cargo init` a second time and it refuses: ``error: `cargo init` cannot be run on existing Cargo packages``.

## `cargo run`

```text title="Real output — cargo run, twice"
$ cargo run
   Compiling ok v0.1.0 (/tmp/ok)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/ok`
OK
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/ok`
OK
```

| Line | What it says |
|---|---|
| `Compiling ok v0.1.0 (/tmp/ok)` | `rustc` ran. The second run has no such line: nothing had changed, so nothing was compiled |
| ``Finished `dev` profile [unoptimized + debuginfo]`` | the [profile ↗](https://doc.rust-lang.org/cargo/reference/profiles.html): no optimization, debug info included. `--release` switches it |
| ``Running `target/debug/ok` `` | the executable Cargo runs, and where it is |
| `OK` | your program |

**Cargo's lines go to stderr, your program's to stdout**, so a redirect or `-q` leaves only yours:

```text title="Real output"
$ cargo run 2>/dev/null
OK
$ cargo run -q
OK
```

**The executable is `target/debug/ok`**, not the `./ok` that `rustc ok.rs` left beside the source. `cargo build --release` writes `target/release/ok`.

```text title="Real output"
$ ls target/debug
build
deps
examples
incremental
ok
ok.d
```

`deps/` is where `rustc` actually writes it, under a hashed name. [The next page](what_cargo_passes_rustc/README.md) shows why.

## Trap 1: the old `ok` executable is in the way

If you ran `rustc ok.rs` first, which is how most people arrive at this page, the folder already holds a file named `ok`:

```text title="Real output — the same steps one at a time, run as a bash script"
$ rustc ok.rs
$ ls -l
-rwxr-xr-x 1 root root 4508768 Sep 17 00:47 ok
-rw-r--r-- 1 root root      34 Sep 17 00:47 ok.rs
$ mkdir ok
mkdir: cannot create directory 'ok': File exists
$ mv ok.rs ok
$ cd ok
/t.sh: line 8: cd: ok: Not a directory
$ ls -l
-rw-r--r-- 1 root root 34 Sep 17 00:47 ok
$ cat ok
fn main() {
    println!("OK");
}
```

`mkdir` fails. Then `mv ok.rs ok` renames the source *over* the executable, because `ok` is an existing file and not a folder. The 4.5 MB program is gone, and your source is now a 34-byte file with no extension. The `cd` error is the first thing that looks wrong, and by then both of those have already happened. (macOS words the first error `mkdir: ok: File exists`.)

- **Joined with `&&`**, as in the four steps above, the line stops at `mkdir` and nothing is lost.
- **Recover** with `mv ok ok.rs`.
- **Avoid it** with `rm ok` before `mkdir ok`.
- **Windows does not have this trap:** the executable is `ok.exe`, which `rustc --print file-names --target x86_64-pc-windows-msvc ok.rs` prints without building anything.

## Trap 2: the file has to have the package's name

`cargo init` adopts a file only if it is at one of a few fixed paths. It found each of these: `ok.rs`, `src/ok.rs`, `main.rs`, `src/main.rs`. Here `ok` is the package name, and that comes from the folder. Give the folder any other name and the file is not found:

```text title="Real output — ok.rs in a folder named ok_project"
$ mkdir ok_project && mv ok.rs ok_project && cd ok_project && cargo init
    Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ ls -A
.git
.gitignore
Cargo.toml
ok.rs
src
$ cat src/main.rs
fn main() {
    println!("Hello, world!");
}
$ cargo run
   Compiling ok_project v0.1.0 (/tmp/ok_project)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/ok_project`
Hello, world!
```

`cargo init` prints the same two lines as in the success case. `ok.rs` sits untouched beside a new `src/main.rs`, and the manifest has no `[[bin]]` table. Nothing reports that your file was skipped. The only sign is `Hello, world!` where you expected your own output.

| Fix | What you get |
|---|---|
| name the folder after the file: `mkdir ok` for `ok.rs` | `[[bin]]` with `path = "ok.rs"` |
| `cargo init --name ok`, in a folder with any name | the same: the lookup uses the package name, not the folder's |
| move the file to `src/main.rs` *before* `cargo init` | no `[[bin]]` table, the layout `cargo new` makes |

## Trap 3: fixing it with `mv` after a build runs the old program

The obvious repair for trap 2 is to move your file over the hello-world. But if you have already run `cargo run` once, it does not work:

```text title="Real output — ok_project from trap 2, after one cargo run"
$ cargo run -q
Hello, world!
$ mv ok.rs src/main.rs
$ cargo run -v 2>&1 | grep -E "Fresh|Dirty|Compiling|^[A-Z][a-z]+,|^OK"
       Fresh ok_project v0.1.0 (/tmp/ok_project)
Hello, world!
$ touch src/main.rs
$ cargo run -v 2>&1 | grep -E "Fresh|Dirty|Compiling|^Hello|^OK"
       Dirty ok_project v0.1.0 (/tmp/ok_project): the file `src/main.rs` has changed (1789606883.956366506s, 90000005ns after last build at 1789606883.866366501s)
   Compiling ok_project v0.1.0 (/tmp/ok_project)
OK
```

Cargo decides whether to rebuild your crate by comparing file **modification times** with the time of the last build. `mv` keeps a file's modification time, and `ok.rs` was written before that build, so the new `src/main.rs` looks older than the executable. Cargo reports `Fresh` and runs the hello-world it built earlier. `cp ok.rs src/main.rs` gives the copy a new time, and it rebuilt and printed `OK`. So does a `touch` after the `mv`.

## Which layout to keep

| | `ok.rs` beside `Cargo.toml` | `src/main.rs` |
|---|---|---|
| manifest | needs `[[bin]]` with `path = "ok.rs"` | no `[[bin]]` table |
| `mod greet;` looks for | `greet.rs` beside `Cargo.toml` | `src/greet.rs` |
| matches `cargo new`, tutorials and IDE templates | no | yes |

Both build. A `mod greet;` in a top-level `ok.rs` found a top-level `greet.rs` and printed `OK`. With the top-level layout, though, every module lands beside `Cargo.toml`, `Cargo.lock` and `target/`. For a file that is going to grow, `src/main.rs` is the layout everything else expects.

## Watching Cargo drive `rustc`: `-v`

`cargo run -v` prints the `rustc` command, but only when something gets compiled. Straight after a build it prints `Fresh` in its place:

```text title="Real output"
$ cargo run -v
       Fresh ok v0.1.0 (/tmp/ok)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/ok`
OK
```

Three ways to make the next build compile:

| Command | What it throws away |
|---|---|
| `rm -rf target/` | every build product, dependencies included |
| `cargo clean` | the same folder, and it reports how much: `Removed 23 files, 8.7MiB total` |
| `touch ok.rs` | nothing. Cargo rebuilds your crate and says why: ``Dirty ok v0.1.0 (/tmp/ok): the file `ok.rs` has changed`` |

With a dependency in the build, `touch` is the cheap one. With a path dependency `util` added, a `touch ok.rs` rebuild printed `Fresh util v0.1.0` and recompiled only `ok`.

The command line itself, flag by flag, is on the next page: [What `cargo run -v` shows](what_cargo_passes_rustc/README.md).

## The book's transcript, run on 1.98

*Rust in Action* (McNamara, Manning 2021), chapter 2, walks through these steps with a transcript from an older Cargo. Here is the same thing re-run:

| The book shows | 1.98 prints | Why |
|---|---|---|
| the four steps, from a folder where `rustc ok.rs` has already left an `ok` | `mkdir: cannot create directory 'ok': File exists` | [trap 1](#trap-1-the-old-ok-executable-is-in-the-way): remove the old `ok` first |
| `Finished dev [unoptimized + debuginfo] target(s)` | ``Finished `dev` profile [unoptimized + debuginfo] target(s)`` | wording only |
| `--edition=2018` | `--edition=2024` | `cargo init` writes the newest edition |
| the `rustc` command, one flag per line | one long line | laid out for the page |
| `-C metadata` and `-C extra-filename` with the same hash | two different hashes | [both explained](what_cargo_passes_rustc/README.md#the-flags) |
| `-C incremental=/tmp/target/debug/incremental` | `-C incremental=/tmp/ok/target/debug/incremental` | a slip in the book: `--out-dir` and `-L` on the same line say `/tmp/ok` |
| `-C link-arg=-fuse-ld=lld`, last | absent | not a Cargo default. `RUSTFLAGS="-C link-arg=-fuse-ld=lld" cargo build -v` puts it back, [at the end](what_cargo_passes_rustc/README.md#flags-cargo-did-not-choose) |
| — | two `--check-cfg` flags, and `artifacts,future-incompat` in `--json` | not in the book's line; [what they do](what_cargo_passes_rustc/README.md#the-flags) |
| — | `-C split-debuginfo=unpacked` | macOS only |
| `rm -rf target/` to force a rebuild | works | `cargo clean` and `touch` [do the same](#watching-cargo-drive-rustc-v) |

## If you are coming from another language

**Go.** `go mod init ok` in a folder holding `hello.go`, then `go run .`, printed `OK` (go 1.25.5). A Go package is every `.go` file in the folder, whatever the file is called, so there is no name to get wrong. A Cargo binary target is exactly one root file, and `cargo init` finds it by name. That is why trap 2 exists in Rust and not in Go.

**Python with uv.** `uv init` in a folder holding `hello.py` wrote a `main.py` beside it that prints `Hello from ok!` (uv 0.11.16). That is trap 2 in another toolchain: the tool starts a project with its own entry point and ignores the file you already had.

**C with make.** `make ok` in a folder holding `ok.c` and no Makefile ran the built-in rule `cc ok.c -o ok`. That leaves an `ok` executable beside `ok.c`, as `rustc ok.rs` does, so trap 1 is make's trap too. Trap 3 is make's as well. After `make ok` built a `Hello` program, an older `ok.c` moved over the source got ``make: `ok' is up to date.`` and the old program still printed `Hello`.

## See also

- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — the `rustc ok.rs` step before this page, and the things Cargo does that plain `rustc` does not
- [What `cargo run -v` shows](what_cargo_passes_rustc/README.md) — the `rustc` command line above, flag by flag
- [A throwaway that needs a crate](../scratch_with_a_crate/README.md) — `cargo new` when there is no file yet, then `cargo add`
- [A disposable Rust workspace, six ways](../create_rust_proj/README.md) — `cargo init --vcs none` in a temporary folder, scripted
- [Makefiles](../../20_Compilers/makefiles/README.md) — the `.d` file behind `Dirty … has changed`, and why Cargo compares timestamps as `make` does
- [RustRover setup](../rustrover_setup/README.md) — a `Fresh` build, and the warnings Cargo replays without compiling
- [Books](../../10_Resources/books/README.md#the-friendly-paid-on-ramps) — where *Rust in Action* sits among the others
- [The Cargo Book — `cargo init` ↗](https://doc.rust-lang.org/cargo/commands/cargo-init.html) · [target auto-discovery ↗](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery)

## Po polsku

Przejście z pojedynczego pliku kompilowanego przez `rustc` do projektu Cargo to dwa ruchy: przenosisz plik do osobnego katalogu i uruchamiasz tam `cargo init`. `cargo init` różni się od `cargo new` tym, że nie tworzy nowego katalogu, tylko zamienia bieżący w pakiet (*package*). Jeśli znajdzie w nim twój program, dopisuje go do `Cargo.toml` jako cel binarny (*binary target*): `[[bin]]` z `path = "ok.rs"`.

Haczyk tkwi w słowie „znajdzie”. `cargo init` szuka programu tylko pod kilkoma stałymi nazwami: `main.rs`, `src/main.rs` albo `<nazwa pakietu>.rs`, a nazwa pakietu to domyślnie nazwa katalogu. Plik `ok.rs` w katalogu `ok` zostanie przyjęty. Ten sam plik w katalogu `ok_project` zostanie pominięty bez słowa: obok powstaje `src/main.rs` z „Hello, world!”, komunikat jest identyczny jak przy sukcesie, a `cargo run` wypisuje `Hello, world!` zamiast twojego wyniku. Pomaga `cargo init --name ok` albo katalog nazwany tak jak plik.

Dwie kolejne pułapki są czysto uniksowe. Po wcześniejszym `rustc ok.rs` w katalogu leży już plik wykonywalny `ok`, więc `mkdir ok` się nie uda. Jeśli wpisujesz polecenia po jednym, następne `mv ok.rs ok` nadpisze ten plik twoim kodem źródłowym. Z kolei `mv` zachowuje czas modyfikacji (*modification time*), a Cargo właśnie po nim ocenia, czy trzeba kompilować ponownie. Plik przeniesiony po pierwszej kompilacji wygląda na „stary”, Cargo pisze `Fresh` i uruchamia poprzedni program. Wystarczy `touch` albo `cp` zamiast `mv`.

Plik wykonywalny ląduje w `target/debug/ok`, a nie obok źródła. Komunikaty Cargo idą na stderr, więc `cargo run -q` albo `2>/dev/null` zostawia tylko wyjście programu. Transkrypcje w *Rust in Action* pochodzą ze starszego Cargo: edycja 2018 zamiast 2024, inne brzmienie linii `Finished`, a flaga `-C link-arg=-fuse-ld=lld` pochodziła z konfiguracji autora, nie z Cargo.

**Szukaj po polsku:** projekt Cargo z istniejącego pliku · `cargo init` · `cargo init existing file` · `cargo target auto-discovery` · `cargo run verbose` · `cargo fresh dirty mtime`
