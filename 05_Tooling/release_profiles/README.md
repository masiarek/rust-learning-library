# Release profiles

**Level:** 201 · working knowledge

**One line:** `--release` is not one switch but a profile — `opt-level`, `debug`, `lto`, `codegen-units`, `panic`, `strip` — and the defaults are a compromise you can move toward a faster build, a faster binary or a smaller one.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What `dev` and `release` set by default, read from the Cargo reference rather than remembered
- `opt-level` 0–3, `"s"` and `"z"`; `lto = "thin"` versus `true`; `codegen-units = 1` — each one measured on one program for build time, run time and size
- `panic = "abort"`: a smaller binary and no unwinding — and what it takes away (`catch_unwind`, `Drop` on panic). Link [what a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md)
- `debug`, `split-debuginfo` and `strip`: why a release binary with symbols is useful and when to drop them
- Custom profiles (`[profile.bench-fast] inherits = "release"`) and per-package overrides (`[profile.dev.package."*"] opt-level = 3`) for a fast dev loop with optimised dependencies
- `overflow-checks`: on in `dev`, off in `release` — the setting behind [signed overflow](../../31_C_and_Cpp/signed_overflow/README.md) behaving differently in the two builds

## The trap it exists for

Benchmarking a `dev` build. Unoptimised Rust can be tens of times slower than `--release`, and every iterator chain looks expensive. The opposite mistake is shipping `overflow-checks = false` without realising that `dev` was catching an overflow the release build now wraps silently.

## Where this sits

[Compile times](../compile_times/README.md) is about making the *build* faster. This page is about what the build produces. [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) shows what `opt-level` changes in the machine code.

## See also

- [Compile times](../../05_Tooling/compile_times/README.md) — the other side of the trade
- [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) — what `opt-level` buys, in instructions
- [`black_box` is a hint](../../33_Time_and_Benchmarking/black_box_is_a_hint/README.md) — benchmarking an optimised build honestly
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — what `panic = "abort"` removes
- [Signed overflow](../../31_C_and_Cpp/signed_overflow/README.md) — `overflow-checks` and the C behaviour it replaces
- [Production template](../../34_Templates/production/README.md) — a profile set chosen for code that ships

## If you are coming from another language

- **C and C++.** `-O0`/`-O2`/`-Os`, `-flto`, `-g` and `strip` map almost one to one. The difference is that Cargo names the bundle, so a project cannot drift into a different combination per file. The C++ library's [the optimizer deletes your benchmark ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/the_optimizer_deletes_your_benchmark/) is the benchmarking half.
- **Go.** Go has no release profile: `go build` always optimises, and `-ldflags="-s -w"` is the nearest thing to `strip`.

## Po polsku

`--release` to nie jeden przełącznik, tylko profil: `opt-level`, `debug`, `lto`, `codegen-units`, `panic`, `strip` i `overflow-checks`. Najdroższa pomyłka to mierzenie wydajności na kompilacji `dev`; druga w kolejności to nieświadomość, że `overflow-checks` jest włączone tylko w `dev`, więc przepełnienie złapane podczas testów w wersji wydanej po cichu się zawinie.

**Szukaj po polsku:** profil kompilacji release · optymalizacja w Cargo · `cargo profile release lto codegen-units` · `rust smaller binary size`
