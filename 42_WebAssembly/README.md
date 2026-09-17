# WebAssembly

**One line:** WebAssembly is a compilation target, not a web technology — a sandboxed bytecode module that can touch only what its host hands it — and Rust reaches it through a handful of `wasm32` target triples that differ in exactly one thing: what the host is assumed to provide.

This section follows the order of the Ultimate Rust WASM outline: what a module is, how Rust builds one, running it in a browser, running it without one, and shipping a page — a Bevy game included. It is not a front-end framework guide: [Which Rust UI, and whether you want one](../08_Interfaces/rust_ui_options/README.md) compares Leptos, Dioxus and Tauri. It is not the general story of Rust without an operating system either, which is [Without std](../40_Without_std/README.md) — though `wasm32v1-none` belongs to both. The game itself is [Games with Bevy](../43_Games/README.md). For a book, *Rust and WebAssembly* is on [Books](../10_Resources/books/README.md), marked there as dated in its tooling.

| Lesson | Level | What it covers |
|---|---|---|
| [What WebAssembly is](what_webassembly_is/README.md) | 201 | A module of bytecode with imports and exports, a sandbox by construction, and not only for browsers — Stub |
| [Compiling to `wasm32`](compiling_to_wasm32/README.md) | 201 | The six Tier 2 `wasm32` targets, `cdylib`, what `std` does with no host, and binary size — Stub |
| [Rust in the browser](rust_in_the_browser/README.md) | 301 | `wasm-bindgen`, `web-sys` and `js-sys`, and every string copied from UTF-8 to UTF-16 at the boundary — Stub |
| [WebAssembly outside the browser](wasm_outside_the_browser/README.md) | 301 | WASI preview 1 and 2, the component model, Wasmtime, and wasm as a plugin sandbox — Stub |
| [Shipping a wasm page](shipping_a_wasm_page/README.md) | 301 | Static hosting, `application/wasm` and streaming compilation, a size budget, and a Bevy game on the web — Stub |

**Every page here is a stub** — an outline with its boundaries and its trap written down, and no runnable example yet. [CONTRIBUTING.md](../CONTRIBUTING.md) says what graduating one takes.

## How these pages graduate

Every checked example in this library is built by bare `rustc` for the machine CI runs on, so a page here graduates by building with `rustc --target wasm32-wasip1` (or `wasm32-unknown-unknown`) inside a Docker demo that also holds the host — `wasmtime`, or Node for a module that needs JavaScript — with the output kept as a dated "Real runs" fence rather than an answer key.

## Where it goes next

[Games with Bevy](../43_Games/README.md) ends in the browser through [Shipping a wasm page](shipping_a_wasm_page/README.md). [Targets and triples](../20_Compilers/targets_and_triples/README.md) is the compiler's account of what a target is, and [Without std](../40_Without_std/README.md) is the same "no operating system" question on a microcontroller. The strings that cross into JavaScript are UTF-16 on the other side, which the encodings library covers in [UTF-16 and surrogates ↗](https://masiarek.github.io/encodings-learning-library/03_Encodings/utf16_and_surrogates/).

## Po polsku

**WebAssembly** (w skrócie wasm) to po polsku nadal „WebAssembly” — nikt tego nie tłumaczy — i warto od początku odkleić tę nazwę od przeglądarki. Moduł `.wasm` to kod bajtowy w piaskownicy (*sandbox*): nie ma dostępu do niczego, czego gospodarz (*host*) mu jawnie nie przekaże jako importu. Przeglądarka jest jednym gospodarzem, `wasmtime` na serwerze drugim, a aplikacja ładująca wtyczki trzecim.

Z tego wynika cała reszta rozdziału: cele `wasm32` różnią się tym, **co zakłada się o gospodarzu**. `wasm32-unknown-unknown` nie zakłada nic, więc `println!` nic nie wypisuje, a `std::fs` zawsze zwraca błąd; `wasm32-wasip1` i `wasm32-wasip2` zakładają interfejs WASI z plikami i zegarem; `wasm32v1-none` nie ma `std` w ogóle. Strony są na razie szkicami, a przykłady będą budowane w kontenerze z `wasmtime` albo Node.

**Szukaj po polsku:** WebAssembly w Ruscie · Rust w przeglądarce · `rust wasm32 targets` · `wasm-bindgen tutorial` · `wasmtime wasi rust` · `webassembly component model`
