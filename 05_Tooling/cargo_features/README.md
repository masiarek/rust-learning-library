# Cargo features

**Level:** 201 · working knowledge

**One line:** A feature is a named switch in `Cargo.toml` that turns on optional code and optional dependencies — and features are *unified* across a build, so one crate enabling a feature turns it on for everyone who depends on that crate.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `[features]`, `default`, and `#[cfg(feature = "name")]` in the code — the two halves that have to agree
- Optional dependencies and the `dep:` prefix: why it exists and what it stopped exposing
- `default-features = false` on a dependency, and why it is fragile: any other crate in the graph can turn the defaults back on
- Unification: the feature set Cargo builds is the union across the graph — which is why features must be **additive**
- What resolver version 2 changed: features of build-dependencies and dev-dependencies no longer leak into normal builds (verify the exact rule in the Cargo reference)
- Checking every combination: `cargo hack --feature-powerset`, and the `unexpected_cfgs` lint that catches a misspelled feature name

## The trap it exists for

A feature that *removes* something (`no-std`, `disable-logging`). Because features are unified, a single dependent turning it on removes it for every other dependent in the same build. The Cargo reference's rule is that features are additive, and this is the failure that rule prevents.

## Where this sits

[Conditional compilation](../../27_Modules/conditional_compilation/README.md) is the `#[cfg]` machinery a feature is read through. This page is the manifest side, and [Workspaces](../workspaces/README.md) is where unification surprises most people.

## See also

- [Conditional compilation](../../27_Modules/conditional_compilation/README.md) — `#[cfg(feature = …)]`, `cfg!` and `cfg_attr`
- [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) — `cargo add --features` and `--no-default-features`
- [Workspaces](../../05_Tooling/workspaces/README.md) — one build, one feature union
- [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) — `#[cfg]` among the other attribute families
- [Semver hazards](../../39_API_Design/semver_hazards/README.md) — removing a feature is a breaking change

## If you are coming from another language

- **C and C++.** `#ifdef FEATURE` with `-DFEATURE` on the command line, except that Cargo knows the full list and unifies it for the whole dependency graph instead of per translation unit.
- **Python.** Extras (`pip install requests[socks]`) are the closest match: optional dependencies named in the package metadata. Python has no compile step, so an extra cannot remove code the way a `cfg` can.

## Po polsku

Funkcja (*feature*) w Cargo to nazwany przełącznik, który włącza opcjonalny kod i opcjonalne zależności. Cargo łączy (*unifies*) funkcje w całym grafie zależności, więc jedna skrzynia włączająca funkcję włącza ją wszystkim — i dlatego funkcja powinna tylko dodawać, nigdy odbierać.

**Szukaj po polsku:** funkcje Cargo · kompilacja warunkowa · `cargo features additive` · `cargo default-features false`
