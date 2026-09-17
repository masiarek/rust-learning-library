# Building the compiler

**Level:** 301 · deep dive

**One line:** `./x build` compiles rustc from your checkout with a downloaded beta compiler — stage 0 — to produce a stage 1 compiler, which then builds the standard library; the rustc dev guide recommends 30 GB of free disk and warns that a build takes more than half an hour on a moderately powerful laptop.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Getting the source: `git clone https://github.com/rust-lang/rust.git`, or a partial clone with `--filter='blob:none'`; the dev guide warns that `--depth 1` breaks `git bisect` and `git blame`, so a shallow clone is for reading, not contributing
- `./x setup`: it asks for a profile and writes `bootstrap.toml`. The file was `config.toml` until rust-lang/rust PR #137081 (merged March 2025, milestone 1.87.0), so older walkthroughs name a file current instructions do not — does bootstrap still read the old name?
- The stages, as the dev guide defines them: stage 0 is a downloaded beta rustc and its std; stage 1 is your source compiled by stage 0; stage 2 is your source compiled again by stage 1, and is the compiler rustup distributes. Why is `--stage 1` enough for most development?
- The everyday commands: `./x check` for a fast type check, `./x build library` for a stage 1 compiler with a standard library it can link programs against, `./x build --stage 2` when stage 1 is not enough
- Git submodules: which parts of the tree are submodules (`src/tools/cargo`, `src/llvm-project` among them), the dev guide's note that bootstrap updates them itself, and `download-ci-llvm`, which makes the LLVM checkout unnecessary
- Hardware, from the dev guide's prerequisites: 30 GB+ free disk, 8 GB+ RAM, 2+ cores (more helps), an internet connection for submodules and the beta compiler, around 100 GB once you build past stage 1, and `-j` or `-j1` when a build that takes over an hour is really swapping
- Why a build of rustc with a beta compiler needs unstable features at all, and how that connects to the `RUSTC_BOOTSTRAP` variable [Printing the HIR](../printing_the_hir/README.md) borrows for one command
- One measured build to set beside the guide's recommendations: disk used, wall-clock time, and the size of `build/host/stage1`

## The trap it exists for

Following a walkthrough written before the 2025 rename. It tells you to edit `config.toml`, which a current checkout calls `bootstrap.toml`; and if it favoured a quick download with `--depth 1`, the first `git blame` to find who wrote the code you are changing does not work.

## Where this sits

[What a compiler does](../what_a_compiler_does/README.md) and [LLVM](../llvm_and_its_ir/README.md) explain what rustc is; [rustup](../../05_Tooling/rustup/README.md) is how you normally get one without building it. This page ends when `build/host/stage1` exists; [Using a compiler you built](../using_a_compiler_you_built/README.md) starts there.

## See also

- [What a compiler does before your program runs](../what_a_compiler_does/README.md) — the program you are about to build
- [LLVM: the part of rustc that is not Rust](../llvm_and_its_ir/README.md) — the submodule `download-ci-llvm` lets you skip
- [Printing the HIR](../printing_the_hir/README.md) — `RUSTC_BOOTSTRAP`, from the user's side
- [rustup: the `rustc` you run is not the compiler](../../05_Tooling/rustup/README.md) — the toolchains your build will sit beside
- [`rustup default nightly`](../../05_Tooling/nightly/README.md) — the prebuilt version of what you are building
- [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) — why a project directory can ignore the compiler you built
- [Compile times](../../05_Tooling/compile_times/README.md) — the same phases, on a crate small enough to time
- [Using a compiler you built](../using_a_compiler_you_built/README.md) — the next step
- [rustc dev guide: how to build and run the compiler ↗](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html) — cloning, `./x setup`, `bootstrap.toml`, disk space
- [rustc dev guide: what bootstrapping does ↗](https://rustc-dev-guide.rust-lang.org/building/bootstrapping/what-bootstrapping-does.html) — the stages, with a diagram
- [rustc dev guide: prerequisites ↗](https://rustc-dev-guide.rust-lang.org/building/prerequisites.html) — hardware and dependencies

## If you are coming from another language

- **C.** GCC's `make bootstrap` builds the compiler in three stages and compares the stage 2 and stage 3 object files, checking that the compiler reproduces itself — the same idea as rustc's stages, taken one step further.
- **Go.** Building Go from source needs an existing Go toolchain, named by `GOROOT_BOOTSTRAP` — Rust's stage 0, which bootstrap downloads for you instead.
- **Python.** CPython's interpreter is written in C, so there is no Python stage 0 in the chain that produces `python`: a C compiler starts the build, where rustc needs an earlier rustc.

## Po polsku

Kompilator Rusta jest napisany w Ruście, więc budowanie go to **bootstrapping** w trzech etapach: etap 0 to pobrany kompilator beta, etap 1 to twój kod skompilowany etapem 0, a etap 2 to ten sam kod skompilowany etapem 1 — i właśnie on trafia do rustup. Do codziennej pracy wystarcza etap 1. Uwaga na stare poradniki: plik konfiguracyjny nazywa się dziś `bootstrap.toml`, nie `config.toml`, a płytki klon (`--depth 1`) psuje `git blame` i `git bisect`. Przewodnik zaleca co najmniej 30 GB wolnego miejsca i 8 GB RAM.

**Szukaj po polsku:** budowanie kompilatora Rusta · bootstrapping kompilatora · `rustc dev guide build` · `x.py build stage1` · `bootstrap.toml config.toml`
