# Running a scratch program: `rustc` alone, `cargo new`, and `src/bin/`

**Level:** 101 · for newcomers

**One line:** A single file needs no project at all — `rustc file.rs -o file && ./file` is the whole toolchain — and the three routes to a running program differ mainly in what you have to clean up afterwards.

Every example in this library is one `.rs` file compiled by one `rustc` command, so this page is also the answer to *"how do I run these?"*. The interesting part is what the shortest route leaves out, because that list is a precise description of what Cargo is for.

---

## 1. `rustc` alone — no project at all

```sh
echo 'fn main() { println!("hi"); }' > /tmp/t.rs
rustc /tmp/t.rs -o /tmp/t && /tmp/t
```

Thirty bytes of source in, one 451 KB native executable out, and it prints `hi`.

| Piece | What it does |
|---|---|
| `fn main()` | the entry point — every Rust *binary* needs exactly one |
| `println!` | the `!` marks a **macro**, not a function; the format string is checked at compile time |
| `-o /tmp/t` | names the output. Without it you get `t` in the current directory |
| `&&` | run it only if compilation succeeded |

`/tmp/t` is a real executable with no runtime attached: copy it to another machine of the same platform, one with no Rust installed at all, and it runs. That is the contrast with `python t.py`, which needs the interpreter present forever.

**Where the 451 KB goes** — none of it is your code. It is the standard library, statically linked, plus the panic/unwind machinery and the symbol names a backtrace would need. It is close to a fixed floor rather than a rate: a five-thousand-line program is not meaningfully bigger. Measured on one machine, same one-line program:

| Command | Size |
|---|---|
| `rustc t.rs -o t` | 451,760 |
| `rustc -O t.rs -o t` | 451,224 |
| `rustc -O t.rs -o t && strip t` | 330,992 |
| `rustc -O -C strip=symbols -C panic=abort t.rs -o t` | 330,696 |

### The trap: rustc's default edition is 2015

This is the one that will actually cost you an afternoon. `rustc` on its own compiles as **edition 2015**, not the current edition, and it says so only indirectly — by rejecting modern syntax:

```text
error: let chains are only allowed in Rust 2024 or later
 --> ed.rs:5:8
  |
5 |     if let Some(x) = opt && x > 1 { … }
  |        ^^^^^^^^^^^^^^^^^
```

