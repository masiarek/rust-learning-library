# Building and linking

**Level:** 201 → 301 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Rust links C through a convention, not a feature — a `-sys` crate whose [build script ↗](https://doc.rust-lang.org/cargo/reference/build-scripts.html) compiles or finds the C library and tells Cargo what to link — and in a codebase that already has Make or CMake, the harder question is which build system is in charge.

## What it has to cover

- **`build.rs`:** what it runs before, the instructions it prints to Cargo (`rustc-link-lib`, `rustc-link-search`, `rerun-if-changed`), and why it must not reach the network
- The [`cc` ↗](https://docs.rs/cc/latest/cc/) crate for compiling C sources from `build.rs`, and `pkg-config` for finding an installed library
- **The `-sys` pattern:** a crate named `foo-sys` that only links and declares, with a `links = "foo"` key so two versions cannot both link the same native library — and a separate `foo` crate for the safe API
- Static against dynamic linking, and the error each one gives when the library is missing
- **Rust inside an existing build:** a `staticlib` or `cdylib` crate linked by Make or CMake, and tools such as [Corrosion ↗](https://github.com/corrosion-rs/corrosion) for CMake — against Cargo driving the C build instead
- One compiler toolchain or two: target triples, and building both sides for a cross target

## The trap it exists for

A build that works on the author's machine because a library happens to be installed there. `build.rs` finds it by accident, CI or a colleague's machine does not have it, and the failure is a linker error pages away from the cause.

## See also

- [The C ABI](../the_c_abi/README.md) — what is being linked
- [Generating bindings](../generating_bindings/README.md) — the declarations a `-sys` crate usually generates rather than writes
- [Build systems are not compilers](../../../20_Compilers/build_systems_are_not_compilers/README.md) and [Makefiles](../../../20_Compilers/makefiles/README.md) — the layer Cargo shares with Make
- [Cargo dependencies](../../../05_Tooling/cargo_dependencies/README.md) — how a `-sys` crate enters the graph

## Po polsku

Rust łączy się z C przez konwencję, a nie osobną funkcję języka: crate `-sys` ma **skrypt budujący** (`build.rs`), który kompiluje albo odnajduje bibliotekę C i mówi Cargo, co zlinkować, a bezpieczne API trafia do osobnego crate’a. W istniejącym projekcie z Make lub CMake trudniejsze jest pytanie, który system budowania rządzi — Cargo budujący C, czy CMake linkujący bibliotekę statyczną Rusta.

**Szukaj po polsku:** linkowanie bibliotek C w Ruscie · skrypt build.rs · `rust sys crate pattern` · `rust cmake corrosion staticlib`
