# Workspaces

**Level:** 201 · working knowledge

**One line:** A workspace is several packages sharing one `Cargo.lock` and one `target/` directory — which buys consistent versions and shared build artifacts, and costs one feature set for everything built together.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `[workspace] members`, the root manifest that is not a package, and a virtual manifest versus a root package
- What is shared (lockfile, `target/`, `[profile]`) and what is not (each member's dependencies and features)
- Inheritance: `[workspace.package]`, `[workspace.dependencies]` with `dep.workspace = true`, and `[workspace.lints]` — check which Cargo release stabilised each
- `cargo build -p name`, `--workspace`, and `default-members`
- When splitting a crate into a workspace speeds the build up (parallel crates, smaller rebuilds) and when it does not
- The practice-project layout this library already uses — see [a tree of practice projects](../practice_workspace/README.md)

## The trap it exists for

Expecting `cargo build -p a` and `cargo build` to compile `a` identically. Building members together unifies their dependencies' features, so a shared dependency can be compiled twice with two feature sets depending on which command you ran.

## Where this sits

[A tree of practice projects](../practice_workspace/README.md) is one workspace put to use. This page is the mechanism in general. [Cargo features](../cargo_features/README.md) is the unification that makes members interact.

## See also

- [A tree of practice projects](../../05_Tooling/practice_workspace/README.md) — a workspace in daily use
- [Scaffolding a practice tree](../../05_Tooling/scaffolding/README.md) — the files a workspace root owns
- [Cargo features](../../05_Tooling/cargo_features/README.md) — one build, one feature union
- [`Cargo.lock`](../../05_Tooling/cargo_lock/README.md) — the single lockfile a workspace shares
- [Compile times](../../05_Tooling/compile_times/README.md) — where a split crate saves time
- [Packages and crates](../../27_Modules/packages_and_crates/README.md) — the unit a workspace member is

## If you are coming from another language

- **Python.** A `uv` workspace is a direct copy of the design: one lockfile, several `pyproject.toml` members. The Python library's [`pyproject.toml` ↗](https://masiarek.github.io/python-learning-library/02_Projects_and_Environments/pyproject_toml/) page has the single-package half.
- **Go.** `go.work` joins modules for local development, but each module keeps its own `go.sum`, so it is closer to a development convenience than to Cargo's shared lockfile.
- **C and C++.** A top-level `CMakeLists.txt` with `add_subdirectory` is the build-graph half; there is no shared lockfile half, because there is no package manager to lock.

## Po polsku

Obszar roboczy (*workspace*) to kilka pakietów z jednym `Cargo.lock` i jednym katalogiem `target/`. Zysk to spójne wersje i wspólne artefakty kompilacji; koszt to jeden zestaw funkcji (*features*) dla wszystkiego, co budujesz razem — więc `cargo build -p a` i `cargo build` potrafią skompilować tę samą zależność inaczej.

**Szukaj po polsku:** obszar roboczy Cargo · monorepo w Ruście · `cargo workspace dependencies inheritance` · `cargo workspace feature unification`
