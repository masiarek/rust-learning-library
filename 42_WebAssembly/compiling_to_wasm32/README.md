# Compiling to `wasm32`

**Level:** 201 · working knowledge

**One line:** Rust 1.98.0 lists six `wasm32` targets at Tier 2, and choosing one is choosing how much of `std` works — all of it on `wasm32-wasip2`, stubs that compile and then fail on `wasm32-unknown-unknown`, and none at all on `wasm32v1-none`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The six, all *Tier 2 without host tools* in the 1.98.0 rustc book, with what `rustc --print cfg --target …` reports for each: [`wasm32-unknown-unknown` ↗](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html) (`target_os="unknown"`), [`wasm32-wasip1` ↗](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html) and [`wasm32-wasip2` ↗](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip2.html) (`target_os="wasi"`, `target_env` `p1` and `p2`), [`wasm32-wasip1-threads` ↗](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1-threads.html), [`wasm32v1-none` ↗](https://doc.rust-lang.org/rustc/platform-support/wasm32v1-none.html) (`target_os="none"`, `core` and `alloc` only), and [`wasm32-unknown-emscripten` ↗](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-emscripten.html) — the only one of the six whose cfg says `panic="unwind"`. `wasm32-wasip3` also exists, at Tier 3.
- What `std` does with no host. The rustc book for `wasm32-unknown-unknown` says `println!` does nothing, `std::fs` always returns errors and `std::thread::spawn` panics; in 1.98.0's source, `fs` on that target falls through to the `unsupported` module, whose error is `ErrorKind::Unsupported`, *"operation not supported on this platform"*. The probe the finished page has to run: does `std::fs::read("data.txt")` compile for that target without a warning, and what does it return when the module runs?
- `--crate-type cdylib` — [`crate-type = ["cdylib"]` ↗](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field) in Cargo — for a module that a host calls into. What does a `bin` crate produce instead on `wasm32-wasip1`, and who calls it?
- Linkers: the target spec names `rust-lld` with the `wasm-lld` flavour for `wasm32-unknown-unknown` and `wasm32-wasip1`, and `wasm-component-ld` for `wasm32-wasip2`, which the rustc book says *outputs a component as opposed to a core wasm module*. What is the difference a host can see?
- The heap: the rustc book names `dlmalloc` as `wasm32-unknown-unknown`'s default global allocator. How much of a small module is allocator, and what does swapping it change?
- Binary size, as a table the finished page measures: debug against release, `opt-level = "z"`, `lto`, `strip`, `codegen-units = 1`, and then [`wasm-opt` ↗](https://github.com/WebAssembly/binaryen) from Binaryen over the result. `panic = "abort"` is already the default on five of the six.
- Features: 1.98.0's cfg for `wasm32-unknown-unknown` enables `bulk-memory`, `multivalue`, `mutable-globals`, `nontrapping-fptoint`, `reference-types` and `sign-ext`. What happens when an older engine loads a module that uses one — the reason `wasm32v1-none` exists, which the rustc book says enables nothing past the WebAssembly Core 1.0 spec except importing and exporting mutable globals?
- Conditional code: the rustc book recommends `#[cfg(all(target_family = "wasm", target_os = "unknown"))]` and warns that no `cfg` can tell whether the code will run on the web.

## The trap it exists for

Picking `wasm32-unknown-unknown` for a program that reads files or prints, because it is the target every browser tutorial uses. It compiles cleanly. Then `println!` writes nowhere and `std::fs` returns errors at run time — the host-shaped half of `std` is present as stubs, so the compiler has nothing to refuse.

## Where this sits

[Targets and triples](../../20_Compilers/targets_and_triples/README.md) covers what a triple is and `rustup target add`; [`core`, `alloc` and `std`](../../40_Without_std/core_alloc_and_std/README.md) covers the layers these targets keep or drop. This page covers only the `wasm32` family and choosing among it.

## See also

- [Targets and triples](../../20_Compilers/targets_and_triples/README.md) — the `wasm32-unknown-unknown` bullet this page expands
- [rustup](../../05_Tooling/rustup/README.md) — `rustup target add wasm32-unknown-unknown` and what it installs
- [`core`, `alloc` and `std`](../../40_Without_std/core_alloc_and_std/README.md) — the three layers `wasm32v1-none` stops short of
- [Cross-compiling](../../40_Without_std/cross_compiling/README.md) — the same build-link-run sequence for a microcontroller
- [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) — the size settings are optimizer settings
- [Compile times](../../05_Tooling/compile_times/README.md) — what `lto` and `codegen-units = 1` cost on the other side
- [TOOLCHAIN.md](../../TOOLCHAIN.md) — where targets sit among toolchains and components

## If you are coming from another language

- **C.** The rustc book is blunt that `wasm32-unknown-unknown` *has no equivalent in C/C++* and points C users at Emscripten or wasi-sdk. That is the same split as in Rust: `wasm32-unknown-emscripten` when you want a POSIX-like C library emulated for you, `wasm32-wasip1` when the host speaks WASI.
- **Go.** `GOOS=js GOARCH=wasm` and `GOOS=wasip1 GOARCH=wasm` are Go's two ports (both listed by `go tool dist list`, Go 1.25.5): a JavaScript host and a WASI host. Rust has the same two, plus the host-less `wasm32-unknown-unknown` and `wasm32v1-none`, which that list has no counterpart for.
- **Python.** There is no "compile my script to wasm" step; Python reaches wasm by compiling the interpreter, as Pyodide does through Emscripten. The target question still exists, one level down, for whoever builds that interpreter.

## Po polsku

Rust 1.98.0 ma sześć celów `wasm32` na poziomie Tier 2 i różnią się one tym, ile z `std` naprawdę działa. Na `wasm32-unknown-unknown` kod się kompiluje, ale `println!` nic nie wypisuje, a `std::fs` zawsze zwraca błąd (`ErrorKind::Unsupported`) — kompilator nie ma czego odrzucić, bo funkcje istnieją jako zaślepki. `wasm32-wasip1` i `wasm32-wasip2` zakładają gospodarza WASI z plikami i zegarem, a `wasm32v1-none` nie ma `std` wcale, tylko `core` i `alloc`. Do modułu wywoływanego z zewnątrz służy typ skrzynki `cdylib`, a rozmiar pliku zmniejsza się ustawieniami profilu i narzędziem `wasm-opt`.

**Szukaj po polsku:** kompilacja Rusta do WebAssembly · rozmiar pliku wasm · `rust wasm32-unknown-unknown std fs` · `rust cdylib wasm` · `wasm-opt binaryen size` · `wasm32-wasip2 component`
