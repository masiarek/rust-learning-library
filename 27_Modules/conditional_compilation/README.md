# Conditional compilation

**Level:** 201 · working knowledge

**One line:** `#[cfg(…)]` removes an item before type checking if its condition is false — so code for another OS, a disabled feature or a test build is never compiled at all — while `cfg!(…)` is an ordinary `bool` in code that is always compiled.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The conditions: `target_os`, `target_arch`, `target_pointer_width`, `unix`/`windows`, `feature = "x"`, `test`, `debug_assertions`, and `all`/`any`/`not`
- `#[cfg]` versus `cfg!`: the first deletes code (a branch for Windows need not compile on Linux), the second keeps both branches and must type-check both
- `#[cfg_attr(condition, attribute)]`: applying an attribute only sometimes, e.g. a derive behind a feature
- Custom `--cfg` flags, and the `unexpected_cfgs` lint that warns on an unknown name — what it prints, and how a crate declares its expected cfgs
- Why this library's answer keys avoid printing `cfg!(target_os = …)`: the output is a constant on each machine and differs between them (the CONTRIBUTING rule about machine-dependent examples)
- Testing more than one configuration: `cargo check --target`, `cargo hack`

## The trap it exists for

Misspelling a condition — `#[cfg(feature = "serd")]` or `#[cfg(target_os = "macosx")]` — and silently compiling nothing. Before `unexpected_cfgs` that was invisible, because a false `cfg` and a misspelled one produce the same empty result.

## Where this sits

[What an attribute is](../what_an_attribute_is/README.md) introduces `cfg` among five attribute families. [Cargo features](../../05_Tooling/cargo_features/README.md) is the manifest half. [Where a test goes](../../28_Testing/where_a_test_goes/README.md) is `#[cfg(test)]` in use.

## See also

- [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) — `cfg` among the attribute families
- [Cargo features](../../05_Tooling/cargo_features/README.md) — the `feature = …` conditions
- [Where a test goes](../../28_Testing/where_a_test_goes/README.md) — `#[cfg(test)] mod tests`
- [Targets and triples](../../20_Compilers/targets_and_triples/README.md) — where `target_os` and `target_arch` come from
- [Build scripts](../../05_Tooling/build_scripts/README.md) — setting a cfg from `build.rs`
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — why examples here never print a platform constant

## If you are coming from another language

- **C and C++.** `#ifdef _WIN32` is the direct ancestor. The difference is that `#[cfg]` works on parsed items rather than text, so an excluded item must still parse — and a misspelled name now gets a warning.
- **Go.** Build tags (`//go:build linux`) and `_linux.go` file suffixes do the same job per file rather than per item.
- **Python.** `if sys.platform == "win32":` is `cfg!` — both branches exist and are checked at run time; Python has no counterpart to deleting code before compilation.

## Po polsku

`#[cfg(…)]` usuwa element przed sprawdzaniem typów, jeśli warunek jest fałszywy — kod dla innego systemu czy wyłączonej funkcji w ogóle się nie kompiluje. `cfg!(…)` to zwykła wartość `bool` w kodzie, który kompiluje się zawsze, więc obie gałęzie muszą być poprawne. Literówka w warunku daje po cichu pusty wynik; dziś ostrzega przed tym lint `unexpected_cfgs`.

**Szukaj po polsku:** kompilacja warunkowa · atrybut cfg · `rust cfg vs cfg macro` · `rust unexpected_cfgs lint`
