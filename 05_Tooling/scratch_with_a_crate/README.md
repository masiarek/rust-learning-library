# A throwaway that needs a crate: three commands, and the message that means you skipped them

**Level:** 101 · for newcomers

**One line:** `rand::random_range(..)` in a loose `.rs` file gets you *"you might be missing a crate named `rand`"* — nothing is missing and there is nothing to install; the file is being compiled by `rustc`, which links only what its command line names, and the three commands that fix it also write the folder layout, the manifest, the lockfile and a working test runner.

## The three commands

```sh
cargo new /tmp/scratch && cd /tmp/scratch
cargo add rand
cargo run
```

`cargo new` leaves a hello-world in `src/main.rs`. Replace its body and `cargo run` again:

```rust
fn main() {
    let n: u32 = rand::random_range(1..=100);
    println!("secret = {n}");  // secret = 57   (a different number every run)
}
```

The whole thing, verbatim:

```text title="Real output — cargo 1.97.1, rand 0.10.2"
$ cargo new /tmp/scratch
    Creating binary (application) `scratch` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

$ cargo add rand
    Updating crates.io index
      Adding rand v0.10.2 to dependencies
             Features:
             + alloc
             + std
             + std_rng
             + sys_rng
             + thread_rng
             - chacha
             - log
             - serde
             - simd_support
             - unbiased
    Updating crates.io index
     Locking 8 packages to latest Rust 1.97.1 compatible versions

$ cargo run
   Compiling libc v0.2.189
   Compiling rand_core v0.10.1
   Compiling cfg-if v1.0.4
   Compiling getrandom v0.4.3
   Compiling cpufeatures v0.3.0
   Compiling chacha20 v0.10.1
   Compiling rand v0.10.2
   Compiling scratch v0.1.0 (/private/tmp/scratch)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.59s
     Running `target/debug/scratch`
secret = 57
```

Two and a half seconds, and six crates nobody typed. `rand` is a small crate that depends on `getrandom` for the OS entropy, `chacha20` for the generator itself, and `libc`, `cfg-if`, `cpufeatures` and `rand_core` under those.

**Eight packages locked, seven compiled.** The odd one out is `r-efi`, a UEFI firmware interface that `getrandom` names for a target this laptop is not — it is in `Cargo.lock` and never in the build. A lockfile records the resolution for *every* platform the manifest can reach, so its length is a poor estimate of what your machine builds.

## The message that means you skipped them

Compile that same three-line program as a loose file and the compiler stops on the second line:

```text title="Real output — rustc 1.97.1, `rustc --edition 2024 secret.rs -o secret`"
error[E0433]: cannot find module or crate `rand` in this scope
 --> secret.rs:2:18
  |
2 |     let n: u32 = rand::random_range(1..=100);
  |                  ^^^^ use of unresolved module or unlinked crate `rand`
  |
  = help: you might be missing a crate named `rand`

error: aborting due to 1 previous error
```

A `use rand::RngExt;` line at the top adds an `error[E0432]: unresolved import` above it, from the same cause.