Cargo passes the edition from `Cargo.toml` on every invocation, so you never meet this inside a project. By hand you pass it yourself — which is why every command in this repo reads `rustc --edition 2024 …`, and why [`tools/run_examples.py` ↗](https://github.com/masiarek/rust-learning-library/blob/master/tools/run_examples.py) has `EDITION = "2024"` near the top rather than trusting the default.

### What else Cargo was quietly doing

- **The profile.** Plain `rustc` is the debug profile: `cfg!(debug_assertions)` is true and integer overflow **panics**. Add `-O` and the same source *wraps* instead. The flag you forgot changes behaviour, not just speed — `cargo run` and `cargo run --release` at least name the two.
- **`CARGO_*` variables.** `env!("CARGO_PKG_NAME")` is a compile error without Cargo (*"environment variable `CARGO_PKG_NAME` not defined at compile time"*), so a crate that reads its own version cannot be built this way at all.
- **Tests.** `#[test]` functions are compiled out of a normal build. `rustc --test file.rs -o t` builds the harness as the entry point instead — that is exactly what `cargo test` runs, and it works fine on a loose file.
- **Dependencies.** The wall. One `use rand::…` means fetching that crate, building it, and passing `--extern rand=librand.rlib` by hand — for it, and for each of *its* dependencies, transitively. This is the thing Cargo exists to do, and the point at which the shortcut stops being one.

## 2. `cargo new` — the actual shortcut

```sh
cargo new /tmp/scratch      # --bin is the default
cd /tmp/scratch
cargo run
```

`cargo new` writes the hello-world `main.rs` for you, so it is two commands to a running program — and you get the edition, the profiles, and the dependency machinery for free. `--lib` gives a library skeleton instead.

Nesting is allowed: run `cargo new` inside another package's directory and you get a completely independent package the outer `Cargo.toml` knows nothing about. Two things decide whether that is quiet or not, and both are about the *outer* manifest — if it declares a `[workspace]`, current Cargo adds your new package as a **member** (`Adding 'scratch' as member of workspace at …`), which means it shares that workspace's lockfile and `target/`; if it is a plain package, nothing is said and nothing is shared.

**Clean up after these.** A hello-world `target/` is about 1 MB, but that number is set by your dependencies, not by your code: a scratch project with a few real crates in it reaches hundreds of MB, and a Rustlings checkout's `target/` measured 128 MB here. `rm -rf` the whole directory when you are done rather than letting scratch projects accumulate.

## 3. `src/bin/*.rs` — scratch code inside a package you already have

Any file under `src/bin/` with a `main` becomes its own binary target, sharing the package's dependencies, edition, and lints:

```sh
mkdir -p src/bin
echo 'fn main() { println!("hi"); }' > src/bin/scratch.rs
cargo run --bin scratch
```

This is the one to reach for while working through exercises — [Rustlings ↗](https://rustlings.rust-lang.org/), say — because the scratch program is compiled exactly like the exercise beside it, with the same toolchain and the same crates in scope, and you never touch an exercise file to try something out.

It keeps working in packages whose manifest looks unusual. Rustlings opens its `Cargo.toml` with a root-level array listing every exercise and solution:

```toml
bin = [
  { name = "intro1", path = "exercises/00_intro/intro1.rs" },
  { name = "intro1_sol", path = "solutions/00_intro/intro1.rs" },
  …
]
```

In TOML an array of inline tables under a top-level key is exactly equivalent to repeating `[[bin]]` sections — it is a compact spelling of the same thing, and it has to appear *before* the first `[table]` header or it becomes a key of that table instead. **Listing targets explicitly does not switch auto-discovery off**: `autobins` is on for edition 2018 and later, so `src/bin/*.rs` targets are added alongside the listed ones.

**The name collision is quieter than an error.** Cargo does reject a name listed twice *in the manifest* — `found duplicate binary name ex01, but all binary targets must have a unique name`. But a `src/bin/<name>.rs` that collides with an **already-listed** target is not an error at all: the explicit entry wins, your file is silently ignored, and `cargo run --bin <name>` cheerfully runs the exercise you were trying to experiment beside. Nothing is printed. Give scratch binaries a name no manifest would use.

One more habit: `src/` is usually not in a Rustlings-style `.gitignore` (that file lists `target/`, `Cargo.lock`, `.vscode/`), so either add it or delete the scratch file when you are done.

## Which one to reach for

| Situation | Route |
|---|---|
| One file, std only, want it gone in a minute | `rustc --edition 2024 f.rs -o /tmp/f && /tmp/f` |
| Reading this library's examples | the same — that is all they are |
| You want a dependency, or `cargo test`/`clippy`/`fmt` | [`cargo new`, then `cargo add`](../../05_Tooling/scratch_with_a_crate/README.md) |
| Poking at an idea beside an exercise you are doing | `src/bin/scratch.rs` in that package |
| Something you will still have next week | `cargo new`, somewhere you will find it again |

## What the program itself can tell you

Compile-time facts, printed at run time — every one of them decided by the command line rather than by the source:

<!-- output:rustc_without_cargo -->
*Verified output of [`rustc_without_cargo.rs`](examples/rustc_without_cargo.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: Who built me?
  No Cargo here — CARGO_PKG_NAME was never set.
      `option_env!` asks at COMPILE time, and answers None when the
      variable is absent. Its cousin `env!` is a hard error instead:
      'environment variable `CARGO_PKG_NAME` not defined at compile
      time'. A crate that reads its own version that way cannot be
      built by rustc alone — setting those is Cargo's job.

──── Step 2: Which edition?
  A let chain ran, so this was built as edition 2024: top = 5
      That `if let … && …` head is the proof. rustc's DEFAULT edition
      is 2015, not the current one, and under it this file does not
      compile: 'let chains are only allowed in Rust 2024 or later'.
      Cargo passes the edition from Cargo.toml every time; by hand
      you pass it yourself, which is why every command in this repo
      reads `rustc --edition 2024`.

──── Step 3: Which profile?
  cfg!(debug_assertions) = true
  bump(255) = 0
  255u8.checked_add(1) = None
      Plain `rustc` is the debug profile: overflow checks ON. Add -O
      and cfg!(debug_assertions) turns false — and a plain `v + 1`
      that panicked here ('attempt to add with overflow', exit 101)
      wraps quietly to 0 there. The flag you forgot changed the
      program's BEHAVIOUR, not just its speed. `cargo run` gives you
      the first and `cargo run --release` the second, under names
      that are harder to forget.

──── Step 4: What you gave up
  std is all here: seats["Ada"] = Some(2)
      All of std, and nothing else. `use rand::…` means fetching that
      crate, building it, and passing --extern rand=librand.rlib by
      hand — for it, and for every dependency of its own. That is the
      wall, and clearing it is the one thing Cargo exists to do.
      Tests too: #[test] functions are compiled out of this binary
      entirely. `rustc --test` builds the harness instead of main.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 15_First_Programs/rustc_without_cargo/examples/rustc_without_cargo.rs -o /tmp/rwc && /tmp/rwc
```

## If you are coming from another language

- **Python** — there is no step that produces a distributable artifact, so the closest thing to `rustc f.rs` is `python f.py` and the closest thing to `cargo new` is a virtualenv plus a `pyproject.toml`. What transfers is the instinct that a scratch file needs no project. What changes is that the scratch file here still compiles to something standalone — and that the moment you want one third-party package, the ceremony you skipped becomes mandatory rather than merely convenient.
- **ABAP** — a report lives *in* the system; you never hold the artifact, and there is no "just run this file" at all. `cargo new` is nearest to creating a package for something you intend to keep, and `src/bin/scratch.rs` is the `Z…_TEST` report you write inside an existing package so it inherits everything around it — including the habit of deleting it before it ships. What actually changes is ownership of the output: `rustc` hands you a file you can copy to a machine that has no toolchain, which has no counterpart in a system-resident program.

---

## Practice

**One file, three builds.** Write a single `.rs` file with a small function, a `#[cfg(test)] mod tests` that tests it, and a `main` that prints a couple of answers. No `Cargo.toml`.

Now build it three ways and predict each result before you run it: plain `rustc --edition 2024`, then `--test`, then `-O`. One of them never calls your `main`; one of them prints a different value than the other two; and if you leave out `--edition 2024`, one of them may not compile at all. Say out loud which is which before checking.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:rustc_without_cargo_kata -->
*[`rustc_without_cargo_kata.rs`](examples/rustc_without_cargo_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one file, three builds.
//!
//!   rustc --edition 2024 rustc_without_cargo_kata.rs -o /tmp/rwck && /tmp/rwck
//!   rustc --edition 2024 --test rustc_without_cargo_kata.rs -o /tmp/rwck_t && /tmp/rwck_t
//!   rustc --edition 2024 -O rustc_without_cargo_kata.rs -o /tmp/rwck_o && /tmp/rwck_o
//!
//! Same source, three different programs: your `main`, a test harness that
//! never calls `main` at all, and an optimized build where one of the answers
//! below changes. No Cargo.toml anywhere.

/// The scoring round of a STAR count, for one candidate.
fn total(scores: &[u8]) -> u32 {
    scores.iter().map(|s| u32::from(*s)).sum()
}

/// A ballot may leave a candidate unscored; a blank is not a zero-score.
fn mean(scores: &[u8]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }
    Some(f64::from(total(scores)) / scores.len() as f64)
}

fn main() {
    let ballots: [u8; 4] = [5, 3, 0, 4];

    println!("total     = {}", total(&ballots));
    println!("mean      = {:?}", mean(&ballots));
    println!("mean([])  = {:?}", mean(&[]));

    println!("\nBuilt with debug_assertions = {}", cfg!(debug_assertions));
    println!("  Plain `rustc` says true, `rustc -O` says false. That is the");
    println!("  same split as `cargo run` versus `cargo run --release`, and it");
    println!("  decides whether an arithmetic overflow panics or wraps.");

    println!("\nThe tests below did not run, and were not even compiled:");
    println!("  #[cfg(test)] is false in this build, so the module does not");
    println!("  exist in this binary. `rustc --test` builds the harness as the");
    println!("  entry point instead — main is the thing that goes unused then.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_a_ballot() {
        assert_eq!(total(&[5, 3, 0, 4]), 12);
    }

    #[test]
    fn empty_has_no_mean() {
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn mean_is_the_total_over_the_count() {
        assert_eq!(mean(&[5, 3, 0, 4]), Some(3.0));
    }
}
```
<!-- /source -->

<!-- output:rustc_without_cargo_kata -->
*Verified output of [`rustc_without_cargo_kata.rs`](examples/rustc_without_cargo_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
total     = 12
mean      = Some(3.0)
mean([])  = None

Built with debug_assertions = true
  Plain `rustc` says true, `rustc -O` says false. That is the
  same split as `cargo run` versus `cargo run --release`, and it
  decides whether an arithmetic overflow panics or wraps.

The tests below did not run, and were not even compiled:
  #[cfg(test)] is false in this build, so the module does not
  exist in this binary. `rustc --test` builds the harness as the
  entry point instead — main is the thing that goes unused then.
```
<!-- /output -->

The other two builds, run by hand. `--test` replaces your entry point with the harness `cargo test` uses:

```text title="Real output of `rustc --edition 2024 --test rustc_without_cargo_kata.rs -o t && ./t`"

running 3 tests
test tests::empty_has_no_mean ... ok
test tests::mean_is_the_total_over_the_count ... ok
test tests::totals_a_ballot ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

And `-O` changes one line of the output above — `debug_assertions` reports `false`, because optimization and the debug profile are the same switch here.

</details>

---

## See also

- [A throwaway that needs a crate](../../05_Tooling/scratch_with_a_crate/README.md) — the sequel to route 2: the three commands that clear the dependency wall above, and the `rustc` message that means you skipped them
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — the exit code 101 mentioned above, and what unwinding does and does not give back
- [`if let`](../../17_Option_and_Result/if_let/README.md) — the let chain used as an edition detector, and why it is 2024-only
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — how a lesson's example is compiled, run, and held to a recorded answer key
- [The long way round to a STAR count](../../ROADMAP.md) — rung 10 is where the single-file rule finally retires and this repo grows a `Cargo.toml`
- [The Cargo Book — Cargo targets ↗](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) — `autobins`, `src/bin/`, and the manifest keys behind all of the above

## Po polsku

Pojedynczy plik nie potrzebuje żadnego projektu: `rustc plik.rs -o plik && ./plik` to cały łańcuch narzędzi i dokładnie tak kompilowany jest każdy przykład w tej bibliotece. Warto zobaczyć, co z tego wychodzi — trzydzieści bajtów źródła daje natywny plik wykonywalny ważący około 451 KB, który uruchomi się na innej maszynie tej samej platformy, także takiej, gdzie Rusta w ogóle nie zainstalowano. To jest ta różnica wobec `python plik.py`, przy którym interpreter musi być obecny zawsze. I te 451 KB to nie jest twój kod: to biblioteka standardowa linkowana statycznie plus maszyneria panik i nazwy symboli potrzebne do śladu stosu — czyli **stały narzut, a nie koszt rosnący z kodem**. Program na pięć tysięcy linii nie będzie zauważalnie większy.

Pułapka, która potrafi zjeść popołudnie, brzmi: **`rustc` uruchomiony sam kompiluje w edycji (*edition*) 2015**, a nie w bieżącej. I nie mówi tego wprost — mówi przez odrzucenie nowszej składni, komunikatem `error: let chains are only allowed in Rust 2024 or later`. To zdanie czyta się jak „masz za starego Rusta”, więc odruchem jest `rustup update`, po którym nic się nie zmienia: problem nigdy nie był w wersji kompilatora, tylko w domyślnej edycji. Cargo podaje edycję z `Cargo.toml` przy każdym wywołaniu, więc wewnątrz projektu nigdy tego nie spotkasz; z ręki podajesz ją sam — i dlatego każde polecenie w tym repozytorium brzmi `rustc --edition 2024 …`.

Reszta strony to lista rzeczy, które Cargo robiło po cichu, a jedna z nich zmienia **zachowanie** programu, nie jego szybkość. Goły `rustc` to profil debug: `cfg!(debug_assertions)` jest prawdą, a przepełnienie arytmetyczne (*overflow*) w zwykłym `v + 1` **panikuje** — „attempt to add with overflow”, kod wyjścia 101. Po dodaniu `-O` ten sam kod cicho zawija się do zera. `cargo run` i `cargo run --release` przynajmniej nazywają te dwa tryby po imieniu. Poza tym `env!("CARGO_PKG_NAME")` bez Cargo jest zwykłym błędem kompilacji, funkcje `#[test]` w normalnym budowaniu nie trafiają do binarki w ogóle (`rustc --test` buduje zamiast `main` ten sam program testowy, który uruchamia `cargo test`), a prawdziwą ścianą są zależności: jedno `use rand::…` to ręczne `--extern` dla tego crate'a i przechodnio dla każdej jego zależności. Po to właśnie istnieje Cargo.

Trzecia droga — plik w `src/bin/` wewnątrz pakietu, który już masz — jest tą, po którą warto sięgać przy przerabianiu ćwiczeń, bo kod roboczy kompiluje się tym samym narzędziem i z tymi samymi crate'ami co ćwiczenie obok. Ma jednak cichą pułapkę: jeśli nazwa twojego pliku pokrywa się z celem **wypisanym w manifeście**, nie dostaniesz żadnego błędu — wygrywa wpis z manifestu, twój plik zostaje po prostu zignorowany, a `cargo run --bin <nazwa>` uruchamia to ćwiczenie, obok którego chciałeś eksperymentować. Nic nie zostaje wypisane. Dawaj plikom roboczym nazwę, jakiej żaden manifest by nie użył, i pamiętaj o sprzątaniu katalogu `target/`: dla hello world to około 1 MB, ale przy kilku prawdziwych zależnościach idzie w setki megabajtów.

**Szukaj po polsku:** kompilacja bez Cargo · edycja 2024 w Ruscie · przepełnienie arytmetyczne · `rustc --edition 2024` · `rust let chains are only allowed in Rust 2024 or later` · `cargo new src/bin`
