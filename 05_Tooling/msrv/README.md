# MSRV: the oldest compiler you promise to support

**Level:** 201 · working knowledge

**One line:** `rust-version = "1.80"` in `Cargo.toml` is a promise that your crate builds on that compiler — and keeping it means checking it, because a dependency's update can break it without a single line of yours changing.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What `rust-version` does today: the error Cargo prints on an older toolchain, and what it does not check (your code's use of newer APIs)
- The MSRV-aware resolver: whether Cargo on the pinned toolchain prefers dependency versions compatible with your `rust-version`, and which setting controls it — verify in the Cargo reference before writing it down
- Testing it: a CI job on the declared version, `cargo hack --rust-version`, and `cargo +1.80 check`
- Minimal versions: `-Z minimal-versions` / `-Z direct-minimal-versions` on nightly, and why `serde = "1"` can be a lie about the oldest `serde` you actually work with
- Changelogs and unreleased versions: what a changelog promises about MSRV bumps, and treating an MSRV increase as a minor change or a breaking one
- How this library pins its own toolchain instead — [pinning the toolchain](../pinning_the_toolchain/README.md) — and why an application and a library answer this differently

## The trap it exists for

Declaring `rust-version` once and never testing it. The field is checked against the compiler, not against the code, so a crate can claim 1.70 and call a method stabilised in 1.80 until someone on 1.70 tries it.

## Where this sits

[Pinning the toolchain](../pinning_the_toolchain/README.md) chooses the compiler *you* build with. MSRV is the oldest compiler your *users* may build with. [Publishing a crate](../publishing_a_crate/README.md) is where the promise becomes public.

## See also

- [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) — one compiler for your own builds
- [rustup](../../05_Tooling/rustup/README.md) — `cargo +1.80` and installing an older toolchain
- [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) — the range a version requirement really accepts
- [Publishing a crate](../../05_Tooling/publishing_a_crate/README.md) — where the promise goes public
- [Semver hazards](../../39_API_Design/semver_hazards/README.md) — is an MSRV bump a breaking change?

## If you are coming from another language

- **Python.** `requires-python = ">=3.10"` is the same field with the same weakness: it is metadata, and nothing checks your code against it unless CI runs the oldest interpreter.
- **Go.** The `go 1.22` line in `go.mod` is stricter since Go 1.21 — the toolchain refuses or downloads a newer one — which is the enforcement `rust-version` leaves to CI.
- **C++.** The closest idea is a minimum `-std=c++17` plus a compiler version matrix, kept by hand in CI.

## Po polsku

MSRV (*minimum supported Rust version*) to najstarszy kompilator, na którym obiecujesz, że skrzynia się zbuduje; zapisuje się go jako `rust-version` w `Cargo.toml`. To pole jest sprawdzane względem kompilatora, a nie kodu, więc obietnica jest coś warta dopiero wtedy, gdy CI faktycznie buduje projekt na tej wersji.

**Szukaj po polsku:** minimalna wspierana wersja Rusta · `rust-version cargo toml` · `msrv policy rust crate` · `cargo minimal-versions`
