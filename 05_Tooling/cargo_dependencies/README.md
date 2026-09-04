# Adding a dependency: `search`, `info`, `add` — and what the caret permits

**Level:** 101 → 201 · working knowledge

**One line:** `cargo add rayon` writes `rayon = "1.12.0"` into your manifest, and that string is not the version you got — it is a *range* that already includes every 1.x release nobody has published yet, which is where "it built yesterday" comes from.

## Three commands, three questions

```sh
cargo search rayon    # does a crate like this exist, and what is it called?
cargo info rayon      # should I use this one?
cargo add rayon       # put it in the manifest
cargo remove rayon    # take it out again — `cargo rm` is the same command
```

`cargo search` queries crates.io by name and prints one line each. It answers *"what is this called"* and little else — the descriptions are one line and the ranking is not a quality signal.

`cargo info` is the one worth knowing about, because it is newer than most tutorials: it was **stabilized in Rust 1.82**, having lived outside Cargo as the third-party `cargo-information` for years. It prints the version, license, repository, feature list and — the field that matters most and gets read least — `rust-version`, the crate's [MSRV](../pinning_the_toolchain/README.md):

```text
rust-version: 1.80
```

That is a dependency's claim about which compilers it supports, and it is the number that turns "add a crate" into "raise this project's minimum Rust". Worth reading *before* `cargo add`, which is the whole reason the command exists.

`cargo add` then writes the entry and picks the requirement string for you. It also takes a version and a feature list in one go, with `@`:

```text title="Real output — cargo 1.98.0"
$ cargo add serde@1.0.229 --features derive
    Updating crates.io index
      Adding serde v1.0.229 to dependencies
             Features:
             + derive
             + serde_derive
             + std
             - alloc
             - rc
             - unstable
     Locking 7 packages to latest Rust 1.98.0 compatible versions
```

```toml title="What it wrote"
[dependencies]
serde = { version = "1.0.229", features = ["derive"] }
```

The `+`/`-` list is worth a glance every time: it is the crate's optional features, with the ones now on marked `+`. `derive` switched on `serde_derive` with it, which is why seven packages were locked for one line.

`cargo remove` reverses it — the manifest line goes, and the next command drops the seven from the lockfile.

## What it wrote is a range

This is the part the slide-sized version of the story leaves out. After `cargo add rayon`, the manifest says:

```toml
[dependencies]
rayon = "1.12.0"
```

That is not "use 1.12.0". A bare version string in Cargo is a **caret requirement**, and it means *"anything from 1.12.0 up to but not including 2.0.0"* — every future 1.12.x, 1.13, 1.99, published by anyone, at any time after you wrote the line. Cargo resolves it to something concrete and records *that* in `Cargo.lock`.

So the two files divide the work:

| File | Says | Commit it? |
|---|---|---|
| `Cargo.toml` | the range you will accept | yes, obviously |
| `Cargo.lock` | the exact versions you actually built and tested against | **yes** — for applications always, and for libraries the modern advice is also yes |

Without the lockfile, "the same code" is a different program on two machines, for the same reason an [unpinned compiler](../pinning_the_toolchain/README.md) is a different compiler. With it, upgrades happen when you run:

```sh
cargo update       # move within the ranges, rewrite the lockfile
cargo update -n    # ...or just say what would move, and change nothing
```

The `-n` form is the useful one day to day, and it is exactly what appears in the [devenv](../devenv/README.md) configuration's shell hook: print what has moved on the way into the project, without touching anything. The lockfile has [a page of its own](../cargo_lock/README.md) — who writes it, who reads it, the one command that ignores it, and what `--precise` can and cannot do.

## The other operators

The bare number is the **default requirement**, and `^1.12.0` spells the same thing. Four more exist, and the recommendation — the Cargo Book's and every talk on the subject — is to use the default unless a *specific* restriction forces one of the others:

| You write | It means | Reach for it when |
|---|---|---|
| `1.2.3` or `^1.2.3` | `>=1.2.3, <2.0.0` | always, by default |
| `0.2.3` | `>=0.2.3, <0.3.0` | a `0.x` crate — the same rule, applied to the left-most **non-zero** component |
| `~1.2.3` | `>=1.2.3, <1.3.0` | patch releases only; `~1.2` is the same range, `~1` is the whole major |
| `1.2.*` | `>=1.2.0, <1.3.0` | a wildcard — and a bare `*` is refused by crates.io at publish time |
| `=1.2.3` | exactly `1.2.3` | a known-bad release above it, or a reproduction case; no leniency and no fixes |
| `>=1.2, <1.5` | both, comma-separated | a range with a ceiling below the next major — rare and worth a comment |

The `0.x` row is the one that bites. `rand = "0.8"` and `rand = "0.9"` are as far apart to Cargo as `1` and `2` — [Two versions of one crate](../two_versions_of_one_crate/README.md) is what happens when two of your dependencies disagree about that, and it is silent.

## Three tables, not one

`cargo add` puts the crate in `[dependencies]` unless told otherwise, and the other two tables are a flag each:

| Table | The crate is compiled into | Written by |
|---|---|---|
| `[dependencies]` | your library or binary | `cargo add rayon` |
| `[dev-dependencies]` | tests, examples and benches only | `cargo add --dev pretty_assertions` |
| `[build-dependencies]` | your `build.rs` script, and nothing else | `cargo add --build cc` |

