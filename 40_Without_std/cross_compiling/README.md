# Cross-compiling

**Level:** 301 · deep dive

**One line:** `rustup target add` installs a standard library compiled for another machine — not a compiler and not a linker — so a first cross build tends to fail twice: once because the target's `core` is missing, and once at the linker, the one tool rustup may not have given you.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The first refusal, before any target is installed: `rustc --target thumbv7em-none-eabihf` gives ``error[E0463]: can't find crate for `core` `` with the note *"the `thumbv7em-none-eabihf` target may not be installed"* and a `rustup target add` hint (1.98.0).
- What [`rustup target add` ↗](https://doc.rust-lang.org/rustc/platform-support.html) puts under `lib/rustlib/<triple>/`: precompiled `core` and `alloc`, plus `std` only where the target has one. Which column of the platform-support table says whether it does?
- The linker, per target. 1.98.0's toolchain ships `rust-lld` (and `wasm-component-ld`) in its own `bin` directory, and the nightly target spec names `rust-lld` as the linker for `thumbv7em-none-eabihf` — while `aarch64-unknown-linux-gnu` names no linker and the `gnu-cc` flavour, meaning a C compiler driver that must know the target. What does that difference mean for a first build from a Mac?
- Setting the linker: [`-C linker=` ↗](https://doc.rust-lang.org/rustc/codegen-options/index.html#linker) for bare `rustc`, and [`[target.<triple>] linker` ↗](https://doc.rust-lang.org/cargo/reference/config.html#targettriplelinker) in `.cargo/config.toml` for Cargo — and [`cargo-zigbuild` ↗](https://github.com/rust-cross/cargo-zigbuild), which uses Zig as that cross linker.
- [`cross` ↗](https://github.com/cross-rs/cross): the same Cargo command run inside a container image that already holds the linker and C libraries. Its README requires Docker or Podman. What happens to a `build.rs` that compiles C, inside and outside the container?
- `gnu` against `musl` on Linux — which libc the binary expects to find at run time, as [Targets and triples](../../20_Compilers/targets_and_triples/README.md) sets out. Is a statically linked musl binary the simple answer to "copy it to the server", and what does it cost?
- Running the result: QEMU user mode for a Linux binary, QEMU system emulation or [`probe-rs` ↗](https://probe.rs/) for a board. The linker writes ELF, a flasher may want raw bytes or Intel HEX — `rust-objcopy` is in the same toolchain `bin` directory. Which format does which tool take?
- Support levels: `thumbv7em-none-eabihf` and the six `wasm32` targets on [Compiling to `wasm32`](../../42_WebAssembly/compiling_to_wasm32/README.md) are listed under *Tier 2 without host tools* in the 1.98.0 rustc book, which it describes as "guaranteed to build" with automated tests "not always run". What does that promise, and what does it leave out?

## The trap it exists for

Believing `rustup target add` makes a cross build work. It makes *compiling* work. Linking is a separate program that has to understand the other machine's object format and libraries, and when it does not, the error comes from that program — naming architectures and missing symbols — long after `rustc` finished successfully.

## Where this sits

[Targets and triples](../../20_Compilers/targets_and_triples/README.md) explains what the four fields of a triple decide, and [rustup](../../05_Tooling/rustup/README.md) explains toolchains, components and targets; this page is only the practical sequence of building, linking and running for a machine that is not the one you are sitting at.

## See also

- [Targets and triples](../../20_Compilers/targets_and_triples/README.md) — the four decisions in a name like `x86_64-unknown-linux-gnu`
- [rustup: the `rustc` you run is not the compiler](../../05_Tooling/rustup/README.md) — where `rustup target add` fits among components and toolchains
- [The linker](../../20_Compilers/the_linker/README.md) — the stage a cross build fails in
- [Compiling to `wasm32`](../../42_WebAssembly/compiling_to_wasm32/README.md) — the same moves for six WebAssembly targets
- [What runs before `main`](../before_main_runs/README.md) — the reset handler the linked image starts in
- [TOOLCHAIN.md](../../TOOLCHAIN.md) — the map of which version of what, and who decided
- [ELF ↗](https://masiarek.github.io/encodings-learning-library/16_Formats/elf/), [Intel Hex ↗](https://masiarek.github.io/encodings-learning-library/16_Formats/intel_hex/) and [raw binary ↗](https://masiarek.github.io/encodings-learning-library/16_Formats/raw_binary/) — the three shapes the image passes through on its way to the chip

## If you are coming from another language

- **C.** A C cross toolchain is a family of triple-prefixed tools — `aarch64-linux-gnu-gcc` and its binutils — plus a C library built for the target, so choosing a compiler means choosing all of them. Rust splits that apart: one `rustc` emits code for every supported target, and the parts that still have to match the other machine are the target's standard library and the linker, which is why those are the two steps that fail.
- **Go.** `GOOS=linux GOARCH=arm64 go build` on a Mac produces a statically linked aarch64 ELF with nothing extra installed (checked with Go 1.25.5), because `go env CGO_ENABLED` reports `0` when cross-compiling and Go's own linker does the rest. Rust's bare-metal targets feel similar thanks to `rust-lld`; a Rust target that links against a C library needs a linker that knows that library, which is where the two experiences part.
- **Java.** Bytecode is the cross-compilation answer: build once, and the JVM on the other machine is the part compiled for it. WebAssembly is Rust's closest equivalent — see [What WebAssembly is](../../42_WebAssembly/what_webassembly_is/README.md).

## Po polsku

**Kompilacja skrośna** (*cross-compiling*) w Ruscie zwykle psuje się dwa razy. Najpierw brakuje `core` dla docelowej platformy — błąd `E0463` z podpowiedzią `rustup target add`. Potem, już po udanej kompilacji, zawodzi linker, bo `rustup` instaluje bibliotekę standardową dla innej maszyny, ale nie zawsze linker, który ją rozumie: dla gołego ARM wystarcza dołączony `rust-lld`, dla Linuksa na ARM potrzebny jest kompilator C znający cel, `cargo-zigbuild` albo kontener z narzędziem `cross`.

**Szukaj po polsku:** kompilacja skrośna Rust · linker dla innej architektury · `rustup target add thumbv7em-none-eabihf` · `rust cross compile linker not found` · `cross-rs docker` · `cargo zigbuild`
