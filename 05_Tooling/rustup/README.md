# rustup: the `rustc` you run is not the compiler

**Level:** 101 → 201 · working knowledge

**One line:** `rustup` installs compilers and then puts a 154-byte stand-in on your `PATH` under the name `rustc`, which decides *per command* which real compiler runs — and the five-rung rule it uses to decide is essentially the whole of rustup.

## The shim

Ask this machine where `rustc` is, and then look at what is actually there:

```sh
which rustc                 # /usr/local/opt/rustup/bin/rustc   — 154 bytes
ls -l ~/.rustup/toolchains/stable-x86_64-apple-darwin/bin/rustc   # 392 KB
```

Two files, the same name, a factor of two and a half thousand apart in size. The small one is a **shim**: it works out which toolchain this invocation should use, and execs the large one. Every `rustc`, `cargo`, `clippy` and `rustfmt` you type goes through one.

This is not trivia. It is the mechanism that every other toolchain feature is built on, and the reason each of them can fail in the same way. A [`rust-toolchain.toml`](../pinning_the_toolchain/README.md) that appears to be ignored, a `cargo +nightly` that runs stable anyway, a CI job using a compiler nobody chose — all of them are the same bug: **something called the real binary directly and never passed through the shim.** An absolute path in a script, a Docker image with the toolchain baked in, an IDE configured with a hard-coded compiler path. The version pin was never consulted, because nothing was there to consult it.

## How it decides

When the shim runs, it picks a toolchain by the first of these that applies — highest priority first:

| # | Source | Scope |
|---|---|---|
| 1 | `+toolchain` on the command line — `cargo +nightly build` | this one command |
| 2 | the `RUSTUP_TOOLCHAIN` environment variable | this shell, or this CI job |
| 3 | a directory override, from `rustup override set` | one directory, recorded outside the project |
| 4 | [`rust-toolchain.toml`](../pinning_the_toolchain/README.md) in this directory or a parent | the project, for everyone who clones it |
| 5 | the default, from `rustup default` | every project on the machine |

Read that column on the right rather than the numbers. It runs from *narrowest and most visible* at the top to *widest and least visible* at the bottom, and the two properties travel together: rung 1 is written in the command you are looking at, and rung 5 is written nowhere near the code it affects. Rungs 3 and 4 are additionally resolved **by proximity** — the closest one to your working directory wins — which is what makes a toolchain file in a subdirectory beat one at the repository root.

The command that reads the decision back is the one to reach for whenever a version surprises you:

```sh
rustup show
```

On this machine it answers `stable-x86_64-apple-darwin`, and — the useful half — *why*: `active because: it's the default toolchain`. Rung 5, nothing overriding it.

## Channels

Three moving names and one fixed form:

- **`stable`** — a release every six weeks. What almost everything should use.
- **`beta`** — the next stable, six weeks early. Its purpose is that you test *your* crate against it and report regressions before they ship.
- **`nightly`** — built from the master branch, most nights. Unstable features are permitted here and nowhere else. The decision to make this your default is a bigger one than it looks, and has [its own page](../nightly/README.md).
- **`nightly-2026-08-11`** — a specific night, frozen. This is the form to use when a project needs nightly, because plain `nightly` is a name whose meaning changes while you sleep.

## Components and targets

Two things a toolchain carries beyond the compiler, and they are not the same kind of thing:

- **Components** are tools that ship alongside `rustc` — `clippy`, `rustfmt`, `rust-analyzer`, `rust-src`, `llvm-tools`. `rustup component add clippy`, or list them in the toolchain file so nobody has to.
- **Targets** are platforms you can compile *for*: `rustup target add wasm32-unknown-unknown`, then `cargo build --target wasm32-unknown-unknown`. Cross-compiling needs the target's standard library, which is what this installs; it does not install a linker, which is the usual reason a first cross-build still fails.

## If you are coming from another language

- **Python** — `pyenv` is the direct analogue, shims and all, and it gets the same class of bug for the same reason: a script calling `/usr/bin/python3` by absolute path bypasses the version manager entirely. If you have debugged that, you have already debugged the rustup version.
- **ABAP** — nothing corresponds. The compiler is part of the system you logged into, chosen by whoever administers the landscape, and it is the same for every developer on it by construction. Rust hands that choice to you, per project, which is why it needs a rule for resolving it.

## See also

- [Pinning the toolchain](../pinning_the_toolchain/README.md) — rung 4, and why it is the only rung worth using for a shared project
- [Nightly by default](../nightly/README.md) — rung 5, and what it quietly decides
- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — what `rustc` does once the shim has picked one

---

*No generated output block: the sizes and paths above were read off the machine this library is written on, and would differ on yours — an answer key cannot hold a filesystem.*

## Po polsku

Polskie kursy zwykle zaczynają się zdaniem „zainstaluj Rusta przez rustupa” i na tym temat zamykają — a właśnie tutaj siedzi cały mechanizm. `rustc`, którego masz na `PATH`, nie jest kompilatorem: to licząca 154 bajty nakładka (*shim*), która przy **każdym** wywołaniu z osobna ustala, który prawdziwy `toolchain` ma się uruchomić, i dopiero jego wywołuje. Samego słowa `toolchain` nie warto tłumaczyć — „zestaw narzędzi” powiedz raz, dla zrozumienia, a potem pisz `toolchain`, bo to podkomenda (`rustup toolchain list`) i nazwa pliku `rust-toolchain.toml`; przetłumaczony nie znajdzie się w żadnym komunikacie ani wyniku wyszukiwania.

Wybór toolchaina to drabinka pięciu szczebli, którą lepiej czytać po prawej kolumnie tabeli niż po numerach: od najwęższego i najlepiej widocznego (`cargo +nightly build` — widać go w poleceniu, które właśnie piszesz) do najszerszego i najmniej widocznego (`rustup default`, ustawiony pół roku temu i nieobecny nigdzie w pobliżu kodu, na który wpływa). Szczeble 3 i 4 rozstrzyga dodatkowo bliskość katalogu, więc `rust-toolchain.toml` w podkatalogu wygrywa z tym w katalogu głównym repozytorium. Gdy wersja kompilatora zaskakuje, pytanie brzmi nie „jaka wersja”, tylko „dlaczego ta” — i odpowiada na nie `rustup show`, który dopisuje uzasadnienie: `active because: it's the default toolchain`. Warto też zapamiętać, że `nightly` to nazwa, której znaczenie zmienia się w nocy — projekt, który naprawdę potrzebuje nightly, przypina konkretną datę (`nightly-2026-08-11`).

Najczęstsza awaria nie polega na tym, że przypięcie wersji nie działa, tylko na tym, że nikt do niego nie zajrzał: skrypt woła kompilator ścieżką bezwzględną, obraz Dockera ma toolchain wbudowany, IDE ma na sztywno wpisaną ścieżkę do `rustc` — i nakładka, jedyne miejsce, które w ogóle czyta `rust-toolchain.toml`, nie uruchamia się ani razu. Kto debugował to samo w `pyenv` (skrypt wołający `/usr/bin/python3` omija menedżer wersji), zna ten błąd na pamięć; tutaj jest identyczny, zmieniają się tylko nazwy.

**Szukaj po polsku:** instalacja Rusta rustup · wersje kompilatora Rust · przypinanie wersji toolchaina · `rustup show active toolchain` · `rustup override vs rust-toolchain.toml`