A crate in the wrong table costs every one of your users a download and a compile they did not need. `pretty_assertions` in `[dependencies]` ships in the binary; in `[dev-dependencies]` it never leaves your machine.

## The ecosystem around it

Two third-party tools from the same talk, worth separating by how established they are — a distinction the recommendation itself does not make:

- **[`cargo-shear` ↗](https://crates.io/crates/cargo-shear)** — finds dependencies in `Cargo.toml` that nothing actually uses, and removes them. Mature and widely adopted (v1.13.4, ~474k downloads). Unused dependencies are pure cost — compile time, audit surface, lockfile churn — and nothing in Cargo warns about them, so this fills a real gap.
- **[`cargo-seek` ↗](https://crates.io/crates/cargo-seek)** — a terminal UI over crates.io: search on the left, the crate's details on the right, and keys to `add` or `install` without leaving it. Pleasant, and *very* new (v0.2.0, ~2.4k downloads). Try it, but do not build a team workflow on it yet.

Neither is required for anything. `search` + `info` + `add` is the whole job; these make it nicer.

## If you are coming from another language

- **Python** — the split is identical to `pyproject.toml` versus `uv.lock`, or `requirements.in` versus a compiled `requirements.txt`: one file states what you accept, the other states what you got. If you have been bitten by deploying without a lockfile, you already know this page's point.
- **ABAP** — there is no registry and no dependency resolution; code arrives by transport, and "which version" is a property of the system. The trade Cargo makes is a hundred thousand reusable crates in exchange for having to state, in a file, exactly which ones you meant.

## See also

- [`Cargo.lock`](../cargo_lock/README.md) — the file the range resolves into, who reads it, and the command that does not
- [Two versions of one crate](../two_versions_of_one_crate/README.md) — when two requirements cannot share one entry, and what the compiler says about it later
- [Vendoring, and the `[patch]` table](../vendoring_and_patch/README.md) — dependencies that come from a folder in your repo rather than the registry
- [Pinning the toolchain](../pinning_the_toolchain/README.md) — the same lock-versus-range idea, applied to the compiler
- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — what `cargo new` set up, and what `rustc` does without it
- [Compile times](../compile_times/README.md) — why an unused dependency is not free
- [The Cargo Book — specifying dependencies ↗](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) — every operator, plus `git =` and `path =` sources

## Po polsku

Najważniejsza rzecz na tej stronie jest pułapką dla oka: `rayon = "1.12.0"` **nie znaczy** „użyj wersji 1.12.0”. Goły napis z wersją to w Cargo tak zwane **wymaganie z daszkiem** (*caret requirement*), czyli skrót od `^1.12.0` — „cokolwiek od 1.12.0 wzwyż, byle poniżej 2.0.0”. Obejmuje więc także wydania, których jeszcze nikt nie opublikował. Kto przychodzi z Pythona i czyta ten zapis jak `==1.12.0` z `requirements.txt`, właśnie zgodził się na coś zupełnie innego, i stąd bierze się klasyczne „przecież wczoraj się budowało”.

Dlatego pliki są dwa i dzielą się rolami: `Cargo.toml` mówi, **na co się zgadzasz**, a `Cargo.lock` zapisuje, **co faktycznie zbudowałeś i przetestowałeś**. Oba trafiają do repozytorium — dla aplikacji zawsze, a przy bibliotekach dzisiejsze zalecenie też brzmi „tak”. Bez pliku blokującego ten sam kod jest na dwóch maszynach innym programem. Aktualizacja to świadoma decyzja, a nie efekt uboczny budowania: `cargo update` przesuwa wersje wewnątrz dozwolonych zakresów i przepisuje `Cargo.lock`, a `cargo update -n` uruchamia to samo **na sucho** — mówi, co by się przesunęło, i nie zmienia niczego.

Trzecia rzecz, po polsku prawie nieopisana, to skrót **MSRV** — *minimum supported Rust version*, czyli najstarszy kompilator, który dana zależność obsługuje. Wypisuje go `cargo info` w polu `rust-version`, i jest to jedyne pole, które warto przeczytać **przed** `cargo add`: crate z `rust-version: 1.80` po cichu podnosi wymagania całego twojego projektu. Samo polecenie `cargo info` jest przy tym na tyle nowe (weszło na stałe w Ruście 1.82), że starsze polskie poradniki go nie znają i każą szukać tych informacji ręcznie na crates.io.

Pozostałe operatory istnieją po to, żeby ich **nie** używać bez konkretnego powodu: `~1.2.3` dopuszcza tylko wydania *patch* (poniżej `1.3.0`), `=1.2.3` to dokładnie ta wersja i żadna poprawka, `1.2.*` to symbol wieloznaczny, a gołego `*` crates.io nie przyjmie przy publikacji. Ważny jest za to wiersz z zerem: reguła daszka dotyczy skrajnie lewego **niezerowego** składnika, więc `rand = "0.8"` znaczy „poniżej `0.9`” i dla Cargo `0.8` od `0.9` dzieli tyle samo, co `1` od `2`. Polecenie odwrotne do `cargo add` to `cargo remove` (alias `cargo rm`), a wersję i funkcje da się podać za jednym razem: `cargo add serde@1.0.229 --features derive`.

**Szukaj po polsku:** wersjonowanie semantyczne · czy commitować Cargo.lock · `cargo caret requirement` · `rust MSRV rust-version` · `cargo update --dry-run` · `cargo add --features` · `tilde requirement`
