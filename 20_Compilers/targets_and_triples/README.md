# Targets and triples

**Level:** 201 · working knowledge

**One line:** `x86_64-unknown-linux-gnu` is not a label, it is four decisions — architecture, vendor, operating system, ABI — and every one of them changes what the compiler emits and what the result can link against.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The four fields, read left to right, with the ones that are usually `unknown` and why that is fine
- `rustup target list` / `rustup target add`, and what it actually installs: the standard library **compiled for that target**, not a compiler and not a linker
- Why the first cross-build still fails after that — the missing linker, and where `cargo-zigbuild` or `cross` get one
- Tier 1, 2 and 3: what the promise is at each level, and what "no CI" means for a tier 3 target you are considering
- `#[cfg(target_os)]`, `#[cfg(target_arch)]`, `#[cfg(target_endian)]` — conditional compilation as the *front end's* half of portability, decided before LLVM
- `gnu` vs `musl` on Linux, which is the ABI field doing visible work: one dynamic libc dependency versus a statically linkable one
- `wasm32-unknown-unknown`: a target with no operating system at all, and what disappears from `std` when there is nobody to ask

## The trap it exists for

A target triple looks like a naming convention and behaves like a contract. Two builds that differ only in the last field produce binaries that cannot link to the same libraries, and the error arrives from the linker, in the linker's vocabulary, long after the part of the build that knew what you meant.

## See also

- [The linker](../the_linker/README.md) — the stage where a target mismatch is discovered, and why the message names an architecture
- [LLVM and its IR](../llvm_and_its_ir/README.md) — one IR, many back ends: the reason a target is a flag rather than a different compiler
- [rustup](../../05_Tooling/rustup/README.md) — components, toolchains and targets, and which of the three a given command changes
