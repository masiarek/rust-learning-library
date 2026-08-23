# Adding a dependency: `search`, `info`, `add` — and what the caret permits

**Level:** 101 → 201 · working knowledge

**One line:** `cargo add rayon` writes `rayon = "1.12.0"` into your manifest, and that string is not the version you got — it is a *range* that already includes every 1.x release nobody has published yet, which is where "it built yesterday" comes from.

## Three commands, three questions

```sh
cargo search rayon    # does a crate like this exist, and what is it called?
cargo info rayon      # should I use this one?
cargo add rayon       # put it in the manifest
```

`cargo search` queries crates.io by name and prints one line each. It answers *"what is this called"* and little else — the descriptions are one line and the ranking is not a quality signal.

`cargo info` is the one worth knowing about, because it is newer than most tutorials: it was **stabilized in Rust 1.82**, having lived outside Cargo as the third-party `cargo-information` for years. It prints the version, license, repository, feature list and — the field that matters most and gets read least — `rust-version`, the crate's [MSRV](../pinning_the_toolchain/README.md):

```text
rust-version: 1.80
```

That is a dependency's claim about which compilers it supports, and it is the number that turns "add a crate" into "raise this project's minimum Rust". Worth reading *before* `cargo add`, which is the whole reason the command exists.

`cargo add` then writes the entry and picks the requirement string for you.

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

The `-n` form is the useful one day to day, and it is exactly what appears in the [devenv](../devenv/README.md) configuration's shell hook: print what has moved on the way into the project, without touching anything.

## The ecosystem around it

Two third-party tools from the same talk, worth separating by how established they are — a distinction the recommendation itself does not make:

- **[`cargo-shear`](https://crates.io/crates/cargo-shear)** — finds dependencies in `Cargo.toml` that nothing actually uses, and removes them. Mature and widely adopted (v1.13.4, ~474k downloads). Unused dependencies are pure cost — compile time, audit surface, lockfile churn — and nothing in Cargo warns about them, so this fills a real gap.
- **[`cargo-seek`](https://crates.io/crates/cargo-seek)** — a terminal UI over crates.io: search on the left, the crate's details on the right, and keys to `add` or `install` without leaving it. Pleasant, and *very* new (v0.2.0, ~2.4k downloads). Try it, but do not build a team workflow on it yet.

Neither is required for anything. `search` + `info` + `add` is the whole job; these make it nicer.

## If you are coming from another language

- **Python** — the split is identical to `pyproject.toml` versus `uv.lock`, or `requirements.in` versus a compiled `requirements.txt`: one file states what you accept, the other states what you got. If you have been bitten by deploying without a lockfile, you already know this page's point.
- **ABAP** — there is no registry and no dependency resolution; code arrives by transport, and "which version" is a property of the system. The trade Cargo makes is a hundred thousand reusable crates in exchange for having to state, in a file, exactly which ones you meant.

## See also

- [Pinning the toolchain](../pinning_the_toolchain/README.md) — the same lock-versus-range idea, applied to the compiler
- [Running a scratch program](../../01_Foundations/rustc_without_cargo/README.md) — what `cargo new` set up, and what `rustc` does without it
- [Compile times](../compile_times/README.md) — why an unused dependency is not free
