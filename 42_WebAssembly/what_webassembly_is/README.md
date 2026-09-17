# What WebAssembly is

**Level:** 201 · working knowledge

**One line:** A `.wasm` file is a module of bytecode for a stack machine that can reach nothing it was not given — every capability arrives as an import and every entry point leaves as an export — which is why the same file can run in a browser, on a server and inside another program.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The file: an eight-byte preamble — `\0asm`, then version 1 — followed by sections, among them types, imports, functions, memories, globals, exports and code. Node 20.20.2's `WebAssembly.validate` accepts those eight bytes alone as a complete, empty module, and rejects them with the version byte changed to 2.
- The sandbox as a property of the format rather than a policy: code reads and writes *linear memory* — raw bytes addressed by offsets, bounds-checked — and has no way to name an address in the host. A buffer overflow inside a module is still possible — so what can it corrupt, and what can it not?
- Imports and exports as the entire interface. What makes a Rust function an export, and what does a module that imports nothing — the `wasm32v1-none` idea — have left to do besides compute?
- The core specification's value types are number types (`i32`, `i64`, `f32`, `f64`), vector types for SIMD, and reference types: no strings, no structs. How does a Rust `&str` cross — a pointer and a length into linear memory — and who agrees on that convention?
- Not only for browsers: [Wasmtime ↗](https://wasmtime.dev/) runs `rustc hello.rs --target wasm32-wasip1` output from a terminal, and WASI is a set of standard host functions for exactly that — see [WebAssembly outside the browser](../wasm_outside_the_browser/README.md).
- How fast, measured rather than asserted: the host's engine compiles the bytecode to machine code, so what does a module cost at load time, and what is left of "near-native" in a loop that calls out to the host?
- What WebAssembly does not have by default: a garbage collector you must use, a standard library, threads. Which of those did later proposals add, and which does Rust need?

## The trap it exists for

Thinking of wasm as "Rust running inside JavaScript", and so expecting the browser's facilities to be there. A module has no console, no clock and no files until the host imports them — which is why `println!` compiled for `wasm32-unknown-unknown` prints nothing and reports nothing.

## Where this sits

[Compiling to `wasm32`](../compiling_to_wasm32/README.md) covers the targets and what `std` becomes on each; [Rust in the browser](../rust_in_the_browser/README.md) and [WebAssembly outside the browser](../wasm_outside_the_browser/README.md) cover the two kinds of host. This page covers only what a module is.

## See also

- [Compiling to `wasm32`](../compiling_to_wasm32/README.md) — the targets that produce a module
- [What a compiler does](../../20_Compilers/what_a_compiler_does/README.md) — the pipeline whose last stage wasm replaces
- [Compiled, interpreted, or something between](../../20_Compilers/compiled_or_interpreted/README.md) — where a bytecode that is compiled again on arrival sits
- [LLVM and its IR](../../20_Compilers/llvm_and_its_ir/README.md) — the back end that emits the module
- [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) — what the sandbox still contains when Rust's checks are off
- [WebAssembly on MDN ↗](https://developer.mozilla.org/en-US/docs/WebAssembly) and [the core specification ↗](https://webassembly.github.io/spec/core/)

## If you are coming from another language

- **Java.** The closest counterpart: a portable bytecode, validated before it runs, compiled again by the host. What differs is how little comes with it — no class library, no required garbage collector, no object model — so a Rust module brings its own allocator into linear memory, where a JVM program relies on the JVM's heap.
- **C.** Emscripten compiles C to wasm with a C library emulated on top of browser APIs, and Rust can use the same toolchain through `wasm32-unknown-emscripten`. The Rust-native targets do without that emulation layer, which is why so much of `std` behaves differently on them.
- **Python.** CPython itself can be compiled to wasm: the 1.98.0 rustc book names Pyodide, a Python runtime built with Emscripten. That is the whole interpreter shipped as a module, where a Rust module is the program alone.
- **Go.** The standard toolchain lists both `js/wasm` and `wasip1/wasm` among its ports (`go tool dist list`, Go 1.25.5). Go ships its runtime and garbage collector inside the module; a Rust module carries no runtime of that size.

## Po polsku

Plik `.wasm` to moduł kodu bajtowego dla maszyny stosowej, zamknięty w piaskownicy: operuje na pamięci liniowej (*linear memory*) i nie może sięgnąć po nic, czego gospodarz nie przekazał mu jako importu. Cały interfejs modułu to importy i eksporty, a typy wartości to liczby, wektory SIMD i referencje — nie ma łańcuchów znaków ani struktur, więc `&str` przechodzi jako wskaźnik i długość. Stąd najczęstsze nieporozumienie: WebAssembly to nie „Rust uruchomiony w JavaScripcie”, tylko format, który równie dobrze działa w przeglądarce, na serwerze i jako wtyczka.

**Szukaj po polsku:** czym jest WebAssembly · piaskownica WebAssembly · pamięć liniowa wasm · `webassembly imports exports` · `wasm module binary format` · `webassembly outside the browser`
