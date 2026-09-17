# Cargo subcommands worth knowing

**Level:** 201 · working knowledge

**One line:** Any program named `cargo-something` on your `PATH` becomes `cargo something` — so most of Cargo's ecosystem is installed rather than built in, and a dozen of those programs answer questions `cargo` alone cannot.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The mechanism: how `cargo foo` finds `cargo-foo`, and `cargo --list`
- Which ones answer which question, one line each and run once on a real project: `cargo-expand` (what did the macros write?), `cargo-udeps` / `cargo-machete` (which dependency is unused?), `cargo-deny` / `cargo-audit` (licences and advisories), `cargo-bloat` / `cargo-llvm-lines` (what makes the binary big and the build slow?), `cargo-hack` (every feature combination), `cargo-semver-checks` (did I break my API?)
- What moved into Cargo itself: `cargo add` and `cargo remove`, formerly `cargo-edit`
- `cargo install` vs `cargo binstall`, `--locked`, and where the binaries go
- Which of these need nightly (`cargo-udeps` does, as of its last release — verify)
- Tools this library already covers: [bacon](../bacon/README.md), [cargo-nextest](../nextest/README.md), [`cargo vendor`](../vendoring_and_patch/README.md)

## The trap it exists for

`cargo install` without `--locked`. It ignores the tool's own `Cargo.lock` and resolves fresh dependency versions, so a tool that built yesterday can fail today. The [`Cargo.lock`](../cargo_lock/README.md) page shows the command and why.

## Where this sits

[Tooling](../README.md) has a page for each tool this library has measured. This page is the index of the ones it has not, sorted by the question each answers. *Rust for Rustaceans* ch. 13 "The Rust Ecosystem" lists many of the same tools.

## See also

- [`Cargo.lock`](../../05_Tooling/cargo_lock/README.md) — why `cargo install` wants `--locked`
- [bacon](../../05_Tooling/bacon/README.md) — a subcommand-shaped tool, measured
- [cargo-nextest](../../05_Tooling/nextest/README.md) — another, measured
- [Expanding a macro](../../38_Declarative_Macros/expanding_a_macro/README.md) — `cargo expand` in use
- [Cargo features](../../05_Tooling/cargo_features/README.md) — `cargo hack --feature-powerset`
- [TOOLCHAIN.md](../../TOOLCHAIN.md) — the toolchain pages by problem

## If you are coming from another language

- **Git.** The same plug-in rule: an executable named `git-foo` becomes `git foo`, which is where Cargo borrowed it from.
- **Python.** There is no single entry point, so the equivalents are separate commands: `pip-audit`, `deptry`, `vulture`. The `uv tool install` / `pipx` split mirrors `cargo install`.
- **Go.** `go vet` and `govulncheck` cover part of this list; the rest are separate binaries installed with `go install`.

## Po polsku

Każdy program o nazwie `cargo-coś` leżący na `PATH` staje się poleceniem `cargo coś` — dlatego większość ekosystemu Cargo się instaluje, a nie jest wbudowana. Instaluj z `--locked`, bo bez tego `cargo install` ignoruje `Cargo.lock` narzędzia i potrafi dziś nie zbudować tego, co wczoraj działało.

**Szukaj po polsku:** podpolecenia Cargo · narzędzia Cargo · `cargo install locked` · `useful cargo subcommands`
