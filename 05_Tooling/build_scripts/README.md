# Build scripts: `build.rs`

**Level:** 301 · deep dive

**One line:** `build.rs` is a Rust program Cargo compiles and runs before your crate, whose only channel back is the lines it prints — `cargo::rustc-link-lib=…`, `cargo::rerun-if-changed=…` — which is how C libraries get linked and code gets generated.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- When it runs, what it can read (`OUT_DIR`, `TARGET`, `CARGO_CFG_*`, `CARGO_FEATURE_*`), and why it must write only under `OUT_DIR`
- The instructions it prints, the `cargo::` prefix and the older `cargo:` form, and what happens to a line Cargo does not recognise — verify both against the pinned Cargo
- `rerun-if-changed`: without it, when does Cargo re-run the script — and the full rebuilds that follow
- Compiling C with the `cc` crate and linking a system library with `pkg-config`; generating bindings with [Generating bindings](../../31_C_and_Cpp/migrating_c_to_rust/generating_bindings/README.md)
- Generated Rust: `include!(concat!(env!("OUT_DIR"), "/generated.rs"))`
- The cost: a build script is compiled for the *host*, so cross-compiling runs it on a different machine than the program

## The trap it exists for

Writing generated files into `src/`. A build script that modifies the source tree triggers rebuilds, breaks `cargo publish`'s verify step, and makes a read-only checkout fail. `OUT_DIR` is the one place it is allowed to write.

## Where this sits

[A build system is not a compiler](../../20_Compilers/build_systems_are_not_compilers/README.md) explains Cargo's role. This page is the escape hatch for the one step Cargo cannot express. [Calling C](../../09_Advanced/calling_c/README.md) is the most common reason to write one.

## See also

- [A build system is not a compiler](../../20_Compilers/build_systems_are_not_compilers/README.md) — what Cargo decides, and what it delegates
- [Calling C](../../09_Advanced/calling_c/README.md) — the FFI that usually needs a build script
- [Generating bindings](../../31_C_and_Cpp/migrating_c_to_rust/generating_bindings/README.md) — generating the `extern` declarations
- [The linker](../../20_Compilers/the_linker/README.md) — what `rustc-link-lib` finally asks for
- [Makefiles](../../20_Compilers/makefiles/README.md) — the hand-written build graph this replaces
- [Conditional compilation](../../27_Modules/conditional_compilation/README.md) — `rustc-cfg` from a build script

## If you are coming from another language

- **C and C++.** A custom command in CMake or a rule in a Makefile does the same job, but inside the build graph rather than before it. The C library's [reading a real Makefile ↗](https://masiarek.github.io/c-learning-library/01_Building/reading_a_real_makefile/) shows the kind of step `build.rs` replaces.
- **Python.** A `setup.py` that compiles an extension, or a build backend hook, is the counterpart — also arbitrary code at install time, with the same supply-chain concern.
- **Go.** `go generate` is similar in purpose but run by hand and committed, where `build.rs` runs on every build and writes to a temporary directory.

## Po polsku

`build.rs` to program w Ruście, który Cargo kompiluje i uruchamia przed kompilacją skrzyni; z Cargo porozumiewa się wyłącznie liniami wypisanymi na standardowe wyjście, takimi jak `cargo::rustc-link-lib=…`. Pisać wolno tylko do `OUT_DIR` — skrypt modyfikujący `src/` psuje przebudowy i publikację.

**Szukaj po polsku:** skrypt budowania · generowanie kodu w Ruście · `rust build.rs OUT_DIR` · `cargo rerun-if-changed`
