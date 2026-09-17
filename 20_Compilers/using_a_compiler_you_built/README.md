# Using a compiler you built

**Level:** 201 · working knowledge

**One line:** `rustup toolchain link stage1 build/host/stage1` registers your build as a toolchain named `stage1`, so `rustc +stage1 main.rs` runs your change against a test program — and `./x test tests/ui` is how you show it did not break anything else.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `rustup toolchain link stage1 build/host/stage1`, and `rustc +stage1 -vV` reporting a version ending in `-dev` — the check that you are running your build and not a downloaded one
- The link points into the build directory, so every later `./x build` or `./x test` for that stage updates the `stage1` toolchain in place, with no second link
- No Cargo in that toolchain: rustup falls back to the `cargo` from an installed nightly, beta or stable toolchain, in that order — what does that mean for `cargo +stage1 build` on a project that uses unstable Cargo flags?
- A test program first: one scratch `.rs` compiled with `rustc +stage1` and with `rustc +stable`, the outputs compared — the loop from [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) with a different toolchain name
- `rustup override set stage1` for a whole project directory, and the rust-analyzer caveat the dev guide attaches to it (the proc-macro server component)
- UI tests: `./x test tests/ui` for the suite, a directory or a single file as a filter (`./x test tests/ui/const-generics`), `.stderr` snapshots next to each test, and `--bless` to regenerate them — after which the dev guide says to read them by hand
- `//~ ERROR` annotations in a UI test: the line a diagnostic has to land on, kept deliberately redundant with the `.stderr` file
- `./x test tidy` before pushing: the style and formatting check CI runs anyway

## The trap it exists for

Building, then running `rustc main.rs` inside a project with a `rust-toolchain.toml` — or with any rustup default — and concluding the change does nothing. The pinned or default toolchain answered; only `+stage1`, an override, or the path to the binary runs the compiler you built. `rustc +stage1 -vV` settles it in one line.

## Where this sits

[rustup](../../05_Tooling/rustup/README.md) explains toolchains and `+name`; [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) explains why a directory can silently choose a different compiler. [Building the compiler](../building_the_compiler/README.md) produces `build/host/stage1`; [A first contribution](../a_first_contribution/README.md) is what you do once the tests pass.

## See also

- [rustup: the `rustc` you run is not the compiler](../../05_Tooling/rustup/README.md) — toolchains, and the `+toolchain` shorthand
- [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) — the file that overrides your default
- [`rustup default nightly`](../../05_Tooling/nightly/README.md) — why linking beats changing the default
- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — bare `rustc` on one file, and its edition trap
- [What a compiler does before your program runs](../what_a_compiler_does/README.md) — the kind of change a test program can show
- [Printing the HIR](../printing_the_hir/README.md) — a `-Z` flag your own build accepts without `RUSTC_BOOTSTRAP`?
- [Building the compiler](../building_the_compiler/README.md) — the step before this one
- [rustc dev guide: creating a rustup toolchain ↗](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html#creating-a-rustup-toolchain) — `toolchain link` and the Cargo fallback
- [rustc dev guide: UI tests ↗](https://rustc-dev-guide.rust-lang.org/tests/ui.html) — snapshots, `--bless` and annotations
- [rustup book: toolchains ↗](https://rust-lang.github.io/rustup/concepts/toolchains.html) — custom toolchains among the others

## If you are coming from another language

- **Python.** In a CPython checkout, `./python` runs the interpreter you just built without installing it, and `make test` runs the regression suite — the counterparts of `+stage1` and `./x test`.
- **Go.** A Go toolchain built from source is used by putting its `bin` directory first on `PATH`. rustup's named toolchains replace that juggling with `+stage1` per command.

## Po polsku

Zbudowany kompilator rejestruje się w rustup poleceniem `rustup toolchain link stage1 build/host/stage1`, a potem uruchamia przez `rustc +stage1 plik.rs`; wersja zakończona na `-dev` w `rustc +stage1 -vV` potwierdza, że to twoja kompilacja. Testy interfejsu (*UI tests*) uruchamia `./x test tests/ui`, a `--bless` odświeża pliki `.stderr`, które potem trzeba przeczytać. Pułapka: w katalogu z `rust-toolchain.toml` samo `rustc` uruchamia przypięty kompilator, nie twój — i zmiana „nic nie robi”.

**Szukaj po polsku:** własny kompilator w rustup · testy UI kompilatora · `rustup toolchain link stage1` · `x test tests/ui bless` · `rustc dev guide ui tests`