The help line is the reason this costs anyone an afternoon. Nothing is missing — not from your machine, not from your toolchain, and there is no `install rand` step anywhere in Rust. `rustc` compiles the crates its command line names and no others, so *every* name that is not `std` or your own file produces this message: a typo, a module you forgot to declare, and a package on [crates.io ↗](https://crates.io) are one error with one wording.

`rustc --explain E0433` does eventually say *"Make sure the crate has been added as a dependency in `Cargo.toml`"* — four paragraphs down, after a `HashMap` example about a missing `use` statement, which is the same message from an unrelated cause.

The narrow reading is [in the scratch-program page](../../15_First_Programs/rustc_without_cargo/README.md): a dependency can be linked by hand with `--extern rand=librand.rlib`, once you have built `rand` and each of its six dependencies yourself, in order. That is the wall Cargo exists to clear, and `cargo add` is the whole of clearing it.

## What the three commands wrote

```text
/tmp/scratch
├── .git/                 an initialised repository
├── .gitignore            one line: /target
├── Cargo.toml            the manifest
├── Cargo.lock            written by cargo add, not by cargo new
└── src/
    └── main.rs           fn main() { println!("Hello, world!"); }
```

| What you would otherwise decide | What decided it |
|---|---|
| the edition | `edition = "2024"` in the manifest — so the [2015 default](../../15_First_Programs/rustc_without_cargo/README.md) that bites bare `rustc` cannot reach you |
| the version requirement | `cargo add` read crates.io and wrote `rand = "0.10.2"` |
| which versions actually built | `Cargo.lock`, written by `cargo add` when it resolved the graph — before anything was compiled |
| debug or release | `cargo run` is debug, `cargo run --release` is not — the two profiles have names instead of a `-O` you forget |
| where the build output goes | `target/`, already in the `.gitignore` |
| whether tests run | `cargo test` works in this folder before you have written one |

## The manifest, and the entries a throwaway needs

All of it:

```toml
[package]
name = "scratch"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.10.2"
```

That last line is the only one you would have typed, and `cargo add` typed it. Prefer that to writing it out: the command asks crates.io what the current version is, so it cannot enshrine a number you half-remember from a tutorial — [and `rand` is a crate that has renamed its API twice](../../15_First_Programs/randomness/README.md), which makes an accidentally-stale version number a compile error with a confusing message.

Two more tables are worth knowing about and neither belongs in a throwaway:

| Table | What it is for | Written by |
|---|---|---|
| `[dev-dependencies]` | a crate used only by tests, benches and examples — not compiled into the binary | `cargo add --dev pretty_assertions` |
| `[lints]`, `[profile.*]`, `[workspace]` | policy: what is denied, how release builds are optimized, which packages share a build | you, and only when there is a second project to share it with |

`rand = "0.10.2"` is a **range**, not a pin — it accepts every future 0.10.x. [Adding a dependency](../cargo_dependencies/README.md) is the page on what that permits and what `Cargo.lock` does about it.

The failure mode when you get ahead of yourself:

```text
$ cargo add rand
error: could not find `Cargo.toml` in `/private/tmp/nopkg` or any parent directory
```

`cargo add` edits a manifest; there has to be one. `cargo new` first, always.

## Testing came with the folder

There is no test runner to install, no test directory to create, and nothing to register. `cargo test` works in the folder `cargo new` just made — it simply has nothing to run yet:

```text
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Tests live in the file they test, in a module the normal build does not compile:

```rust
fn clamp_score(n: i32) -> u8 {
    n.clamp(0, 5) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_high() {
        assert_eq!(clamp_score(99), 5);
    }

    #[test]
    fn clamps_low() {
        assert_eq!(clamp_score(-3), 0);
    }
}
```

```text title="Real output — `cargo test`"
running 2 tests
test tests::clamps_high ... ok
test tests::clamps_low ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`#[cfg(test)]` is why this costs the binary nothing: the module is not in a `cargo run` build at all, so a throwaway can carry its tests without carrying their weight. Bare `rustc` reaches the same harness with `--test`, which is [what that flag was doing on a loose file](../../15_First_Programs/rustc_without_cargo/README.md) — what Cargo adds is not the harness but not having to remember it.

Automating the loop from here is one more command and no configuration: [`bacon`](../bacon/README.md) re-runs the tests on every save in a pane you leave open, and [`cargo-nextest`](../nextest/README.md) is the replacement runner worth knowing about once a suite is big enough to notice.

## Clean up after it

`target/` for the three-line program above measured **14 MB**, which is set by the dependency, not by your code — a scratch project with a few real crates in it reaches hundreds of megabytes. `rm -rf /tmp/scratch` when you are done.

What survives that is the download cache: `~/.cargo/registry` measured 646 MB here, shared by every project on the machine, and it is why the *second* project to use `rand` builds without touching the network. Deleting it is safe and costs you those downloads again.

## Which route for which throwaway

| The throwaway | Route |
|---|---|
| one file, `std` only, gone in a minute | `rustc --edition 2024 f.rs -o /tmp/f && /tmp/f` |
| one file that wants a crate | `cargo new` + `cargo add` — this page |
| an idea to try beside an exercise that already has the crate | `src/bin/scratch.rs` in that package |
| the fortieth exercise this month | [one workspace](../practice_workspace/README.md), and `cargo new` inside it |
| something you will still want next week | `cargo new`, somewhere you will find it again |

## What about a single-file script?

Cargo can run a `.rs` file directly, with the manifest embedded at the top of the file, which is exactly the shape this page is working around. It is unstable, and on a pinned stable toolchain it declines rather than degrades:

```text
$ cargo -Zscript secret.rs
error: the `-Z` flag is only accepted on the nightly channel of Cargo, but this is the `stable` channel
```

It is [tracked as Cargo issue #12207 ↗](https://github.com/rust-lang/cargo/issues/12207) and documented under [`script` in the unstable reference ↗](https://doc.rust-lang.org/cargo/reference/unstable.html#script). Worth watching, not worth [switching channel for](../nightly/README.md) — three commands is not the bottleneck.

## If you are coming from another language

**Python.** The muscle memory that fails here is `pip install`. In Python a scratch file that says `import requests` works whenever the interpreter it happens to run under has `requests` somewhere on its path — which is convenient, and is also why the same file works in your terminal and fails under cron, in an editor, or on a colleague's machine, and why the diagnosis starts with `which python`. Rust has no ambient environment to consult: a build links what the manifest names and nothing else, so `cargo add rand` is not "install rand on this machine" but "record that this program uses rand", and the recording travels with the code. What you give up is the one-liner — there is no equivalent of `pip install x` followed by `python f.py`, because there is no interpreter with a search path to install *into*. What you get is that the failure happens at compile time, on your machine, in a message naming the crate, rather than at import time on someone else's.

The nearest true analogy is not `pip` at all but a `pyproject.toml` plus a lockfile plus a virtualenv that is created for you, per project, and cannot be forgotten — `cargo new` is `uv init`, `cargo add rand` is `uv add rand`, and `cargo run` is `uv run`. That similarity is not a coincidence: `uv` and Cargo solve the same problem, and `uv` is written in Rust.

**ABAP.** There is no step here that has an ABAP counterpart, and that is the thing worth naming. A report you write in SE38 can call anything the system already contains — every function module, every class in every package — because the system *is* the dependency set, provisioned by Basis long before you sat down, and identical for everyone logged into that client. `cargo add` is what fills that role for a language with no system behind it: each program carries its own list of what it needs and where to get it, and the build assembles that list from scratch. Two consequences follow, and they cut in opposite directions. A Rust throwaway is genuinely self-describing — hand someone the folder and they can build it, with no transport, no client, no `$TMP`, and no request to Basis for a missing component. And a Rust throwaway can be *wrong on its own*: two programs on one machine can want different versions of the same crate and both get what they asked for, which in ABAP terms is closer to two reports running against two different releases of the same function group. `Cargo.lock` is the artifact that makes that fact auditable, and it is the file with no ABAP equivalent because ABAP never needed one.

## See also

- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — the page this one is the sequel to: `rustc` alone, `cargo new`, `src/bin/`, and what each leaves out
- [Randomness, and the `rand` API the Rust Book still teaches](../../15_First_Programs/randomness/README.md) — the *next* error after this one, and the reason `rand` is the crate everybody meets first
- [Adding a dependency](../cargo_dependencies/README.md) — `cargo search`, `cargo info`, and what a caret requirement really permits
- [A tree of practice projects](../practice_workspace/README.md) — the answer when there are forty of these rather than one
- [bacon](../bacon/README.md) — the tests re-running on save, in a pane you leave open
- [The Cargo Book — `cargo add` ↗](https://doc.rust-lang.org/cargo/commands/cargo-add.html) — every flag, including `--dev`, `--features` and `--optional`

## Po polsku

Podpowiedź kompilatora — *you might be missing a crate named `rand`* — po polsku brzmi jak „brakuje ci pakietu `rand`”, i to jest dokładnie to jedno zdanie, przez które ta pomyłka kosztuje ludzi całe popołudnie. **Nic nie brakuje.** Nie ma czego doinstalować ani na maszynie, ani w toolchainie, bo w Ruście nie istnieje krok „zainstaluj bibliotekę”. `rustc` linkuje wyłącznie to, co wymieniono w jego wierszu poleceń, więc literówka w nazwie modułu, zapomniane `mod` i prawdziwy pakiet z [crates.io ↗](https://crates.io) dają **jeden i ten sam** komunikat `E0433`. Uwaga na pułapkę wyszukiwania: hasło „jak zainstalować crate” prowadzi wprost do `cargo install`, a to polecenie instaluje **programy** (`ripgrep`, `bacon`), nie biblioteki — na bibliotece odpowie, że nie ma tam nic do zainstalowania, i można przy nim stracić kolejne pół godziny. Słowa `crate` nie tłumaczymy: „skrzynka” to kalka, której nikt nie używa, a samo słowo jest częścią poleceń i komunikatów.

Właściwe polecenie to `cargo add rand` i warto je czytać nie jako „zainstaluj”, tylko jako **„zapisz, że ten program używa `rand`”** — zapis ląduje w `Cargo.toml` i podróżuje razem z kodem, a nie zostaje na twoim laptopie. Przy okazji dostajesz cztery rzeczy, o których w luźnym pliku `.rs` trzeba pamiętać samemu: `edition = "2024"` w manifeście (goły `rustc` bez tego przełącznika wciąż domyślnie kompiluje w edycji 2015), `Cargo.lock` z wersjami, które faktycznie się zbudowały, katalog `target/` już wpisany do `.gitignore` oraz działające `cargo test` — jeszcze zanim napiszesz pierwszy test. Wersję też lepiej zostawić `cargo add`owi: sam pyta crates.io o aktualną, więc nie utrwali numeru zapamiętanego z jakiegoś kursu, a akurat `rand` zmieniał API dwa razy.

Dwa szczegóły z tej strony warto zapamiętać, bo oba przeczą intuicji. Po pierwsze, długość `Cargo.lock` nie mówi, ile się kompiluje: zablokowanych jest osiem pakietów, a zbudowanych siedem, bo `r-efi` (interfejs firmware UEFI) dotyczy platformy, którą ten laptop nie jest — plik blokady zapisuje rozwiązanie zależności dla **wszystkich** platform, jakie manifest może osiągnąć. Po drugie, testy nie mieszkają w osobnym katalogu: idą do modułu `#[cfg(test)]` w tym samym pliku co testowana funkcja, a kompilator w ogóle nie wstawia ich do zwykłego `cargo run` — więc jednorazowy programik może mieć testy i nic za nie nie płacić. Kto przychodzi z Pythona i szuka katalogu `tests/` oraz biblioteki do zainstalowania, szuka dwóch rzeczy, których tu nie ma.

Na koniec sprzątanie, o którym nikt nie uprzedza: `target/` dla trzylinijkowego programu waży 14 MB, bo o rozmiarze decyduje zależność, a nie twój kod. Kasuj cały katalog projektu, gdy skończysz — pobrane paczki i tak zostają we współdzielonym `~/.cargo/registry` i to dlatego drugi projekt z `rand` buduje się bez sieci.

**Szukaj po polsku:** jak dodać bibliotekę w Ruście · zależności w Cargo.toml · `rust E0433 you might be missing a crate` · `cargo add vs cargo install` · `rust #[cfg(test)] unit tests`
